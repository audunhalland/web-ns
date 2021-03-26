#![forbid(unsafe_code)]

pub mod attr;

mod static_unicase;

pub mod schema {
    pub mod html5;
}

///
/// A specific document Schema used on the web.
///
pub enum Schema {
    Html5,
}

// FIXME: Proper error impl
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidName,
    Dang,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
