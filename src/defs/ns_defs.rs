#[derive(Eq, PartialEq)]
pub struct NS {
    pub name: &'static str,
    pub path: &'static str,
}

pub const HTML5: NS = NS {
    name: "HTML5",
    path: "crate::html5",
};

pub const SVG: NS = NS {
    name: "svg",
    path: "crate::svg",
};
