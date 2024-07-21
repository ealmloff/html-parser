use kalosm_sample::*;
type SampAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Samp {
    attributes: Vec<SampAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Samp {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        SampAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</samp>")
            .map_output(|(attributes, body)| Samp { attributes, body })
    }
}
