use kanttiinit::args::parse;
use std::ffi::OsString;

fn make_args(args: &[&str]) -> Vec<OsString> {
    args.iter().map(OsString::from).collect()
}

#[test]
fn test_args_parsing_with_query() {
    let args = make_args(&["kanttiinit", "-q", "otaniemi"]);
    let parsed = parse(args);
    assert_eq!(parsed.query, Some("otaniemi".to_string()));
}

#[test]
fn test_args_parsing_with_day_offset() {
    let args = make_args(&["kanttiinit", "-q", "töölö", "-d", "1"]);
    let parsed = parse(args);
    assert_eq!(parsed.day, 1);
}

#[test]
fn test_args_parsing_with_filter() {
    let args = make_args(&["kanttiinit", "-q", "otaniemi", "-f", "salad"]);
    let parsed = parse(args);
    assert_eq!(parsed.filter, Some("salad".to_string()));
}

#[test]
fn test_args_parsing_with_number() {
    let args = make_args(&["kanttiinit", "-q", "otaniemi", "-n", "5"]);
    let parsed = parse(args);
    assert_eq!(parsed.number, Some(5));
}

#[test]
fn test_args_parsing_with_flags() {
    let args = make_args(&["kanttiinit", "-q", "otaniemi", "-a", "-u", "-h"]);
    let parsed = parse(args);
    assert!(parsed.address);
    assert!(parsed.url);
    assert!(parsed.hide_closed);
}

#[test]
fn test_args_parsing_set_lang() {
    let args = make_args(&["kanttiinit", "--set-lang", "fi"]);
    let parsed = parse(args);
    assert_eq!(parsed.set_lang, Some("fi".to_string()));
}
