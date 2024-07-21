use kalosm_sample::*;
type KbdAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Kbd {
    attributes: Vec<KbdAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Kbd {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        KbdAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</kbd>")
            .map_output(|(attributes, body)| Kbd { attributes, body })
    }
}
