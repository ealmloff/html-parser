use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum BdiAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for BdiAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Bdi {
    attributes: Vec<BdiAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Bdi {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        BdiAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</bdi>")
            .map_output(|(attributes, body)| Bdi { attributes, body })
    }
}
