use crate::scenario::constraint::Constraint;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Guess<'a> {
    pub constraint: &'a Constraint,
    pub verified: bool,
}

impl<'a> std::fmt::Display for Guess<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if self.verified { "" } else { "!" },
            self.constraint
        )
    }
}

impl<'a> std::fmt::Debug for Guess<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
