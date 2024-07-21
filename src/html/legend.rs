use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum LegendAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for LegendAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Legend {
    attributes: Vec<LegendAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Legend {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        LegendAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</legend>")
            .map_output(|(attributes, body)| Legend { attributes, body })
    }
}
