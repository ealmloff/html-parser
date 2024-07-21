use kalosm_sample::*;
type DivAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Div {
    attributes: Vec<DivAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Div {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        DivAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</div>")
            .map_output(|(attributes, body)| Div { attributes, body })
    }
}
