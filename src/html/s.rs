use kalosm_sample::*;
type SAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct S {
    attributes: Vec<SAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for S {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        SAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</s>")
            .map_output(|(attributes, body)| S { attributes, body })
    }
}
