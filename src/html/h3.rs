use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum H3Attributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for H3Attributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct H3 {
    attributes: Vec<H3Attributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for H3 {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        H3Attributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</h3>")
            .map_output(|(attributes, body)| H3 { attributes, body })
    }
}
