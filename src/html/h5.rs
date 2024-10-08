use kalosm_sample::*;
type H5Attributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct H5 {
    attributes: Vec<H5Attributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for H5 {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        H5Attributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</h5>")
            .map_output(|(attributes, body)| H5 { attributes, body })
    }
}
