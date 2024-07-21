use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum StrongAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for StrongAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Strong {
    attributes: Vec<StrongAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Strong {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        StrongAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</strong>")
            .map_output(|(attributes, body)| Strong { attributes, body })
    }
}
