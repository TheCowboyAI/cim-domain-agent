# Copyright (c) 2025-2026 - Cowboy AI, Inc.
#
# Decrypt ~/.claude secrets into the environment. SOURCE this, do not execute:
#
#     source ~/.claude/scripts/load-secrets.sh
#
# Add that line to your shell rc so ALICE_API_KEY is exported BEFORE Claude Code
# starts — .mcp.json refers to ${ALICE_API_KEY} and the harness expands it at
# launch, so a shell without it will start the alice MCP server unauthenticated.
#
# There is deliberately no cleartext copy on disk. secrets/claude-secrets.env.age
# is the only stored form and is safe to commit; the age identity at
# ~/.config/sops/age/keys.txt is the root of trust, exactly like an SSH key.

_claude_secrets_age="${HOME}/.claude/secrets/claude-secrets.env.age"
_claude_age_identity="${HOME}/.config/sops/age/keys.txt"

if [ ! -r "${_claude_secrets_age}" ]; then
  echo "load-secrets: missing ${_claude_secrets_age}" >&2
elif [ ! -r "${_claude_age_identity}" ]; then
  echo "load-secrets: missing age identity ${_claude_age_identity}" >&2
elif ! command -v age >/dev/null 2>&1; then
  echo "load-secrets: 'age' not on PATH" >&2
else
  # Decrypt straight into the shell. The plaintext exists only in this process;
  # it is never written to a file, so there is nothing to shred or leak.
  eval "$(age -d -i "${_claude_age_identity}" "${_claude_secrets_age}" 2>/dev/null)" \
    || echo "load-secrets: decryption failed (wrong identity?)" >&2
fi

unset _claude_secrets_age _claude_age_identity
