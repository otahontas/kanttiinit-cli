# Rust code quality improvements

This file tracks improvements to make the codebase more idiomatic Rust. Each improvement includes full context so it can be implemented in isolation.

## Status legend

- `[ ]` - Not started
- `[x]` - Completed

---

## 1. Extract time parsing helper and eliminate unwraps in search.rs

- [x] Done

**Problem:** `search.rs:44-90` has 7 levels of nesting and multiple `.unwrap()` calls that can panic on malformed data.

**Files to modify:** `src/search.rs`

**Implementation:**

1. Add a helper function before `get_restaurants_by_query_filtered_by_closed_status_and_ordered_alphabetically`:

```rust
fn is_restaurant_open_now(opening_hours: &[Option<String>], weekday_index: u32) -> bool {
    let hours = match opening_hours.get(weekday_index as usize) {
        Some(Some(h)) => h,
        _ => return false,
    };
    let (start_str, end_str) = match hours.split_once('-') {
        Some((s, e)) => (s.trim(), e.trim()),
        None => return false,
    };
    let start_time = match chrono::NaiveTime::parse_from_str(start_str, "%H:%M") {
        Ok(t) => t,
        Err(_) => return false,
    };
    let end_time = match chrono::NaiveTime::parse_from_str(end_str, "%H:%M") {
        Ok(t) => t,
        Err(_) => return false,
    };
    let now = chrono::Local::now().time();
    start_time <= now && now <= end_time
}
```

2. Replace the filter closure (lines 44-90) with:

```rust
.filter(|restaurant| {
    if !hide_closed {
        return true;
    }
    is_restaurant_open_now(&restaurant.opening_hours, current_date_index_in_week)
})
```

3. Remove the now-unused `Local` import if it's only used in the filter (keep it if used elsewhere).

**Commit message:** `refactor(search): extract time parsing helper and remove unwraps`

---

## 2. Fix unwraps in output.rs time parsing

- [x] Done

**Problem:** `output.rs:35-39` uses `.unwrap()` on time parsing which can panic.

**Files to modify:** `src/output.rs`

**Implementation:**

Replace lines 31-60 (the time parsing and display logic inside `Some(todays_opening_hours)` arm) with safe parsing:

```rust
Some(todays_opening_hours) => {
    if day_offset != 0 {
        cprintln!("<bold>{}</> {}", restaurant.name, todays_opening_hours);
    } else {
        let current_time = Local::now().time();
        let end_time = todays_opening_hours
            .split_once('-')
            .and_then(|(_, end)| chrono::NaiveTime::parse_from_str(end.trim(), "%H:%M").ok());

        match end_time {
            Some(end_time) if current_time > end_time => {
                cprintln!(
                    "<strong>{}</> <dim>{}</>",
                    restaurant.name,
                    todays_opening_hours
                );
            }
            Some(end_time) => {
                let closes_in = end_time.signed_duration_since(current_time);
                let closes_in_formatted = format!(
                    "{}h {}m",
                    closes_in.num_hours(),
                    closes_in.num_minutes() % 60
                );
                cprintln!(
                    "<bold>{}</> <green>{}</> <dim>closes in {}</>",
                    restaurant.name,
                    todays_opening_hours,
                    closes_in_formatted
                );
            }
            None => {
                cprintln!("<bold>{}</> {}", restaurant.name, todays_opening_hours);
            }
        }
    }
}
```

**Commit message:** `fix(output): handle time parsing errors gracefully`

---

## 3. Use Option combinators instead of match in search.rs

- [x] Done

**Problem:** `search.rs:155-172` uses verbose match for Option handling.

**Files to modify:** `src/search.rs`

**Implementation:**

In `filter_menus_and_format_to_restaurants_with_menus`, replace lines 155-172:

```rust
let menu_items = match maybe_menu_items {
    Some(menu_items) => {
        let filter = match maybe_filter {
            Some(filter_string) => filter_string.clone(),
            None => "".to_string(),
        };
        // ... rest
    }
    None => None,
};
```

With:

```rust
let menu_items = maybe_menu_items.map(|menu_items| {
    let filter = maybe_filter.as_deref().unwrap_or("");
    menu_items
        .iter()
        .filter(|menu_item| menu_item.title.contains(filter))
        .map(|menu_item| FormattedMenuItem {
            title: menu_item.title.clone(),
            properties: menu_item.properties.join(", "),
        })
        .collect::<Vec<FormattedMenuItem>>()
});
```

