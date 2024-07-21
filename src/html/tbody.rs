use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum TbodyAttributesName {
    #[parse(rename = " align=")]
    Align,
}
#[derive(Debug, Clone)]
pub enum TbodyAttributes {
    Align(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for TbodyAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(TbodyAttributesName::new_parser()
                .then_lazy(|name| match name {
                    TbodyAttributesName::Align => {
                        String::new_parser().map_output(Self::Align).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Tbody {
    attributes: Vec<TbodyAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Tbody {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        TbodyAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</tbody>")
            .map_output(|(attributes, body)| Tbody { attributes, body })
    }
}
