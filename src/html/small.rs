use kalosm_sample::*;
type SmallAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Small {
    attributes: Vec<SmallAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Small {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        SmallAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</small>")
            .map_output(|(attributes, body)| Small { attributes, body })
    }
}
