#![allow(dead_code)]
use convert_case::Casing;
use serde::Deserialize;
use std::path::PathBuf;
use std::{collections::HashSet, io::Write};

const TESTED_COMMIT: &str = "https://raw.githubusercontent.com/microsoft/vscode-custom-data/52ef9eadc08e33f5b1921f0c9d23adbf1cf99e9d/web-data/data/browsers.html-data.json";
const LATEST_COMMIT: &str = "https://raw.githubusercontent.com/microsoft/vscode-custom-data/main/web-data/data/browsers.html-data.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if option_env!("UPDATE_BINDINGS") != Some("true") {
        return Ok(());
    }

    // Try with the latest commit first, then with the tested commit if that fails
    let mut contents = reqwest::blocking::get(LATEST_COMMIT).and_then(|response| {
        response.json::<Response>().or_else(|_| {
            reqwest::blocking::get(TESTED_COMMIT).and_then(|response| response.json::<Response>())
        })
    })?;

    let out_file = std::fs::File::create("./src/html.rs")?;
    let mut out = std::io::BufWriter::new(out_file);

    writeln!(out, "#![allow(clippy::enum_variant_names)]")?;
    writeln!(out, "#![allow(clippy::module_inception)]")?;
    writeln!(out, "#![allow(unused_imports)]")?;
    writeln!(out, "#![allow(dead_code)]")?;
    writeln!(out, "use kalosm_sample::*;")?;

    contents.write_value_set(&mut out)?;
    contents.write_global_attributes(&mut out)?;
    contents.write_elements(&mut out)?;

    Ok(())
}

#[derive(Deserialize)]
struct Response {
    tags: Vec<Tag>,
    #[serde(rename = "globalAttributes")]
    global_attributes: Vec<Attribute>,
    #[serde(rename = "valueSets")]
    value_sets: Vec<ValueSet>,
}

impl Response {
    fn write_value_set(
        &mut self,
        out: &mut impl std::io::Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for value_set in &mut self.value_sets {
            value_set.write(out)?;
        }
        Ok(())
    }

    fn get_value(&self, value_set: &Option<String>) -> String {
        let value = if let Some(value) = &value_set {
            if let Some(value) = self
                .value_sets
                .iter()
                .find(|value_set| value_set.name == *value)
            {
                value.rust_name.clone()
            } else {
                "String".to_string()
            }
        } else {
            "String".to_string()
        };
        value
    }

    fn write_global_attributes(
        &mut self,
        out: &mut impl std::io::Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        write_attribute_name_enum(out, "GlobalAttributeName", &self.global_attributes)?;
        // Create an attributes enum for the element
        writeln!(out, "#[derive(Debug, Clone)]")?;
        writeln!(out, "pub enum GlobalAttribute {{")?;
        for attribute in &self.global_attributes {
            let attribute_rust_name = to_upper_camel_case(&attribute.name);
            let mut value = self.get_value(&attribute.value_set);
            if value != "String" {
                value = format!("crate::{value}");
            }
            writeln!(out, "    {attribute_rust_name}({value}),")?;
        }
        writeln!(out, "}}")?;
        writeln!(out)?;
        writeln!(out, "impl kalosm_sample::Parse for GlobalAttribute {{")?;
        writeln!(
            out,
            "    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {{"
        )?;
        writeln!(out, "        use kalosm_sample::*;")?;
        writeln!(out, "        GlobalAttributeName::new_parser()")?;
        writeln!(out, "        .then_lazy(|name| match name {{")?;
        for attribute in &self.global_attributes {
            let name = &attribute.name;
            let attribute_rust_name = to_upper_camel_case(name);
            let mut value = self.get_value(&attribute.value_set);
            if value != "String" {
                value = format!("crate::{value}");
            }
            writeln!(
                out,
                "        GlobalAttributeName::{attribute_rust_name} => {{"
            )?;
            writeln!(
                out,
                "            {value}::new_parser().map_output(Self::{attribute_rust_name}).boxed()",
            )?;
            writeln!(out, "        }}")?;
        }
        writeln!(out, "        }}).map_output(|(_, attribute)| attribute)")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;
        Ok(())
    }

