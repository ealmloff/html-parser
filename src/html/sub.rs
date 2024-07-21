use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum SubAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for SubAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Sub {
    attributes: Vec<SubAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Sub {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        SubAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</sub>")
            .map_output(|(attributes, body)| Sub { attributes, body })
    }
}
