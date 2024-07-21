use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum H2Attributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for H2Attributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct H2 {
    attributes: Vec<H2Attributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for H2 {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        H2Attributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</h2>")
            .map_output(|(attributes, body)| H2 { attributes, body })
    }
}
