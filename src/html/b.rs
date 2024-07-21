use kalosm_sample::*;
type BAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct B {
    attributes: Vec<BAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for B {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        BAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</b>")
            .map_output(|(attributes, body)| B { attributes, body })
    }
}
