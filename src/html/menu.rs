use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum MenuAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for MenuAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Menu {
    attributes: Vec<MenuAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Menu {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        MenuAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</menu>")
            .map_output(|(attributes, body)| Menu { attributes, body })
    }
}
