use kalosm_sample::*;
type MainAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Main {
    attributes: Vec<MainAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Main {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        MainAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</main>")
            .map_output(|(attributes, body)| Main { attributes, body })
    }
}
