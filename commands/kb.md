# Knowledge Base Expert Skill

Query and explore the CIM Neo4j knowledge graph containing concepts, projects, conversations, and their semantic relationships based on Conceptual Spaces theory.

## Connection Details

- **Neo4j Browser**: http://10.0.224.150:7474
- **Database**: `cim`
- **Access**: Via SSH through Proxmox (10.0.0.200), Container 150

## Quick Query Function

```bash
SSH_KEY="$HOME/.ssh/id_cim_thecowboyai"
HOST="root@10.0.0.200"

run_cypher() {
    local cypher="$1"
    ssh -i "$SSH_KEY" "$HOST" "pct exec 150 -- /run/current-system/sw/bin/curl -s -X POST 'http://localhost:7474/db/cim/tx/commit' -H 'Content-Type: application/json' -d '{\"statements\":[{\"statement\":\"$cypher\"}]}'" 2>/dev/null
}
```

## Common Queries

### Graph Overview
```cypher
MATCH (n) RETURN labels(n)[0] as type, count(*) as count ORDER BY count DESC
```

### Find Concept
```cypher
MATCH (c:Concept) WHERE c.name CONTAINS "keyword" RETURN c.name, c.category, c.description
```

### Similar Concepts
```cypher
MATCH (c:Concept {name: "ConceptName"})-[s:SIMILAR_TO]-(other)
RETURN other.name, s.similarity ORDER BY s.similarity DESC LIMIT 10
```

### Project Conversations
```cypher
MATCH (p:Project {name: "project-name"})<-[:ABOUT_PROJECT]-(conv)
RETURN conv.id, conv.summary, conv.messageCount ORDER BY conv.messageCount DESC
```

### Concept Hierarchy
```cypher
MATCH path = (child)-[:IS_A|PART_OF*1..3]->(parent:Concept {name: "ParentConcept"})
RETURN path
```

### Search by Quality Dimensions
```cypher
// Find async, distributed, stable concepts
MATCH (c:Concept)
WHERE c.temporal_orientation > 0.7 AND c.coupling_scope > 0.5 AND c.stability > 0.8
RETURN c.name, c.category
```

### Conversation Topics
```cypher
MATCH (conv:Conversation)-[:DISCUSSES]->(c:Concept)
WITH c.name as concept, count(conv) as discussions
ORDER BY discussions DESC LIMIT 15
RETURN concept, discussions
```

## Graph Schema

**Nodes**: Concept (71), Project (74), Conversation (662), Technology (8)

**Relationships**:
- SIMILAR_TO (414) - Computed from 6D quality space distance
- ABOUT_PROJECT (668) - Conversation → Project
- DISCUSSES (468) - Conversation → Concept
- IS_A, PART_OF, USES, IMPLEMENTS, REQUIRES, ENABLES

**Quality Dimensions** (0.0 → 1.0):
1. abstraction_level - concrete to abstract
2. coupling_scope - local to distributed
3. mutability - immutable to mutable
4. temporal_orientation - sync to async
5. human_involvement - automated to human-centric
6. stability - volatile to stable

## Example Requests

- "Find concepts similar to Event Sourcing"
- "What concepts does the cim-sage project discuss?"
- "Show me all async, immutable patterns"
- "How are SAGE and Calendar connected?"
- "What are the most discussed concepts?"
- "Find conversations about testing"

## User's Request
$ARGUMENTS
