---
id: kc-t78n
status: closed
deps: []
links: []
created: 2026-02-13T07:22:46Z
type: refactor
priority: 2
assignee: Otto Ahoniemi
---

# Simplify get_menus fold into filter_map

The .fold(HashMap::new(), ...) in get_menus inserts entries conditionally. A .filter_map + .collect() expresses the same intent more directly.

## Acceptance Criteria

- fold replaced with filter_map + collect
- Return value identical
- Existing deserialization tests pass
