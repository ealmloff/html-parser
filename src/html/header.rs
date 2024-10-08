use kalosm_sample::*;
type HeaderAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Header {
    attributes: Vec<HeaderAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Header {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        HeaderAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</header>")
            .map_output(|(attributes, body)| Header { attributes, body })
    }
}
