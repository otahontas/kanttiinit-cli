---
id: kc-i9cx
status: done
deps: []
links: []
created: 2026-02-13T07:22:42Z
type: refactor
priority: 2
assignee: Otto Ahoniemi
---

# Simplify handle_arg control flow

handle_arg guards args.query.is_none() early, then later does if let Some(query) = &args.query. Replace with a single match or let-else to eliminate the redundant check.

## Acceptance Criteria

- query is unwrapped once
- No is_none() + later if let Some pattern
- Behavior unchanged
