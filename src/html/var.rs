use kalosm_sample::*;
type VarAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Var {
    attributes: Vec<VarAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Var {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        VarAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</var>")
            .map_output(|(attributes, body)| Var { attributes, body })
    }
}
