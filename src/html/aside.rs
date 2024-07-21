use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum AsideAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for AsideAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Aside {
    attributes: Vec<AsideAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Aside {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        AsideAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</aside>")
            .map_output(|(attributes, body)| Aside { attributes, body })
    }
}
