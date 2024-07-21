use kalosm_sample::*;
type IAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct I {
    attributes: Vec<IAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for I {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        IAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</i>")
            .map_output(|(attributes, body)| I { attributes, body })
    }
}
