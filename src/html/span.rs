use kalosm_sample::*;
type SpanAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Span {
    attributes: Vec<SpanAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Span {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        SpanAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</span>")
            .map_output(|(attributes, body)| Span { attributes, body })
    }
}
