# Tickets for maintenance-ready state

## Ticket 1: Fix opening hours display bug

**Type:** bug

`format_restaurants_with_menus` in `search.rs` always calls `restaurant.opening_hours.first()`, which returns Monday's hours regardless of the selected day. The function needs the day offset (or computed weekday index) so it picks the correct day's hours.

**Acceptance criteria:**

- Opening hours shown match the queried day, not always Monday
- Existing tests updated, new tests cover day-offset scenarios

---

## Ticket 2: Widen restaurant ID type from u8

**Type:** bug

`Restaurant.id` is `u8` (max 255). Current API IDs already reach 74. If more restaurants are added, deserialization will fail silently or panic. Change to `u32`.

**Acceptance criteria:**

- `Restaurant.id` uses `u32`
- All references updated (test fixtures, menus hashmap keys, etc.)

---

## Ticket 3: Add MIT LICENSE file

**Type:** chore

`Cargo.toml` declares `license = "MIT"` but no LICENSE file exists in the repository. Add a standard MIT license file.

**Acceptance criteria:**

- `LICENSE` file exists at repo root with MIT text
- Copyright line matches `Cargo.toml` authors field

---

## Ticket 4: Return non-zero exit codes on errors

**Type:** bug

`handle_arg` prints errors to stderr but always exits with code 0. The process should exit with a non-zero code when:

- Conflicting flags are used (`-d` + `--hide-closed`, `-d` + `--hide-no-menu`)
- API or config errors occur
- No query is provided (without `--set-lang`)

**Acceptance criteria:**

- `main` exits with non-zero code on all error paths
- Success paths still exit with 0

---

## Ticket 5: Resolve or remove stale TODOs

**Type:** chore

Two TODOs remain in the code:

- `output.rs`: `// TODO: i18n based on lang selection` — the lang is already passed to the API but not to `print_menus`. Decide: either pass lang through and localize date formatting, or remove the TODO with a comment explaining the scope.
- `search.rs`: `// TODO: format opening_hours properly` — this is addressed by ticket 1. Remove after that fix lands.

**Acceptance criteria:**

- No TODO comments remain in `src/`
- If i18n is out of scope, a brief comment explains the decision

---

## Ticket 6: Update CHANGELOG.md

**Type:** docs

CHANGELOG is missing the 0.2.0 release entry. The `[Unreleased]` section still lists the geo removal, which shipped in 0.2.0. Version comparison links at the bottom are stale.

**Acceptance criteria:**

- 0.2.0 entry exists with its changes (geo removal, hide-no-menu feature, version bump)
- `[Unreleased]` section is empty or absent
- Comparison links updated

---

## Ticket 7: Fix README inaccuracies

**Type:** docs

Several parts of the README diverge from the actual CLI:

- Clone instructions say `cd cli` instead of the correct directory name
- "All options" section shows `-h, --hide-closed` but `-h` is `--help` in the actual CLI
- `--hide-no-menu` option is missing from the options list
- `-n` is documented as `--number` but the actual long form is `--head`

**Acceptance criteria:**

- Clone path matches actual repo name
- Options section matches `kanttiinit --help` output exactly
- All current flags are documented
