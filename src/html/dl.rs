use kalosm_sample::*;
type DlAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Dl {
    attributes: Vec<DlAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Dl {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        DlAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</dl>")
            .map_output(|(attributes, body)| Dl { attributes, body })
    }
}
