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

    writeln!(&mut file, "use crate::attr::attr_impl::*;")?;
    writeln!(&mut file, "use crate::attr::attr_type::*;")?;
    writeln!(&mut file, "use crate::static_unicase::*;")?;
    writeln!(&mut file)?;

    {
        writeln!(&mut file, "static INTERNAL_ATTRS: &[InternalAttr] = &[")?;

        for (attr, prop, flags) in defs {
            writeln!(
                &mut file,
                r#"    InternalAttr {{ attribute: "{}", property: "{}", attr_type: AttrType({}) }},"#,
                attr, prop, flags
            )?;
        }

        writeln!(&mut file, "];")?;
    }

    {
        let mut map_codegen: phf_codegen::Map<StaticUniCase> = phf_codegen::Map::new();
        for (i, (attr, _, _)) in defs.iter().enumerate() {
            map_codegen.entry(StaticUniCase::new(attr), &format!("{}", i));
        }

        writeln!(
            &mut file,
            "static ATTRIBUTE_UNICASE_PHF: phf::Map<StaticUniCase, usize> = \n{};\n",
            map_codegen.build()
        )?;
    }

    {
        let mut map_codegen: phf_codegen::Map<&'static str> = phf_codegen::Map::new();
        for (i, (_, prop, _)) in defs.iter().enumerate() {
            map_codegen.entry(prop, &format!("{}", i));
        }

        writeln!(
            &mut file,
            "static PROPERTY_PHF: phf::Map<&'static str, usize> = \n{};\n",
            map_codegen.build()
        )?;
    }

    Ok(())
}
