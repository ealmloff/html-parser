use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum OlAttributesName {
    #[parse(rename = " compact=")]
    Compact,
    #[parse(rename = " reversed=")]
    Reversed,
    #[parse(rename = " start=")]
    Start,
    #[parse(rename = " type=")]
    Type,
}
#[derive(Debug, Clone)]
pub enum OlAttributes {
    Compact(String),
    Reversed(String),
    Start(String),
    Type(crate::LtValues),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for OlAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(OlAttributesName::new_parser()
                .then_lazy(|name| match name {
                    OlAttributesName::Compact => {
                        String::new_parser().map_output(Self::Compact).boxed()
                    }
                    OlAttributesName::Reversed => {
                        String::new_parser().map_output(Self::Reversed).boxed()
                    }
                    OlAttributesName::Start => String::new_parser().map_output(Self::Start).boxed(),
                    OlAttributesName::Type => {
                        crate::LtValues::new_parser().map_output(Self::Type).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Ol {
    attributes: Vec<OlAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Ol {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        OlAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</ol>")
            .map_output(|(attributes, body)| Ol { attributes, body })
    }
}
