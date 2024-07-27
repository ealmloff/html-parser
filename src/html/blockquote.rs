use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum BlockquoteAttributesName {
    #[parse(rename = " cite=")]
    Cite,
}
#[derive(Debug, Clone)]
pub enum BlockquoteAttributes {
    Cite(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for BlockquoteAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(BlockquoteAttributesName::new_parser()
                .then_lazy(|name| match name {
                    BlockquoteAttributesName::Cite => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Cite)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Blockquote {
    attributes: Vec<BlockquoteAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Blockquote {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        BlockquoteAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</blockquote>")
            .map_output(|(attributes, body)| Blockquote { attributes, body })
    }
}
