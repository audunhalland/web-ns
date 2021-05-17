#[derive(Eq, PartialEq)]
pub struct NS {
    pub name: &'static str,
    pub path: &'static str,
}

pub const HTML5: NS = NS {
    name: "HTML5_NS",
    path: "crate::html5",
};

pub const SVG: NS = NS {
    name: "SVG_NS",
    path: "crate::svg",
};
