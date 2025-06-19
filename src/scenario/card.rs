use crate::scenario::constraint::Constraint;

#[derive(Clone, Debug)]
pub(crate) struct Card<'a> {
    pub(crate) constraints: Vec<Constraint<'a>>,
}
