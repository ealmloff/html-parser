use kalosm_sample::*;
type FooterAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Footer {
    attributes: Vec<FooterAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Footer {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        FooterAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</footer>")
            .map_output(|(attributes, body)| Footer { attributes, body })
    }
}
