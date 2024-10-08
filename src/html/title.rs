use kalosm_sample::*;
type TitleAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Title {
    attributes: Vec<TitleAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Title {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        TitleAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</title>")
            .map_output(|(attributes, body)| Title { attributes, body })
    }
}
