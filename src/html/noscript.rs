use kalosm_sample::*;
type NoscriptAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Noscript {
    attributes: Vec<NoscriptAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Noscript {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        NoscriptAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</noscript>")
            .map_output(|(attributes, body)| Noscript { attributes, body })
    }
}
