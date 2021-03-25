// build.rs

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use unicase::UniCase;

#[path = "src/attr/attr_type.rs"]
mod attr_type;

#[path = "src/defs/html_defs.rs"]
mod html_defs;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    gen_attrs(
        html_defs::attrs::DEFS,
        Path::new(&out_dir).join("codegen_html_attrs.rs"),
    );
}

fn gen_attrs(defs: &[(&'static str, &'static str, u32)], out_path: std::path::PathBuf) {
    let mut file = BufWriter::new(File::create(&out_path).unwrap());

    writeln!(&mut file, "use unicase::UniCase;").unwrap();
    writeln!(&mut file, "use crate::attr::attr_impl::*;").unwrap();
    writeln!(&mut file, "use crate::attr::attr_type::*;").unwrap();
    writeln!(&mut file, "use crate::phf_util::*;").unwrap();
    writeln!(&mut file).unwrap();

    {
        writeln!(&mut file, "static attrs: &[InternalAttr] = &[").unwrap();

        for (attr, prop, flags) in defs {
            writeln!(
                &mut file,
                r#"    InternalAttr {{ attribute: "{}", property: "{}", attr_type: AttrType({}) }},"#,
                attr, prop, flags
            )
            .unwrap();
        }

        writeln!(&mut file, "];").unwrap();
    }

    {
        let mut map_codegen: phf_codegen::Map<UniCase<&'static str>> = phf_codegen::Map::new();
        for (i, (attr, _, _)) in defs.iter().enumerate() {
            map_codegen.entry(UniCase::new(attr), &format!("{}", i));
        }

        writeln!(
            &mut file,
            "static ATTRIBUTE_UNICASE_PHF: phf::Map<UniCase<&'static str>, usize> = \n{};\n",
            map_codegen.build()
        )
        .unwrap();
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
        )
        .unwrap();
    }
}
