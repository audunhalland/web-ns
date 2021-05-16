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

struct NamespaceDesc {
    name: &'static str,
    path: &'static str,
}

struct TagDef {
    web_ns: &'static str,
    static_id: usize,
    pub_const_ident: String,
    tag: &'static str,
    is_void: bool,
}

struct AttributeDef {
    web_ns: &'static str,
    kind: AttributeDefKind,
}

impl AttributeDef {
    fn static_kind(&self) -> Option<&StaticAttributeDefKind> {
        match &self.kind {
            AttributeDefKind::Static(kind) => Some(kind),
            _ => None,
        }
    }
}

enum AttributeDefKind {
    Static(StaticAttributeDefKind),
    Data,
}

struct StaticAttributeDefKind {
    static_id: usize,
    pub_const_ident: String,
    enum_ident: String,
    attr: &'static str,
    prop: &'static str,
    flags: u32,
}

fn codegen() -> std::io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();

    let tag_defs = tag_defs();
    let attribute_defs = attribute_defs();

    symbols::codegen_static_tag_symbols(
        &tag_defs,
        Path::new(&out_dir).join("codegen_tag_symbols.rs"),
    )?;

    symbols::codegen_static_attribute_symbols(
        &attribute_defs,
        Path::new(&out_dir).join("codegen_attr_symbols.rs"),
    )?;

    {
        let mut html_enum_file = BufWriter::new(File::create(
            &Path::new(&out_dir).join("codegen_attr_html_enums.rs"),
        )?);
        enums::codegen_static_attribute_enum("HtmlAttr", &attribute_defs, &mut html_enum_file)?;
        enums::codegen_static_attribute_names(&attribute_defs, &mut html_enum_file)?;
        enums::codegen_static_attribute_properties(&attribute_defs, &mut html_enum_file)?;
        enums::codegen_static_attribute_lookup("HtmlAttr", &attribute_defs, &mut html_enum_file)?;
        enums::codegen_static_property_lookup("HtmlAttr", &attribute_defs, &mut html_enum_file)?;
    }

    symbols::codegen_static_web_tag_ns_lookup_tables(
        &tag_defs,
        NamespaceDesc {
            name: "HTML5",
            path: "crate::html5",
        },
        Path::new(&out_dir).join("codegen_static_html_tags.rs"),
    )?;

    symbols::codegen_static_web_attr_ns_lookup_tables(
        &attribute_defs,
        NamespaceDesc {
            name: "HTML5",
            path: "crate::html5",
        },
        Path::new(&out_dir).join("codegen_static_html_attrs.rs"),
    )?;

    Ok(())
}

fn tag_defs() -> Vec<TagDef> {
    let mut defs = vec![];

    for (tag, is_void) in html5_defs::tags::DEFS {
        defs.push(TagDef {
            web_ns: "HTML5",
            static_id: defs.len(),
            pub_const_ident: format!("{}", tag.replace('-', "_").to_uppercase()),
            tag,
            is_void: is_void.0,
        });
    }

    defs
}

fn attribute_defs() -> Vec<AttributeDef> {
    let mut defs = vec![];

    for (attr, prop, flags) in html5_defs::attrs::DEFS {
        defs.push(AttributeDef {
            web_ns: "HTML5",
            kind: AttributeDefKind::Static(StaticAttributeDefKind {
                static_id: defs.len(),
                pub_const_ident: format!("{}", attr.replace('-', "_").to_uppercase()),
                enum_ident: attr
                    .split('-')
                    .map(|seg| {
                        let mut chars: Vec<char> = seg.chars().collect();
                        chars[0] = chars[0].to_ascii_uppercase();
                        chars.into_iter().collect::<String>()
                    })
                    .collect::<Vec<_>>()
                    .join(""),
                attr,
                prop,
                flags: *flags,
            }),
        });
    }

    defs.push(AttributeDef {
        web_ns: "HTML5",
        kind: AttributeDefKind::Data,
    });

    defs
}

// New style using enums
mod enums {
    use super::*;

    pub(crate) fn codegen_static_attribute_names<W: Write>(
        defs: &[AttributeDef],
        f: &mut BufWriter<W>,
    ) -> std::io::Result<()> {
        writeln!(f, "mod names {{")?;
        for def in defs.iter() {
            match &def.kind {
                AttributeDefKind::Static(static_kind) => {
                    writeln!(
                        f,
                        "    pub(crate) const {}: &str = \"{}\";",
                        static_kind.pub_const_ident, static_kind.attr
                    )?;
                }
                AttributeDefKind::Data => {}
            }
        }
        writeln!(f, "}}")?;
        Ok(())
    }