**Commit message:** `refactor(search): use Option combinators for cleaner code`

---

## 4. Use Option combinators in commands.rs

- [x] Done

**Problem:** `commands.rs:10-22` uses verbose match for limit handling.

**Files to modify:** `src/commands.rs`

**Implementation:**

Replace the `limit_restaurants` function:

```rust
fn limit_restaurants(restaurants: &[Restaurant], maybe_limit: Option<u16>) -> &[Restaurant] {
    match maybe_limit {
        Some(limit) => {
            let limit_usize = usize::from(limit);
            if limit_usize < restaurants.len() {
                &restaurants[..limit_usize]
            } else {
                restaurants
            }
        }
        None => restaurants,
    }
}
```

With:

```rust
fn limit_restaurants(restaurants: &[Restaurant], limit: Option<u16>) -> &[Restaurant] {
    limit.map_or(restaurants, |n| {
        let n = usize::from(n);
        &restaurants[..n.min(restaurants.len())]
    })
}
```

**Commit message:** `refactor(commands): simplify limit_restaurants with map_or`

---

## 5. Implement FromStr trait for Lang

- [x] Done

**Problem:** `lang.rs` has a custom `from_str` method instead of implementing the standard `FromStr` trait.

**Files to modify:** `src/lang.rs`

**Implementation:**

1. Add import at top: `use std::str::FromStr;`

2. Replace the `impl Lang` block (lines 23-31):

```rust
impl Lang {
    fn from_str(s: &str) -> Result<Lang, anyhow::Error> {
        match s {
            "fi" => Ok(Lang::Fi),
            "en" => Ok(Lang::En),
            _ => Err(anyhow::anyhow!("Invalid language value: {}", s)),
        }
    }
}
```

With:

```rust
impl FromStr for Lang {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fi" => Ok(Lang::Fi),
            "en" => Ok(Lang::En),
            _ => Err(anyhow::anyhow!("Invalid language value: {}", s)),
        }
    }
}
```

3. Update usages in the same file - change `Lang::from_str(...)` to `s.parse::<Lang>()` or keep as-is since `FromStr` provides the same method signature.

**Commit message:** `refactor(lang): implement FromStr trait instead of custom method`

---

## 6. Use if/else instead of match on boolean

- [x] Done

**Problem:** `lang.rs:66-76` uses `match config_path.exists()` with `true`/`false` arms.

**Files to modify:** `src/lang.rs`

**Implementation:**

Replace:

```rust
match config_path.exists() {
    true => {
        let read_to_string =
            std::fs::read_to_string(config_path).context("Could not read config file")?;
        Ok(toml::from_str(&read_to_string).context("Could not parse config file as TOML")?)
    }
    false => Ok(Config {
        lang: "en".to_string(),
    }),
}
```

With:

```rust
if config_path.exists() {
    let contents = std::fs::read_to_string(config_path).context("Could not read config file")?;
    Ok(toml::from_str(&contents).context("Could not parse config file as TOML")?)
} else {
    Ok(Config {
        lang: "en".to_string(),
    })
}
```

**Commit message:** `refactor(lang): use if/else instead of match on boolean`

---

## 7. Remove unnecessary continue statement

- [x] Done

**Problem:** `output.rs:65` has an unnecessary `continue` as the last statement in a match arm.

**Files to modify:** `src/output.rs`

**Implementation:**

In the `print_menus` function, find:

```rust
None => {
    cprintln!("<bold>{}</>", restaurant.name);
    continue;
}
```

Remove the `continue;` line:

```rust
None => {
    cprintln!("<bold>{}</>", restaurant.name);
}
```

**Commit message:** `refactor(output): remove unnecessary continue statement`

---

## 8. Use split_once instead of split().collect()

- [x] Done

**Problem:** Code uses `split('-').collect::<Vec<&str>>()` followed by `.first()` and `.last()` when `split_once` is cleaner.

**Files to modify:** `src/search.rs` (if any remain after improvement #1)

**Implementation:**

Search for any remaining patterns like:

```rust
let parts = s.split('-').collect::<Vec<&str>>();
let start = parts.first();
let end = parts.last();
```

Replace with:

```rust
let (start, end) = s.split_once('-')?;  // or handle None appropriately
```

Note: After implementing improvement #1, this may already be addressed. Verify and mark as done if no changes needed.

**Commit message:** `refactor: use split_once for cleaner string splitting`

---

## Verification

After each improvement, run:

```bash
cargo build
cargo test
cargo clippy
```

All must pass before committing.
