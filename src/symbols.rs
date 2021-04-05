pub mod tag {
    include!(concat!(env!("OUT_DIR"), "/codegen_tag_symbols.rs"));
}
pub mod attr {
    include!(concat!(env!("OUT_DIR"), "/codegen_attr_symbols.rs"));
}
