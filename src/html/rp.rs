use kalosm_sample::*;
type RpAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Rp {
    attributes: Vec<RpAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Rp {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        RpAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</rp>")
            .map_output(|(attributes, body)| Rp { attributes, body })
    }
}
