use kalosm_sample::*;
type H1Attributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct H1 {
    attributes: Vec<H1Attributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for H1 {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        H1Attributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</h1>")
            .map_output(|(attributes, body)| H1 { attributes, body })
    }
}
