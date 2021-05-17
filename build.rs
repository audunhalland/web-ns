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

#[derive(Eq, PartialEq)]
struct NS {
    name: &'static str,
    path: &'static str,
}

const HTML5: NS = NS {
    name: "HTML5",
    path: "crate::html5",
};

#[derive(Clone)]
struct Def {
    ns: &'static NS,
    kind: DefKind,
}

impl Def {
    fn static_kind(&self) -> Option<&StaticDefKind> {
        match &self.kind {
            DefKind::Static(kind) => Some(kind),
            _ => None,
        }
    }

    fn entity_kind(&self) -> EntityKind {
        match &self.kind {
            DefKind::Static(kind) => kind.entity_kind.clone(),
            DefKind::DataAttr => EntityKind::Attribute,
        }
    }
}

#[derive(Clone)]
enum DefKind {
    Static(StaticDefKind),
    DataAttr,
}

#[derive(Clone)]
struct StaticDefKind {
    entity_kind: EntityKind,
    const_ident: String,
    variant_ident: String,
    local_name: &'static str,
    prop: &'static str,
    flags: u32,
    is_void: bool,
}

#[derive(Clone, Eq, PartialEq)]
enum EntityKind {
    Tag,
    Attribute,
}

impl EntityKind {
    fn name(&self) -> &str {
        match self {
            Self::Tag => "tag",
            Self::Attribute => "attribute",
        }
    }
}

fn codegen() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();

    let defs = defs();

    {
        let tags: Vec<_> = defs
            .iter()
            .filter(|def| def.ns == &HTML5 && def.entity_kind() == EntityKind::Tag)
            .cloned()
            .collect();

        let mut html_enum_file = BufWriter::new(File::create(
            &Path::new(&out_dir).join("codegen_tag_html_enums.rs"),
        )?);
        let f = &mut html_enum_file;

        let enum_ident = "HtmlTag";
        writeln!(f, "use crate::static_unicase::*;")?;
        enums::codegen_enum(enum_ident, EntityKind::Tag, &tags, f)?;
        enums::codegen_local_names(&tags, f)?;
        enums::codegen_local_name_lookup(enum_ident, &tags, f)?;
    }

    {
        let attrs: Vec<_> = defs
            .iter()
            .filter(|def| def.ns == &HTML5 && def.entity_kind() == EntityKind::Attribute)
            .cloned()
            .collect();

        let mut html_enum_file = BufWriter::new(File::create(
            &Path::new(&out_dir).join("codegen_attr_html_enums.rs"),
        )?);
        let f = &mut html_enum_file;

        let enum_ident = "HtmlAttr";
        writeln!(f, "use crate::static_unicase::*;")?;
        enums::codegen_enum(enum_ident, EntityKind::Attribute, &attrs, f)?;
        enums::codegen_local_names(&attrs, f)?;
        enums::codegen_properties(&attrs, f)?;
        enums::codegen_local_name_lookup(enum_ident, &attrs, f)?;
        enums::codegen_property_lookup(enum_ident, &attrs, f)?;
    }

    Ok(())
}

fn defs() -> Vec<Def> {
    let mut defs = vec![];

    for (tag, is_void) in html5_defs::tags::DEFS {
        defs.push(Def {
            ns: &HTML5,
            kind: DefKind::Static(StaticDefKind {
                entity_kind: EntityKind::Tag,
                const_ident: format!("{}", tag.replace('-', "_").to_uppercase()),
                variant_ident: make_enum_ident(tag),
                local_name: tag,
                prop: "",
                flags: 0,
                is_void: is_void.0,
            }),
        });
    }

    for (attr, prop, flags) in html5_defs::attrs::DEFS {
        defs.push(Def {
            ns: &HTML5,
            kind: DefKind::Static(StaticDefKind {
                entity_kind: EntityKind::Attribute,
                const_ident: format!("{}", attr.replace('-', "_").to_uppercase()),
                variant_ident: make_enum_ident(attr),
                local_name: attr,
                prop,
                flags: *flags,
                is_void: false,
            }),
        });
    }

    defs.push(Def {
        ns: &HTML5,
        kind: DefKind::DataAttr,
    });

    defs
}

