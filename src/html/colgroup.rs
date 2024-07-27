use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ColgroupAttributesName {
    #[parse(rename = " align=")]
    Align,
    #[parse(rename = " span=")]
    Span,
}
#[derive(Debug, Clone)]
pub enum ColgroupAttributes {
    Align(crate::StringAttributeValue),
    Span(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for ColgroupAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(ColgroupAttributesName::new_parser()
                .then_lazy(|name| match name {
                    ColgroupAttributesName::Align => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Align)
                        .boxed(),
                    ColgroupAttributesName::Span => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Span)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Colgroup {
    attributes: Vec<ColgroupAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Colgroup {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ColgroupAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</colgroup>")
            .map_output(|(attributes, body)| Colgroup { attributes, body })
    }
}
