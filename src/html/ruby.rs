use kalosm_sample::*;
type RubyAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Ruby {
    attributes: Vec<RubyAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Ruby {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        RubyAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</ruby>")
            .map_output(|(attributes, body)| Ruby { attributes, body })
    }
}
