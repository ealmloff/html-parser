use kalosm_sample::*;
type H3Attributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct H3 {
    attributes: Vec<H3Attributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for H3 {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        H3Attributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</h3>")
            .map_output(|(attributes, body)| H3 { attributes, body })
    }
}
