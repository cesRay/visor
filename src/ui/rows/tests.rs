use super::expand_tabs;

#[test]
fn expands_tabs_and_shifts_ranges() {
    let (text, ranges) = expand_tabs("\tfoo\tbar", &[(1, 4), (5, 8)]);
    assert_eq!(text, "    foo    bar");
    assert_eq!(&text[ranges[0].0..ranges[0].1], "foo");
    assert_eq!(&text[ranges[1].0..ranges[1].1], "bar");
}

#[test]
fn text_without_tabs_is_unchanged() {
    assert_eq!(
        expand_tabs("abc", &[(0, 1)]),
        ("abc".to_string(), vec![(0, 1)])
    );
}
