use kalosm_sample::*;
type DtAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Dt {
    attributes: Vec<DtAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Dt {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        DtAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</dt>")
            .map_output(|(attributes, body)| Dt { attributes, body })
    }
}
