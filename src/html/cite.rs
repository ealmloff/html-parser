use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum CiteAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for CiteAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Cite {
    attributes: Vec<CiteAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Cite {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        CiteAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</cite>")
            .map_output(|(attributes, body)| Cite { attributes, body })
    }
}
