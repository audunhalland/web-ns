extern crate phf;

pub mod attr;

mod phf_util;
mod unicase_util;

mod schema {
    pub mod html;
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
    Dang,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
