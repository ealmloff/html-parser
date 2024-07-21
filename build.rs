#![allow(dead_code)]

use convert_case::Casing;
use serde::Deserialize;
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
    writeln!(out, "#![allow(dead_code)]")?;
    writeln!(out, "use kalosm_sample::*;")?;

    contents.write_value_set(&mut out)?;
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
                "// Could not find value set\nString".to_string()
            }
        } else {
            "String".to_string()
        };
        value
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
            let element_rust_name = to_upper_camel_case(&element.name);
            let mut merged_attributes = element.attributes.clone();
            merged_attributes.extend(self.global_attributes.clone());
            let mut attributes = HashSet::new();
            writeln!(out, "#[derive(Debug, Clone)]")?;
            writeln!(out, "pub enum {}Attributes {{", element_rust_name)?;
            for attribute in &merged_attributes {
                if !attributes.insert(attribute.name.clone()) {
                    continue;
                }
                let attribute_rust_name = to_upper_camel_case(&attribute.name);
                let value = self.get_value(&attribute.value_set);
                writeln!(out, "    {attribute_rust_name}({value}),")?;
            }
            writeln!(out, "}}")?;

            // Implement the Parse trait for the element
            writeln!(out)?;

            writeln!(out, "impl kalosm_sample::Parse for {element_rust_name}Attributes {{")?;
            writeln!(out, "    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {{")?;
            writeln!(out, "        use kalosm_sample::*;")?;
            for (i, attribute) in merged_attributes.iter().enumerate() {
                let name = &attribute.name;
                let attribute_rust_name = to_upper_camel_case(name);
                let value = self.get_value(&attribute.value_set);
                if i > 0 {
                    writeln!(out, "        .or(")?;
                }
                writeln!(out, "        LiteralParser::new(\"\\\"{name}\\\"=\")")?; 
                writeln!(out, "            .ignore_output_then({value}::new_parser())")?;
                writeln!(out, "            .map_output(|value| Self::{attribute_rust_name}(value))")?;
                if i > 0 {
                    writeln!(out, "        )")?;
                }
            }
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }

        // Write a struct for each element
        for element in &self.tags {
            let name = &element.name;
            let element_rust_name = to_upper_camel_case(name);
            writeln!(out, "#[derive(Debug, Clone)]")?;
            writeln!(out, "pub struct {element_rust_name}{{")?;
            writeln!(out, "    attributes: Vec<{element_rust_name}Attributes>,")?;
            writeln!(out, "    body: Vec<Element>,")?;
            writeln!(out, "}}")?;
            // Implement the Parse trait for the element
            writeln!(out)?;
            writeln!(out, "impl kalosm_sample::Parse for {element_rust_name} {{")?;
            writeln!(out, "    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {{")?;
            writeln!(out, "        use kalosm_sample::*;")?;
            writeln!(out, "        LiteralParser::new(\"<{name}\")")?;
            writeln!(out, "            .ignore_output_then(")?;
            writeln!(out, "                LiteralParser::new(\" \")")?;
            writeln!(out, "                    .then({element_rust_name}Attributes::new_parser())")?;
            writeln!(out, "                    .repeat(0..)")?;
            writeln!(out, "            )")?;
            writeln!(out, "            .then_literal(\">\")")?;
            writeln!(out, "            .then(Element::new_parser().repeat(0..))")?;
            writeln!(out, "            .then_literal(\"</{name}>\")")?;
            writeln!(out, "            .map_output(|(attributes, body)| {element_rust_name} {{ attributes, body }})")?;
            writeln!(out, "    }}")?;
            writeln!(out, "}}")?;
        }

        // Write the Element enum
        writeln!(out, "#[derive(Debug, Clone)]")?;
        writeln!(out, "pub enum Element {{")?;
        for element in &self.tags {
            let element_rust_name = to_upper_camel_case(&element.name);
            writeln!(out, "    {element_rust_name}({element_rust_name}),")?;
        }
        writeln!(out, "}}")?;

        Ok(())
    }
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
        writeln!(
            out,
            "#[derive(Debug, Clone, Copy, Parse)]"
        )?;
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
