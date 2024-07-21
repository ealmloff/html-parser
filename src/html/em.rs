use kalosm_sample::*;
type EmAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Em {
    attributes: Vec<EmAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Em {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        EmAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</em>")
            .map_output(|(attributes, body)| Em { attributes, body })
    }
}
