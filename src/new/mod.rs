use dyn_symbol::Symbol;

#[derive(Debug, Eq, PartialEq)]
pub struct Element(pub(crate) Symbol);
#[derive(Debug, Eq, PartialEq)]
pub struct Attribute(pub(crate) Symbol);
