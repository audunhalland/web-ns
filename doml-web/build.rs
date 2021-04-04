// build.rs

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[path = "src/static_unicase.rs"]
mod static_unicase;

#[path = "src/attr/attr_type.rs"]
mod attr_type;

#[path = "src/defs/html5_defs.rs"]
mod html5_defs;

use static_unicase::StaticUniCase;

fn main() {
    codegen().unwrap();
}

fn codegen() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();

    codegen_attrs(
        html5_defs::attrs::DEFS,
        Path::new(&out_dir).join("codegen_html_attrs.rs"),
    )?;

    codegen_static_web_attrs(
        html5_defs::attrs::DEFS,
        NamespaceDesc {
            name: "HTML5",
            path: "crate::html5",
        },
        Path::new(&out_dir).join("codegen_static_html_attrs.rs"),
    )?;

    Ok(())
}

struct NamespaceDesc {
    name: &'static str,
    path: &'static str,
}

fn codegen_attrs(
    defs: &[(&'static str, &'static str, u32)],
    out_path: std::path::PathBuf,
) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(&out_path)?);

    struct Def {
        const_ident: String,
        pub_fn_ident: String,
        attr: &'static str,
        prop: &'static str,
        flags: u32,
    }

    let defs: Vec<_> = defs
        .iter()
        .map(|(attr, prop, flags)| Def {
            const_ident: format!("INTERNAL_{}", attr.replace('-', "_").to_uppercase()),
            pub_fn_ident: format!("r#{}", attr.replace('-', "_")),
            attr,
            prop,
            flags: *flags,
        })
        .collect();

    writeln!(&mut file, "use crate::attr::Attribute;")?;
    writeln!(&mut file, "use crate::attr::attr_impl::*;")?;
    writeln!(&mut file, "use crate::attr::attr_type::*;")?;
    writeln!(&mut file, "use crate::static_unicase::*;")?;
    writeln!(&mut file)?;

    // Public interface:
    {
        for def in defs.iter() {
            writeln!(
                &mut file,
                r#"
pub fn {pub_fn_ident}() -> Attribute {{
    AttrImpl::Internal(&{const_ident}).into()
}}"#,
                pub_fn_ident = def.pub_fn_ident,
                const_ident = def.const_ident
            )?;
        }
    }

    // Internal definitions:
    {
        for def in defs.iter() {
            writeln!(
                &mut file,
                r#"
const {const_ident}: InternalAttr = InternalAttr {{
    attribute: "{attr}",
    property: "{prop}",
    attr_type: AttrType({flags})
}};"#,
                const_ident = def.const_ident,
                attr = def.attr,
                prop = def.prop,
                flags = def.flags
            )?;
        }
    }

    // Attribute map:
    {
        let def_keys: Vec<_> = defs
            .iter()
            .map(|def| {
                (
                    def,
                    PhfKeyRef {
                        key: StaticUniCase::new(def.attr),
                        ref_expr: format!("StaticUniCase::new({}.attribute)", def.const_ident),
                    },
                )
            })
            .collect();

        let mut map_codegen: phf_codegen::Map<PhfKeyRef<StaticUniCase>> = phf_codegen::Map::new();
        for (def, key) in def_keys {
            map_codegen.entry(key, &format!("&{}", def.const_ident));
        }

        writeln!(
            &mut file,
            "static ATTRIBUTE_UNICASE_PHF: phf::Map<StaticUniCase, &'static InternalAttr> = \n{};\n",
            map_codegen.build()
        )?;
    }

    // Property map:
    {
        let def_keys: Vec<_> = defs
            .iter()
            .map(|def| {
                (
                    def,
                    PhfKeyRef {
                        key: def.prop,
                        ref_expr: format!("{}.property", def.const_ident),
                    },
                )
            })
            .collect();

        let mut map_codegen: phf_codegen::Map<PhfKeyRef<&'static str>> = phf_codegen::Map::new();
        for (def, key) in def_keys {
            map_codegen.entry(key, &format!("&{}", def.const_ident));
        }

        writeln!(
            &mut file,
            "static PROPERTY_PHF: phf::Map<&'static str, &'static InternalAttr> = \n{};\n",
            map_codegen.build()
        )?;
    }

    Ok(())
}

