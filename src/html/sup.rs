use kalosm_sample::*;
type SupAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Sup {
    attributes: Vec<SupAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Sup {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        SupAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</sup>")
            .map_output(|(attributes, body)| Sup { attributes, body })
    }
}
