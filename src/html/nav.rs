use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum NavAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for NavAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Nav {
    attributes: Vec<NavAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Nav {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        NavAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</nav>")
            .map_output(|(attributes, body)| Nav { attributes, body })
    }
}
