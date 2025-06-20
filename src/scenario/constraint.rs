use crate::scenario::code::Code;

#[derive(Clone, Copy, Debug)]
pub struct Constraint<'a> {
    pub(crate) name: &'a str,
    pub(crate) verifier: fn(code: &Code) -> bool,
}

impl<'a> std::fmt::Display for Constraint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[test]
fn constraint_display() {
    assert_eq!(
        Constraint {
            name: "foo",
            verifier: |_| true
        }
        .to_string(),
        "foo"
    )
}