    fn write_elements(
        &mut self,
        out: &mut impl std::io::Write,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // First, deduplicate the tags
        self.tags.sort_by(|a, b| a.name.cmp(&b.name));
        self.tags.dedup_by(|a, b| a.name == b.name);

        for tag in &mut self.tags {
            tag.deduplicate();
        }

        // Write the attributes that are valid just for each element
        for element in &self.tags {
            let element_file_name = to_lower_camel_case(&element.name);
            let path = PathBuf::from(format!("./src/html/{}.rs", element_file_name));
            std::fs::create_dir_all(path.parent().unwrap())?;
            let element_file = std::fs::File::create(path)?;
            let mut element_out = std::io::BufWriter::new(element_file);
            writeln!(out, "mod {};", element_file_name)?;
            writeln!(out, "pub use {}::*;", element_file_name)?;
            writeln!(element_out, "use kalosm_sample::*;")?;

            let element_rust_name = to_upper_camel_case(&element.name);
            let mut attributes = element.attributes.clone();
            attributes.sort_by_key(|attribute| attribute.name.clone());
            attributes.dedup_by_key(|attribute| attribute.name.clone());

            write_attribute_name_enum(
                &mut element_out,
                &format!("{}AttributesName", element_rust_name),
                &attributes,
            )?;

            // Create an attributes enum for the element
            writeln!(element_out, "#[derive(Debug, Clone)]")?;
            writeln!(element_out, "pub enum {}Attributes {{", element_rust_name)?;
            for attribute in &attributes {
                let attribute_rust_name = to_upper_camel_case(&attribute.name);
                let mut value = self.get_value(&attribute.value_set);
                if value != "String" {
                    value = format!("crate::{value}");
                }
                writeln!(element_out, "    {attribute_rust_name}({value}),")?;
            }
            writeln!(element_out, "    GlobalAttribute(crate::GlobalAttribute),")?;
            writeln!(element_out, "}}")?;
            writeln!(out)?;
            writeln!(
                element_out,
                "impl kalosm_sample::Parse for {element_rust_name}Attributes {{"
            )?;
            writeln!(element_out, "    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {{")?;
            writeln!(
                element_out,
                "        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)"
            )?;
            if !attributes.is_empty() {
                writeln!(
                    element_out,
                    "        .boxed().or({element_rust_name}AttributesName::new_parser()"
                )?;
                writeln!(element_out, "        .then_lazy(|name| match name {{")?;
                for attribute in attributes.iter() {
                    let name = &attribute.name;
                    let attribute_rust_name = to_upper_camel_case(name);
                    let mut value = self.get_value(&attribute.value_set);
                    if value != "String" {
                        value = format!("crate::{value}");
                    }
                    writeln!(
                        element_out,
                        "        {element_rust_name}AttributesName::{attribute_rust_name} => {{"
                    )?;
                    writeln!(element_out, "            {value}::new_parser().map_output(Self::{attribute_rust_name}).boxed()",)?;
                    writeln!(element_out, "        }}")?;
                }
                writeln!(
                    element_out,
                    "        }}).map_output(|(_, attribute)| attribute).boxed())"
                )?;
            }
            writeln!(element_out, "    }}")?;
            writeln!(element_out, "}}")?;

            let name = &element.name;
            let element_rust_name = to_upper_camel_case(name);
            writeln!(element_out, "#[derive(Debug, Clone)]")?;
            writeln!(element_out, "pub struct {element_rust_name}{{")?;
            writeln!(
                element_out,
                "    attributes: Vec<{element_rust_name}Attributes>,"
            )?;
            if !SELF_CLOSING_ELEMENTS.contains(&element.name.as_str()) {
                writeln!(element_out, "    body: Vec<crate::Node>,")?;
            }
            writeln!(element_out, "}}")?;

            // Implement the Parse trait for the element
            writeln!(element_out)?;
            writeln!(
                element_out,
                "impl kalosm_sample::Parse for {element_rust_name} {{"
            )?;
            writeln!(element_out, "    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {{")?;
            writeln!(element_out, "        use kalosm_sample::*;")?;
            writeln!(
                element_out,
                "                {element_rust_name}Attributes::new_parser()"
            )?;
            writeln!(element_out, "                .repeat(0..=10000)")?;
            if SELF_CLOSING_ELEMENTS.contains(&element.name.as_str()) {
                writeln!(element_out, "                .then_literal(\"/>\")")?;
                writeln!(element_out, "                .map_output(|attributes| {element_rust_name} {{ attributes }})")?;
            } else {
                writeln!(element_out, "                .then_literal(\">\")")?;
                writeln!(
                    element_out,
                    "                .then(kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed()).repeat(0..=10000))"
                )?;
                writeln!(element_out, "                .then_literal(\"</{name}>\")")?;
                writeln!(element_out, "                .map_output(|(attributes, body)| {element_rust_name} {{ attributes, body }})")?;
            }
            writeln!(element_out, "    }}")?;
            writeln!(element_out, "}}")?;
        }

        // Write the Element name enum
        writeln!(out, "#[derive(Debug, Clone, Parse)]")?;
        writeln!(out, "#[parse(unquoted)]")?;
        writeln!(out, "pub enum ElementName {{")?;
        for element in &self.tags {
            let element_rust_name = to_upper_camel_case(&element.name);
            writeln!(out, "    #[parse(rename = \"<{}\")]", element.name)?;
            writeln!(out, "    {element_rust_name},")?;
        }
        writeln!(out, "}}")?;

        // Write the Element enum
        writeln!(out, "#[derive(Debug, Clone)]")?;
        writeln!(out, "pub enum Element {{")?;
        for element in &self.tags {
            let element_rust_name = to_upper_camel_case(&element.name);
            writeln!(out, "    {element_rust_name}({element_rust_name}),")?;
        }
        writeln!(out, "}}")?;

        // Implement Parse for the element enum
        writeln!(out, "impl kalosm_sample::Parse for Element {{")?;
        writeln!(
            out,
            "    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {{"
        )?;
        writeln!(out, "        use kalosm_sample::*;")?;
        writeln!(out, "        ElementName::new_parser()")?;
        writeln!(out, "            .then_lazy(|name| match name {{")?;
        for element in self.tags.iter() {
            let element_rust_name = to_upper_camel_case(&element.name);
            writeln!(out, "                ElementName::{element_rust_name} => {element_rust_name}::new_parser().map_output(Self::{element_rust_name}).boxed(),")?;
        }
        writeln!(out, "            }}).map_output(|(_, element)| element)")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }
}

