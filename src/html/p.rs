use kalosm_sample::*;
type PAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct P {
    attributes: Vec<PAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for P {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        PAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</p>")
            .map_output(|(attributes, body)| P { attributes, body })
    }
}
