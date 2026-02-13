---
id: kc-a2mf
status: open
deps: []
links: []
created: 2026-02-13T12:06:59Z
type: chore
priority: 2
assignee: Otto Ahoniemi
---

# Fix dependency PRs not auto-updating when behind base branch

Dependency update PRs (e.g., Dependabot/Renovate) are not automatically rebased or updated when they fall behind the base branch. This causes them to go stale and not merge. Goal: all dependencies should be kept up to date automatically.
