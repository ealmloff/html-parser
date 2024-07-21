use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum DfnAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for DfnAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Dfn {
    attributes: Vec<DfnAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Dfn {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        DfnAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</dfn>")
            .map_output(|(attributes, body)| Dfn { attributes, body })
    }
}
