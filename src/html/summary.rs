use kalosm_sample::*;
type SummaryAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Summary {
    attributes: Vec<SummaryAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Summary {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        SummaryAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</summary>")
            .map_output(|(attributes, body)| Summary { attributes, body })
    }
}
