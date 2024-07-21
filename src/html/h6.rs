use kalosm_sample::*;
type H6Attributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct H6 {
    attributes: Vec<H6Attributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for H6 {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        H6Attributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</h6>")
            .map_output(|(attributes, body)| H6 { attributes, body })
    }
}
