use kalosm_sample::*;
type H4Attributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct H4 {
    attributes: Vec<H4Attributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for H4 {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        H4Attributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</h4>")
            .map_output(|(attributes, body)| H4 { attributes, body })
    }
}
