use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum MarkAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for MarkAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Mark {
    attributes: Vec<MarkAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Mark {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        MarkAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</mark>")
            .map_output(|(attributes, body)| Mark { attributes, body })
    }
}
