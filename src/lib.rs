pub mod attr;

mod macros;

mod schema {
    pub mod html;
}

pub enum Schema {
    Html5,
}

// FIXME: Proper error impl
#[derive(Debug)]
pub enum Error {
    Dang,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
