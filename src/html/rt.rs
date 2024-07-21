use kalosm_sample::*;
type RtAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Rt {
    attributes: Vec<RtAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Rt {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        RtAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</rt>")
            .map_output(|(attributes, body)| Rt { attributes, body })
    }
}
