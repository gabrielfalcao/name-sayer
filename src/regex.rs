use crate::Result;
use regex::Regex;

pub fn regex() -> Regex {
    Regex::new(
        r"^(?<glob>[*]?)(?<root>[.])?(?<names>(?<name>[a-zA-Z0-9_-]+[.])+)(?<last_name>[a-zA-Z0-9_-]+)(?<tail>[.*]*)$",
    )
        .unwrap()
}

#[test]
fn regex_matching_extra_globs() -> Result<()> {
    let regex = regex();
    let input = "*research*.*";
    assert!(regex.is_match(input));
}