fn codegen_static_web_attrs(
    defs: &[(&'static str, &'static str, u32)],
    ns_desc: NamespaceDesc,
    out_path: std::path::PathBuf,
) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(&out_path)?);

    struct Def {
        static_id: usize,
        pub_const_ident: String,
        attr: &'static str,
        prop: &'static str,
        flags: u32,
    }

    let defs: Vec<_> = defs
        .iter()
        .enumerate()
        .map(|(static_id, (attr, prop, flags))| Def {
            static_id,
            pub_const_ident: format!("{}", attr.replace('-', "_").to_uppercase()),
            attr,
            prop,
            flags: *flags,
        })
        .collect();

    writeln!(
        &mut file,
        "use doml::attribute::{{Attribute, StaticAttribute}};"
    )?;
    writeln!(&mut file, "use crate::attr::StaticWebAttr;")?;
    writeln!(&mut file, "use crate::attr::attr_type::*;")?;
    writeln!(&mut file, "use crate::static_unicase::*;")?;
    writeln!(&mut file)?;

    // Public interface:
    {
        for def in defs.iter() {
            writeln!(
                &mut file,
                r#"
/// The {ns_name} `{attr}` attribute
pub const {pub_const_ident}: Attribute = Attribute::new_static(&WEB_ATTRS[{static_id}].static_attribute);"#,
                ns_name = ns_desc.name,
                attr = def.attr,
                pub_const_ident = def.pub_const_ident,
                static_id = def.static_id,
            )?;
        }

        writeln!(&mut file, "",)?;
    }

    // Attribute class:
    writeln!(
        &mut file,
        r#"
struct StaticAttrClass;

impl doml::attribute::AttributeClass for StaticAttrClass {{
    fn namespace(&self) -> &'static dyn doml::Namespace {{
        &super::HTML5_NS
    }}

    fn local_name(&self, static_id: Option<usize>) -> &str {{
        WEB_ATTRS[static_id.unwrap()].name
    }}

    fn metadata<'a>(&'a self, static_id: Option<usize>, key: &str) -> Option<&'a str> {{
        match key {{
            crate::metadata::PROPERTY => Some(WEB_ATTRS[static_id.unwrap()].property),
            _ => None,
        }}
    }}
}}

const STATIC_ATTR_CLASS: StaticAttrClass = StaticAttrClass;
    "#
    )?;

    // Attribute array:
    {
        writeln!(
            &mut file,
            "pub(crate) const WEB_ATTRS: [StaticWebAttr; {len}] = [",
            len = defs.len()
        )?;

        for def in defs.iter() {
            writeln!(
                &mut file,
                r#"    StaticWebAttr {{
        static_attribute: StaticAttribute {{
            class: &{ns_path}::attrs::STATIC_ATTR_CLASS,
            static_id: {static_id},
        }},
        name: "{attr}",
        property: "{prop}",
        attr_type: AttrType({flags})
    }},"#,
                ns_path = ns_desc.path,
                static_id = def.static_id,
                attr = def.attr,
                prop = def.prop,
                flags = def.flags
            )?;
        }

        writeln!(&mut file, "];\n",)?;
    }

    // Attribute name map:
    {
        let def_keys: Vec<_> = defs
            .iter()
            .map(|def| {
                (
                    def,
                    PhfKeyRef {
                        key: StaticUniCase::new(def.attr),
                        ref_expr: format!("StaticUniCase::new(WEB_ATTRS[{}].name)", def.static_id),
                    },
                )
            })
            .collect();

        let mut map_codegen: phf_codegen::Map<PhfKeyRef<StaticUniCase>> = phf_codegen::Map::new();
        for (def, key) in def_keys {
            map_codegen.entry(key, &format!("&WEB_ATTRS[{}]", def.static_id));
        }

        writeln!(
            &mut file,
            "pub(crate) static ATTRIBUTE_UNICASE_PHF: phf::Map<StaticUniCase, &'static StaticWebAttr> = \n{};\n",
            map_codegen.build()
        )?;
    }

    // Property map:
    {
        let def_keys: Vec<_> = defs
            .iter()
            .map(|def| {
                (
                    def,
                    PhfKeyRef {
                        key: def.prop,
                        ref_expr: format!("WEB_ATTRS[{}].property", def.static_id),
                    },
                )
            })
            .collect();

        let mut map_codegen: phf_codegen::Map<PhfKeyRef<&'static str>> = phf_codegen::Map::new();
        for (def, key) in def_keys {
            map_codegen.entry(key, &format!("&WEB_ATTRS[{}]", def.static_id));
        }

        writeln!(
            &mut file,
            "pub(crate) static PROPERTY_PHF: phf::Map<&'static str, &'static StaticWebAttr> = \n{};\n",
            map_codegen.build()
        )?;
    }

    Ok(())
}

struct PhfKeyRef<T> {
    key: T,
    ref_expr: String,
}

impl<T: PartialEq<T>> PartialEq<PhfKeyRef<T>> for PhfKeyRef<T> {
    fn eq(&self, rhs: &PhfKeyRef<T>) -> bool {
        self.key.eq(&rhs.key)
    }
}
impl<T: Eq> Eq for PhfKeyRef<T> {}

impl<T: std::hash::Hash> std::hash::Hash for PhfKeyRef<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<T: phf_shared::PhfHash> phf_shared::PhfHash for PhfKeyRef<T> {
    fn phf_hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.phf_hash(state);
    }
}

impl<T> phf_shared::FmtConst for PhfKeyRef<T> {
    fn fmt_const(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.ref_expr)
    }
}
