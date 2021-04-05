use dyn_symbol::Symbol;

#[derive(Debug, Eq, PartialEq)]
pub struct Element(pub(crate) Symbol);
#[derive(Debug, Eq, PartialEq)]
pub struct Attribute(pub(crate) Symbol);

impl Attribute {
    pub fn local_name(&self) -> &str {
        self.0.name()
    }
}
