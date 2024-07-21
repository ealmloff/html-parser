use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum FigureAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for FigureAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Figure {
    attributes: Vec<FigureAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Figure {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        FigureAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</figure>")
            .map_output(|(attributes, body)| Figure { attributes, body })
    }
}
