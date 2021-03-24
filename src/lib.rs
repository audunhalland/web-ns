
pub mod attribute;

mod attr_type;
mod macros;
mod internal;

mod schema {
    pub mod html;
}

pub enum Schema {
    Html5
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
