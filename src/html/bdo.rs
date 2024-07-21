use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum BdoAttributesName {
    #[parse(rename = " dir=")]
    Dir,
}
#[derive(Debug, Clone)]
pub enum BdoAttributes {
    Dir(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for BdoAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(BdoAttributesName::new_parser()
                .then_lazy(|name| match name {
                    BdoAttributesName::Dir => String::new_parser().map_output(Self::Dir).boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Bdo {
    attributes: Vec<BdoAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Bdo {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        BdoAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</bdo>")
            .map_output(|(attributes, body)| Bdo { attributes, body })
    }
}
