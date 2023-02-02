use super::*;

const INPUT: &str = "\
1. Just:
2. some test text with s-t-u-f-f to
3. match playable t-u-f-f ball
4. not so controversial
5. final line";

#[test]
fn no_match() {
    let search = "it's just not there";

    // NOTE: How to create an empty vector of a specific type
    assert_eq!(vec![""; 0], search_for(search, INPUT));
}

#[test]
fn single_line() {
    let search = "trove";

    assert_eq!(vec!["4. not so controversial"], search_for(search, INPUT));
}

#[test]
fn two_lines() {
    let search = "t-u-f";

    assert_eq!(
        vec![
            "2. some test text with s-t-u-f-f to",
            "3. match playable t-u-f-f ball"
        ],
        search_for(search, INPUT)
    );
}
