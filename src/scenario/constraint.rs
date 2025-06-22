use crate::scenario::code::Code;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct ConstraintID {
    pub(crate) card: u8,
    pub(crate) idx: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Constraint {
    pub(crate) id: ConstraintID,
    pub(crate) name: &'static str,
    pub(crate) verifier: fn(code: &Code) -> bool,
}

impl std::hash::Hash for Constraint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}.{}) {}", self.id.card, self.id.idx, self.name)
    }
}

#[test]
fn constraint_display() {
    assert_eq!(
        Constraint {
            id: ConstraintID { card: 1, idx: 1 },
            name: "foo",
            verifier: |_| true
        }
        .to_string(),
        "(1.3) foo"
    )
}
