---
id: kc-ly4l
status: done
deps: []
links: []
created: 2026-02-13T07:22:49Z
type: test
priority: 2
assignee: Otto Ahoniemi
---

# Deduplicate API fixture deserialization tests

test_deserialize_otaniemi_restaurants_from_api and test_deserialize_keskusta_restaurants_from_api are near-identical. Same for the two menu tests. Extract a helper that takes the JSON string and runs the assertions, then call it from each test with the appropriate fixture.

## Acceptance Criteria

- Shared assertion logic lives in a helper function
- Each test is a one- or two-liner calling the helper
- No assertions are lost