fn write_attribute_name_enum(
    out: &mut impl std::io::Write,
    name: &str,
    attributes: &[Attribute],
) -> std::io::Result<()> {
    if attributes.is_empty() {
        return Ok(());
    }

    // Create an attributes name enum for the element
    writeln!(out, "#[derive(Debug, Clone, Parse)]")?;
    writeln!(out, "#[parse(unquoted)]")?;
    writeln!(out, "pub enum {name} {{",)?;
    for attribute in attributes {
        let attribute_rust_name = to_upper_camel_case(&attribute.name);
        writeln!(out, "    #[parse(rename = \" {}=\")]", attribute.name)?;
        writeln!(out, "    {attribute_rust_name},")?;
    }
    writeln!(out, "}}")?;
    Ok(())
}

fn to_upper_camel_case(s: &str) -> String {
    let mut name = s
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
        .collect::<String>();
    if name.chars().next().unwrap().is_numeric() || name == "self" {
        name = format!("Value {}", name);
    }
    name.to_case(convert_case::Case::UpperCamel)
}

fn to_lower_camel_case(s: &str) -> String {
    let mut name = s
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
        .collect::<String>();
    if name.chars().next().unwrap().is_numeric() || name == "self" {
        name = format!("mod {}", name);
    }
    name.to_case(convert_case::Case::Camel)
}

#[derive(Deserialize)]
struct Tag {
    name: String,
    description: Description,
    attributes: Vec<Attribute>,
    references: Vec<Reference>,
}

impl Tag {
    fn deduplicate(&mut self) {
        self.attributes.sort_by(|a, b| a.name.cmp(&b.name));
        self.attributes.dedup_by(|a, b| a.name == b.name);
    }
}

#[derive(Deserialize, Clone)]
struct Attribute {
    name: String,
    description: Option<Description>,
    #[serde(rename = "valueSet")]
    value_set: Option<String>,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum Description {
    Simple(String),
    Complex { kind: String, value: String },
}

#[derive(Deserialize)]
struct Reference {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct ValueSet {
    name: String,
    values: Vec<Value>,
    #[serde(skip)]
    rust_name: String,
}

impl ValueSet {
    fn deduplicate(&mut self) {
        self.values.sort_by_key(|value| value.name.clone());
        self.values.dedup_by_key(|value| value.name.clone());
    }

    fn ident(&self) -> String {
        let name = to_upper_camel_case(&self.name);
        format!("{}Values", name)
    }

    fn write(&mut self, out: &mut impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
        self.deduplicate();
        let enum_rust_name = self.ident();
        self.rust_name.clone_from(&enum_rust_name);
        writeln!(out, "#[derive(Debug, Clone, Copy, Parse)]")?;
        writeln!(out, "pub enum {} {{", enum_rust_name)?;
        let mut variants = HashSet::new();
        for value in &mut self.values {
            let mut value_rust_name = to_upper_camel_case(&value.name);
            let mut count = 2;
            while !variants.insert(value_rust_name.clone()) {
                value_rust_name = format!("{}{}", value_rust_name, count);
                count += 1;
            }
            writeln!(out, "    #[parse(rename = \"{}\")]", value.name)?;
            writeln!(out, "    {},", value_rust_name)?;
        }
        writeln!(out, "}}")?;
        Ok(())
    }
}

#[derive(Deserialize)]
struct Value {
    name: String,
    description: Option<Description>,
}

const SELF_CLOSING_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];
