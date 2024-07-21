use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum UAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for UAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct U {
    attributes: Vec<UAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for U {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        UAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</u>")
            .map_output(|(attributes, body)| U { attributes, body })
    }
}
