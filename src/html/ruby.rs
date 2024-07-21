use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum RubyAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for RubyAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
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
