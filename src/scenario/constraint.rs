use crate::scenario::code::Code;

#[derive(Clone, Copy, Debug)]
pub struct Constraint<'a> {
    pub(crate) card_num: u8,
    pub(crate) id: u8,
    pub(crate) name: &'a str,
    pub(crate) verifier: fn(code: &Code) -> bool,
}

impl<'a> std::fmt::Display for Constraint<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}.{}) {}", self.card_num, self.id, self.name)
    }
}

#[test]
fn constraint_display() {
    assert_eq!(
        Constraint {
            card_num: 1,
            id: 3,
            name: "foo",
            verifier: |_| true
        }
        .to_string(),
        "(1.3) foo"
    )
}
