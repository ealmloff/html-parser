use kalosm_sample::*;
type AbbrAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Abbr {
    attributes: Vec<AbbrAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Abbr {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        AbbrAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</abbr>")
            .map_output(|(attributes, body)| Abbr { attributes, body })
    }
}