    pub(crate) fn codegen_static_attribute_properties<W: Write>(
        defs: &[AttributeDef],
        f: &mut BufWriter<W>,
    ) -> std::io::Result<()> {
        writeln!(f, "mod properties {{")?;
        for def in defs.iter() {
            match &def.kind {
                AttributeDefKind::Static(static_kind) => {
                    writeln!(
                        f,
                        "    pub(crate) const {}: &str = \"{}\";",
                        static_kind.pub_const_ident, static_kind.prop
                    )?;
                }
                AttributeDefKind::Data => {}
            }
        }
        writeln!(f, "}}")?;
        Ok(())
    }

    pub(crate) fn codegen_static_attribute_enum<W: Write>(
        enum_ident: &str,
        defs: &[AttributeDef],
        f: &mut BufWriter<W>,
    ) -> std::io::Result<()> {
        writeln!(f, "#[derive(Clone)]")?;
        writeln!(f, "pub enum {} {{", enum_ident)?;
        for def in defs.iter() {
            match &def.kind {
                AttributeDefKind::Static(static_kind) => {
                    writeln!(
                        f,
                        "    /// The {} '{}' attribute",
                        def.web_ns, static_kind.attr
                    )?;
                    writeln!(f, "    {ident},\n", ident = static_kind.enum_ident)?;
                }
                AttributeDefKind::Data => {
                    writeln!(f, "    /// Some {} 'data-' attribute", def.web_ns,)?;
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
                    AttributeDefKind::Static(static_kind) => {
                        writeln!(
                            f,
                            r#"            Self::{ident} => names::{pub_const_ident},"#,
                            ident = static_kind.enum_ident,
                            pub_const_ident = static_kind.pub_const_ident,
                        )?;
                    }
                    AttributeDefKind::Data => {
                        writeln!(f, "            Self::Dataset(data) => data.local_name(),")?;
                    }
                }
            }
            writeln!(f, "        }}")?;
            writeln!(f, "    }}")?;
            writeln!(f, "}}")?;
        }

        // PropertyName
        {
            writeln!(f, "impl crate::PropertyName for {} {{", enum_ident)?;
            writeln!(f, "    fn property_name(&self) -> &str {{")?;
            writeln!(f, "        match self {{")?;
            for def in defs.iter() {
                match &def.kind {
                    AttributeDefKind::Static(static_kind) => {
                        writeln!(
                            f,
                            r#"            Self::{ident} => properties::{pub_const_ident},"#,
                            ident = static_kind.enum_ident,
                            pub_const_ident = static_kind.pub_const_ident,
                        )?;
                    }
                    AttributeDefKind::Data => {
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

    pub(crate) fn codegen_static_attribute_lookup<W: Write>(
        enum_ident: &str,
        defs: &[AttributeDef],
        f: &mut BufWriter<W>,
    ) -> std::io::Result<()> {
        let static_defs: Vec<_> = defs.iter().filter_map(AttributeDef::static_kind).collect();

        // Attribute name map:
        {
            let def_keys: Vec<_> = static_defs
                .iter()
                .map(|def| {
                    (
                        def,
                        PhfKeyRef {
                            key: StaticUniCase::new(def.attr),
                            ref_expr: format!("StaticUniCase::new(names::{})", def.pub_const_ident),
                        },
                    )
                })
                .collect();

            let mut map_codegen: phf_codegen::Map<PhfKeyRef<StaticUniCase>> =
                phf_codegen::Map::new();
            for (def, key) in def_keys {
                map_codegen.entry(key, &format!("{}::{}", enum_ident, def.enum_ident));
            }

            writeln!(
                f,
                "pub(crate) const STATIC_ATTR_LOOKUP: phf::Map<StaticUniCase, {}> = {};",
                enum_ident,
                map_codegen.build()
            )?;
        }

        Ok(())
    }

    pub(crate) fn codegen_static_property_lookup<W: Write>(
        enum_ident: &str,
        defs: &[AttributeDef],
        f: &mut BufWriter<W>,
    ) -> std::io::Result<()> {
        let static_defs: Vec<_> = defs.iter().filter_map(AttributeDef::static_kind).collect();

        // Property name map:
        {
            let def_keys: Vec<_> = static_defs
                .iter()
                .map(|def| {
                    (
                        def,
                        PhfKeyRef {
                            key: def.prop,
                            ref_expr: format!("properties::{}", def.pub_const_ident),
                        },
                    )
                })
                .collect();

            let mut map_codegen: phf_codegen::Map<PhfKeyRef<&str>> = phf_codegen::Map::new();
            for (def, key) in def_keys {
                map_codegen.entry(key, &format!("{}::{}", enum_ident, def.enum_ident));
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

// Old-style symbols
mod symbols {
    use super::*;

    pub(crate) fn codegen_static_tag_symbols(
        defs: &[TagDef],
        out_path: std::path::PathBuf,
    ) -> std::io::Result<()> {
        let mut f = BufWriter::new(File::create(&out_path)?);

        writeln!(&mut f, "use crate::WebNS;")?;
        writeln!(
            &mut f,
            "use crate::static_web_tag::{{StaticWebTag, StaticWebTagSymbolNamespace}};"
        )?;
        writeln!(&mut f)?;

        // Symbol definition array:
        {
            writeln!(
                &mut f,
                "pub(crate) const __WEB_TAGS: [StaticWebTag; {len}] = [",
                len = defs.len()
            )?;

            for def in defs.iter() {
                writeln!(
                    &mut f,
                    r#"    StaticWebTag {{ web_ns: WebNS::{web_ns}, name: "{attr}" }},"#,
                    web_ns = def.web_ns,
                    attr = def.tag,
                )?;
            }

            writeln!(&mut f, "];\n",)?;
        }

        // Symbol namespace for all known attributes:
        {
            writeln!(
                &mut f,
                r#"
    pub(crate) const __TAG_SYMBOL_NS: StaticWebTagSymbolNamespace = StaticWebTagSymbolNamespace {{
        web_tags: &__WEB_TAGS,
    }};"#,
            )?;
        }

        Ok(())
    }

    pub(crate) fn codegen_static_attribute_symbols(
        defs: &[AttributeDef],
        out_path: std::path::PathBuf,
    ) -> std::io::Result<()> {
        let mut f = BufWriter::new(File::create(&out_path)?);

        writeln!(&mut f, "use crate::WebNS;")?;
        writeln!(
            &mut f,
            "use crate::static_web_attr::{{StaticWebAttr, StaticWebAttrSymbolNamespace}};"
        )?;
        writeln!(&mut f, "use crate::attr::attr_type::*;")?;
        writeln!(&mut f)?;

        // Symbol definition array:
        {
            writeln!(
                &mut f,
                "pub(crate) const __WEB_ATTRS: [StaticWebAttr; {len}] = [",
                len = defs.iter().filter_map(AttributeDef::static_kind).count()
            )?;

            for def in defs.iter() {
                if let Some(static_kind) = def.static_kind() {
                    writeln!(
                        &mut f,
                        r#"    StaticWebAttr {{ web_ns: WebNS::{web_ns}, name: "{attr}", property: "{prop}", attr_type: AttrType({flags}) }},"#,
                        web_ns = def.web_ns,
                        attr = static_kind.attr,
                        prop = static_kind.prop,
                        flags = static_kind.flags
                    )?;
                }
            }

            writeln!(&mut f, "];\n",)?;
        }

        // Symbol namespace for all known attributes:
        {
            writeln!(
                &mut f,
                r#"
    pub(crate) const __ATTR_SYMBOL_NS: StaticWebAttrSymbolNamespace = StaticWebAttrSymbolNamespace {{
        web_attrs: &__WEB_ATTRS,
    }};"#,
            )?;
        }

        Ok(())
    }

    pub(crate) fn codegen_static_web_tag_ns_lookup_tables(
        defs: &[TagDef],
        ns_desc: NamespaceDesc,
        out_path: std::path::PathBuf,
    ) -> std::io::Result<()> {
        let mut f = BufWriter::new(File::create(&out_path)?);

        let defs: Vec<_> = defs
            .iter()
            .filter(|def| def.web_ns == ns_desc.name)
            .collect();

        writeln!(&mut f, "use dyn_symbol::Symbol;")?;
        writeln!(
            &mut f,
            "use crate::static_web_tag::{{StaticWebTagLookupTable}};"
        )?;
        writeln!(&mut f, "use crate::static_unicase::*;")?;
        writeln!(&mut f, "use crate::symbols::tag::*;")?;
        writeln!(&mut f)?;

        // Attribute class:
        {
            writeln!(
                &mut f,
                r#"
    pub(crate) const __TAG_LOOKUP_TABLE: StaticWebTagLookupTable = StaticWebTagLookupTable {{"#,
            )?;

            // Tag name map:
            {
                let def_keys: Vec<_> = defs
                    .iter()
                    .map(|def| {
                        (
                            def,
                            PhfKeyRef {
                                key: StaticUniCase::new(def.tag),
                                ref_expr: format!(
                                    "StaticUniCase::new(__WEB_TAGS[{}].name)",
                                    def.static_id
                                ),
                            },
                        )
                    })
                    .collect();

                let mut map_codegen: phf_codegen::Map<PhfKeyRef<StaticUniCase>> =
                    phf_codegen::Map::new();
                for (def, key) in def_keys {
                    map_codegen.entry(key, &format!("{}", def.static_id));
                }

                writeln!(&mut f, "    tag_unicase_map: {},", map_codegen.build())?;
            }

            writeln!(&mut f, "}};\n",)?;
        }

        // Public interface:
        {
            for def in defs.iter() {
                writeln!(
                    &mut f,
                    r#"
    /// The {ns_name} `{tag}` element tag name
    pub const {pub_const_ident}: Symbol = Symbol::Static(&__TAG_SYMBOL_NS, {static_id});"#,
                    ns_name = ns_desc.name,
                    tag = def.tag,
                    pub_const_ident = def.pub_const_ident,
                    static_id = def.static_id,
                )?;
            }

            writeln!(&mut f, "",)?;
        }

        Ok(())
    }

    pub(crate) fn codegen_static_web_attr_ns_lookup_tables(
        defs: &[AttributeDef],
        ns_desc: NamespaceDesc,
        out_path: std::path::PathBuf,
    ) -> std::io::Result<()> {
        let mut f = BufWriter::new(File::create(&out_path)?);

        let defs: Vec<_> = defs
            .iter()
            .filter(|def| def.web_ns == ns_desc.name)
            .filter_map(AttributeDef::static_kind)
            .collect();

        writeln!(&mut f, "use dyn_symbol::Symbol;")?;
        writeln!(
            &mut f,
            "use crate::static_web_attr::{{StaticWebAttrLookupTables}};"
        )?;
        writeln!(&mut f, "use crate::static_unicase::*;")?;
        writeln!(&mut f, "use crate::symbols::attr::*;")?;
        writeln!(&mut f)?;

        // Attribute class:
        {
            writeln!(
                &mut f,
                r#"
    pub(crate) const __ATTR_LOOKUP_TABLES: StaticWebAttrLookupTables = StaticWebAttrLookupTables {{"#,
            )?;

            // Attribute name map:
            {
                let def_keys: Vec<_> = defs
                    .iter()
                    .map(|def| {
                        (
                            def,
                            PhfKeyRef {
                                key: StaticUniCase::new(def.attr),
                                ref_expr: format!(
                                    "StaticUniCase::new(__WEB_ATTRS[{}].name)",
                                    def.static_id
                                ),
                            },
                        )
                    })
                    .collect();

                let mut map_codegen: phf_codegen::Map<PhfKeyRef<StaticUniCase>> =
                    phf_codegen::Map::new();
                for (def, key) in def_keys {
                    map_codegen.entry(key, &format!("{}", def.static_id));
                }

                writeln!(
                    &mut f,
                    "    attribute_unicase_map: {},",
                    map_codegen.build()
                )?;
            }

            // Prop name map:
            {
                let def_keys: Vec<_> = defs
                    .iter()
                    .map(|def| {
                        (
                            def,
                            PhfKeyRef {
                                key: def.prop,
                                ref_expr: format!("__WEB_ATTRS[{}].property", def.static_id),
                            },
                        )
                    })
                    .collect();

                let mut map_codegen: phf_codegen::Map<PhfKeyRef<&'static str>> =
                    phf_codegen::Map::new();
                for (def, key) in def_keys {
                    map_codegen.entry(key, &format!("{}", def.static_id));
                }

                writeln!(&mut f, "    property_map: {},\n", map_codegen.build())?;
            }

            writeln!(&mut f, "}};\n",)?;
        }

        // Public interface:
        {
            for def in defs.iter() {
                writeln!(
                    &mut f,
                    r#"
    /// The {ns_name} `{attr}` attribute
    pub const {pub_const_ident}: Symbol = Symbol::Static(&__ATTR_SYMBOL_NS, {static_id});"#,
                    ns_name = ns_desc.name,
                    attr = def.attr,
                    pub_const_ident = def.pub_const_ident,
                    static_id = def.static_id,
                )?;
            }

            writeln!(&mut f, "",)?;
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
