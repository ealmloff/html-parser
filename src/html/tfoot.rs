use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum TfootAttributesName {
    #[parse(rename = " align=")]
    Align,
}
#[derive(Debug, Clone)]
pub enum TfootAttributes {
    Align(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for TfootAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(TfootAttributesName::new_parser()
                .then_lazy(|name| match name {
                    TfootAttributesName::Align => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Align)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Tfoot {
    attributes: Vec<TfootAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Tfoot {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        TfootAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</tfoot>")
            .map_output(|(attributes, body)| Tfoot { attributes, body })
    }
}
