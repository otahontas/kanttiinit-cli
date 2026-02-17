---
id: kc-q1sv
status: closed
deps: []
links: []
created: 2026-02-13T07:22:39Z
type: refactor
priority: 2
assignee: Otto Ahoniemi
---

# Deduplicate handle_query branches

The if args.hide_no_menu / else branches in handle_query share identical format_restaurants_with_menus + print_menus tails. Only the filtering-before-limiting step differs. Restructure so the shared tail appears once.

## Acceptance Criteria

- format_restaurants_with_menus and print_menus are each called once
- Behavior is identical to current code
- Existing tests pass
