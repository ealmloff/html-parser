use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum TdAttributesName {
    #[parse(rename = " abbr=")]
    Abbr,
    #[parse(rename = " align=")]
    Align,
    #[parse(rename = " axis=")]
    Axis,
    #[parse(rename = " bgcolor=")]
    Bgcolor,
    #[parse(rename = " colspan=")]
    Colspan,
    #[parse(rename = " headers=")]
    Headers,
    #[parse(rename = " rowspan=")]
    Rowspan,
}
#[derive(Debug, Clone)]
pub enum TdAttributes {
    Abbr(crate::StringAttributeValue),
    Align(crate::StringAttributeValue),
    Axis(crate::StringAttributeValue),
    Bgcolor(crate::StringAttributeValue),
    Colspan(crate::StringAttributeValue),
    Headers(crate::StringAttributeValue),
    Rowspan(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for TdAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(TdAttributesName::new_parser()
                .then_lazy(|name| match name {
                    TdAttributesName::Abbr => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Abbr)
                        .boxed(),
                    TdAttributesName::Align => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Align)
                        .boxed(),
                    TdAttributesName::Axis => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Axis)
                        .boxed(),
                    TdAttributesName::Bgcolor => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Bgcolor)
                        .boxed(),
                    TdAttributesName::Colspan => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Colspan)
                        .boxed(),
                    TdAttributesName::Headers => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Headers)
                        .boxed(),
                    TdAttributesName::Rowspan => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Rowspan)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Td {
    attributes: Vec<TdAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Td {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        TdAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</td>")
            .map_output(|(attributes, body)| Td { attributes, body })
    }
}
