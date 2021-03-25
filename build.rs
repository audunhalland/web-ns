// build.rs

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[path = "src/static_unicase.rs"]
mod static_unicase;

#[path = "src/attr/attr_type.rs"]
mod attr_type;

#[path = "src/defs/html_defs.rs"]
mod html_defs;

use static_unicase::StaticUniCase;

fn main() {
    codegen().unwrap();
}

fn codegen() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();

    codegen_attrs(
        html_defs::attrs::DEFS,
        Path::new(&out_dir).join("codegen_html_attrs.rs"),
    )?;

    Ok(())
}

fn codegen_attrs(
    defs: &[(&'static str, &'static str, u32)],
    out_path: std::path::PathBuf,
) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(&out_path)?);

    struct Def {
        const_ident: String,
        attr: &'static str,
        prop: &'static str,
        flags: u32,
    }

    let defs: Vec<_> = defs
        .iter()
        .map(|(attr, prop, flags)| Def {
            const_ident: format!("INTERNAL_{}", attr.replace('-', "_").to_uppercase()),
            attr,
            prop,
            flags: *flags,
        })
        .collect();

    writeln!(&mut file, "use crate::attr::attr_impl::*;")?;
    writeln!(&mut file, "use crate::attr::attr_type::*;")?;
    writeln!(&mut file, "use crate::static_unicase::*;")?;
    writeln!(&mut file)?;

    {
        for def in defs.iter() {
            writeln!(
                &mut file,
                r#"const {const_ident}: InternalAttr = InternalAttr {{
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
