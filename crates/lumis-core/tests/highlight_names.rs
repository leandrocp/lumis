//! `mdex_native` resolves the scope names Lumis sends it with a binary search
//! over its own copy of this table, so the table has to stay sorted. The
//! generator sorts it; this guards the file against a hand edit or a merge
//! resolution that does not.

use lumis_core::highlights::HIGHLIGHT_NAMES;

#[test]
fn highlight_names_are_sorted_and_unique() {
    let disorder = HIGHLIGHT_NAMES
        .windows(2)
        .find(|pair| pair[0] >= pair[1])
        .map(|pair| format!("{:?} is not before {:?}", pair[0], pair[1]));

    assert_eq!(disorder, None, "HIGHLIGHT_NAMES must be strictly ascending");
}

#[test]
fn highlight_names_have_no_empty_entry() {
    assert!(HIGHLIGHT_NAMES.iter().all(|name| !name.is_empty()));
}
