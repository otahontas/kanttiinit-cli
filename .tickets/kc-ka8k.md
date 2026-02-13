---
id: kc-ka8k
status: done
deps: []
links: []
created: 2026-02-13T07:22:35Z
type: refactor
priority: 2
assignee: Otto Ahoniemi
---

# Extract opening hours display logic from print_menus

print_menus in output.rs is ~70 lines with 4 levels of nesting. The match arm that formats opening hours (open/closed/before-opening/unparseable) is an independent concern. Extract it into a function like format_opening_hours_line(name, hours, now) that returns the formatted line.

## Acceptance Criteria

- print_menus reads as a flat loop over restaurants
- The extracted function is unit-testable with controlled time inputs
- No behavior changes in CLI output
