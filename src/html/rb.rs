use kalosm_sample::*;
type RbAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Rb {
    attributes: Vec<RbAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Rb {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        RbAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</rb>")
            .map_output(|(attributes, body)| Rb { attributes, body })
    }
}
