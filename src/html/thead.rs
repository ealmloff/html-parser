use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum TheadAttributesName {
    #[parse(rename = " align=")]
    Align,
}
#[derive(Debug, Clone)]
pub enum TheadAttributes {
    Align(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for TheadAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(TheadAttributesName::new_parser()
                .then_lazy(|name| match name {
                    TheadAttributesName::Align => {
                        String::new_parser().map_output(Self::Align).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Thead {
    attributes: Vec<TheadAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Thead {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        TheadAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</thead>")
            .map_output(|(attributes, body)| Thead { attributes, body })
    }
}