fn make_enum_ident(input: &str) -> String {
    input
        .split('-')
        .map(|seg| {
            let mut chars: Vec<char> = seg.chars().collect();
            chars[0] = chars[0].to_ascii_uppercase();
            chars.into_iter().collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("")
}

// New style using enums
mod enums {
    use super::*;

    pub(crate) fn codegen_local_names<W: Write>(
        defs: &[Def],
        f: &mut BufWriter<W>,
    ) -> std::io::Result<()> {
        writeln!(f, "mod names {{")?;
        for def in defs.iter() {
            match &def.kind {
                DefKind::Static(static_kind) => {
                    writeln!(
                        f,
                        "    pub(crate) const {}: &str = \"{}\";",
                        static_kind.const_ident, static_kind.local_name
                    )?;
                }
                DefKind::DataAttr => {}
            }
        }
        writeln!(f, "}}")?;
        Ok(())
    }

    pub(crate) fn codegen_properties<W: Write>(
        defs: &[Def],
        f: &mut BufWriter<W>,
    ) -> std::io::Result<()> {
        writeln!(f, "mod properties {{")?;
        for def in defs.iter() {
            match &def.kind {
                DefKind::Static(static_kind) => {
                    writeln!(
                        f,
                        "    pub(crate) const {}: &str = \"{}\";",
                        static_kind.const_ident, static_kind.prop
                    )?;
                }
                DefKind::DataAttr => {}
            }
        }
        writeln!(f, "}}")?;
        Ok(())
    }

    pub(crate) fn codegen_enum<W: Write>(
        enum_ident: &str,
        entity_kind: EntityKind,
        defs: &[Def],
        f: &mut BufWriter<W>,
    ) -> std::io::Result<()> {
        writeln!(
            f,
            "#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]"
        )?;
        writeln!(f, "pub enum {} {{", enum_ident)?;
        for def in defs.iter() {
            match &def.kind {
                DefKind::Static(static_kind) => {
                    writeln!(
                        f,
                        "    /// The {} '{}' {}",
                        def.ns.name,
                        static_kind.local_name,
                        entity_kind.name()
                    )?;
                    writeln!(f, "    {ident},\n", ident = static_kind.variant_ident)?;
                }
                DefKind::DataAttr => {
                    writeln!(
                        f,
                        "    /// Some {} 'data-' {}",
                        def.ns.name,
                        entity_kind.name()
                    )?;
                    writeln!(f, "    Dataset(Box<crate::attr::data::DataAttr>),")?;
                }
            }
        }
        writeln!(f, "}}")?;

        // LocalName
        {
            writeln!(f, "impl crate::LocalName for {} {{", enum_ident)?;
            writeln!(f, "    fn local_name(&self) -> &str {{")?;
            writeln!(f, "        match self {{")?;
            for def in defs.iter() {
                match &def.kind {
                    DefKind::Static(static_kind) => {
                        writeln!(
                            f,
                            r#"            Self::{ident} => names::{const_ident},"#,
                            ident = static_kind.variant_ident,
                            const_ident = static_kind.const_ident,
                        )?;
                    }
                    DefKind::DataAttr => {
                        writeln!(f, "            Self::Dataset(data) => data.local_name(),")?;
                    }
                }
            }
            writeln!(f, "        }}")?;
            writeln!(f, "    }}")?;
            writeln!(f, "}}")?;
        }

        // Attribute
        if entity_kind == EntityKind::Attribute {
            writeln!(f, "impl crate::attr::Attribute for {} {{", enum_ident)?;
            writeln!(
                f,
                "    fn attr_type(&self) -> crate::attr::attr_type::AttrType {{"
            )?;
            writeln!(f, "        use crate::attr::attr_type::AttrType;")?;
            writeln!(f, "        match self {{")?;
            for def in defs.iter() {
                match &def.kind {
                    DefKind::Static(static_kind) => {
                        writeln!(
                            f,
                            r#"            Self::{ident} => AttrType({flags}),"#,
                            ident = static_kind.variant_ident,
                            flags = static_kind.flags
                        )?;
                    }
                    DefKind::DataAttr => {
                        writeln!(f, "            Self::Dataset(data) => data.attr_type(),")?;
                    }
                }
            }
            writeln!(f, "        }}")?;
            writeln!(f, "    }}")?;
            writeln!(f, "}}")?;
        }

        // PropertyName
        if entity_kind == EntityKind::Attribute {
            writeln!(f, "impl crate::PropertyName for {} {{", enum_ident)?;
            writeln!(f, "    fn property_name(&self) -> &str {{")?;
            writeln!(f, "        match self {{")?;
            for def in defs.iter() {
                match &def.kind {
                    DefKind::Static(static_kind) => {
                        writeln!(
                            f,
                            r#"            Self::{ident} => properties::{const_ident},"#,
                            ident = static_kind.variant_ident,
                            const_ident = static_kind.const_ident,
                        )?;
                    }
                    DefKind::DataAttr => {
                        writeln!(
                            f,
                            r#"            Self::Dataset(data) => data.property_name(),"#
                        )?;
                    }
                }
            }
            writeln!(f, "        }}")?;
            writeln!(f, "    }}")?;
            writeln!(f, "}}")?;
        }

        Ok(())
    }

    pub(crate) fn codegen_local_name_lookup<W: Write>(
        enum_ident: &str,
        defs: &[Def],
        f: &mut BufWriter<W>,
    ) -> std::io::Result<()> {
        let static_defs: Vec<_> = defs.iter().filter_map(Def::static_kind).collect();

        // Attribute name map:
        {
            let def_keys: Vec<_> = static_defs
                .iter()
                .map(|def| {
                    (
                        def,
                        PhfKeyRef {
                            key: StaticUniCase::new(def.local_name),
                            ref_expr: format!("StaticUniCase::new(names::{})", def.const_ident),
                        },
                    )
                })
                .collect();

            let mut map_codegen: phf_codegen::Map<PhfKeyRef<StaticUniCase>> =
                phf_codegen::Map::new();
            for (def, key) in def_keys {
                map_codegen.entry(key, &format!("{}::{}", enum_ident, def.variant_ident));
            }

            writeln!(
                f,
                "pub(crate) const STATIC_LOCAL_NAME_LOOKUP: phf::Map<StaticUniCase, {}> = {};",
                enum_ident,
                map_codegen.build()
            )?;
        }

        Ok(())
    }

    pub(crate) fn codegen_property_lookup<W: Write>(
        enum_ident: &str,
        defs: &[Def],
        f: &mut BufWriter<W>,
    ) -> std::io::Result<()> {
        let static_defs: Vec<_> = defs.iter().filter_map(Def::static_kind).collect();

        // Property name map:
        {
            let def_keys: Vec<_> = static_defs
                .iter()
                .map(|def| {
                    (
                        def,
                        PhfKeyRef {
                            key: def.prop,
                            ref_expr: format!("properties::{}", def.const_ident),
                        },
                    )
                })
                .collect();

            let mut map_codegen: phf_codegen::Map<PhfKeyRef<&str>> = phf_codegen::Map::new();
            for (def, key) in def_keys {
                map_codegen.entry(key, &format!("{}::{}", enum_ident, def.variant_ident));
            }

            writeln!(
                f,
                "pub(crate) const STATIC_PROPERTY_LOOKUP: phf::Map<&str, {}> = {};",
                enum_ident,
                map_codegen.build()
            )?;
        }

        Ok(())
    }
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
