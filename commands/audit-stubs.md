# Audit Stubs

Scan CIM code for stub verifications and fraud tests. Stubs are violations of CIM axioms — they claim verification without performing it.

## Usage

`/audit-stubs [path]`

Where `[path]` is optional — defaults to current project `src/`.

## What It Detects

### Stub Verifications (fraud — claims to verify but doesn't)

```rust
// ❌ Returns true without checking
fn verify_functor_laws(&self) -> bool { true }
fn verify_naturality(&self) -> bool { !self.components.is_empty() }
fn verify_universal_property(&self) -> bool { true }
fn is_valid(&self) -> bool { true }
```

### Empty Tests (test that tests nothing)

```rust
// ❌ Always passes
#[test] fn test_something() { assert!(true); }
#[test] fn test_it() { }
#[test] fn test_create() { let _ = Thing::new(); }
```

### Ignored Results (creates but doesn't check)

```rust
// ❌ Result not inspected
let _ = aggregate.handle(cmd);
let _ = entity.lift();
```

### Skipped Assertions

```rust
// ❌ TODO assertions
#[test] fn test_law() { /* TODO: verify law */ }
```

## Detection Commands

```bash
# Stub verifications
grep -rn "-> bool.*{.*true.*}" src/ --include="*.rs"

# Empty tests — a test whose body asserts NOTHING.
#
# ⛔ Do NOT use `grep -A3 '#[test]' | grep -E 'assert!\(true\)|^\s*\}$'`.
# That was here and it is a MEASUREMENT ARTIFACT: `^\s*\}$` matches the closing
# brace of every correct test too, so it reports the same thing whether the code
# is fine or fraudulent — it carries no information. Verified: it returns a hit
# on a three-line test containing a real assert_eq!.
#
# Parse the body instead of pattern-matching near it:
python3 - <<'EOF'
import re, pathlib
pat = re.compile(r'#\[(?:tokio::)?test\][^\n]*\n\s*(?:async\s+)?fn\s+(\w+)\s*\([^)]*\)[^{]*\{', re.M)
for f in pathlib.Path('src').rglob('*.rs'):
    src = f.read_text(encoding='utf-8', errors='ignore')
    for m in pat.finditer(src):
        i, depth = m.end(), 1
        while i < len(src) and depth:
            depth += (src[i] == '{') - (src[i] == '}')
            i += 1
        body = src[m.end():i-1]
        if not re.search(r'\bassert\w*!|\bexpect\(|\bpanic!|\?;|\bmatches!', body):
            print(f'{f}:{src[:m.start()].count(chr(10))+1} — {m.group(1)}() asserts nothing')
        elif re.search(r'\bassert!\(\s*true\s*\)', body):
            print(f'{f}:{src[:m.start()].count(chr(10))+1} — {m.group(1)}() asserts assert!(true)')
EOF

# Ignored results in tests
grep -rn "let _ =" tests/ src/ --include="*.rs" | grep -v "// OK"

# TODO in tests
grep -rn "TODO\|FIXME\|HACK\|XXX" tests/ src/ --include="*.rs"
```

## Report Format

```
Stub Audit Report
═════════════════
Files scanned: {N}
Stubs found: {N}

VIOLATIONS:
  src/graphs/functor.rs:42 — verify_functor_laws() returns true unconditionally
  src/aggregate/test.rs:15 — test_create() has no assertions
  tests/integration.rs:88 — result ignored with let _ =

Each violation is a breach of:
  CT-5 (Kan extension) / CT-2 (Functor) / CIM-24 (a verifier that cannot fail is fraud)
```

## Instructions

1. Scan all .rs files in the path
2. Pattern-match for stub patterns listed above
3. Report each with file:line and which axiom it violates
4. Exit with error code if any stubs found
