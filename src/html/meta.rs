use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum MetaAttributesName {
    #[parse(rename = " charset=")]
    Charset,
    #[parse(rename = " content=")]
    Content,
    #[parse(rename = " http-equiv=")]
    HttpEquiv,
    #[parse(rename = " name=")]
    Name,
    #[parse(rename = " scheme=")]
    Scheme,
}
#[derive(Debug, Clone)]
pub enum MetaAttributes {
    Charset(crate::StringAttributeValue),
    Content(crate::StringAttributeValue),
    HttpEquiv(crate::StringAttributeValue),
    Name(crate::StringAttributeValue),
    Scheme(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for MetaAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(MetaAttributesName::new_parser()
                .then_lazy(|name| match name {
                    MetaAttributesName::Charset => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Charset)
                        .boxed(),
                    MetaAttributesName::Content => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Content)
                        .boxed(),
                    MetaAttributesName::HttpEquiv => crate::StringAttributeValue::new_parser()
                        .map_output(Self::HttpEquiv)
                        .boxed(),
                    MetaAttributesName::Name => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Name)
                        .boxed(),
                    MetaAttributesName::Scheme => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Scheme)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Meta {
    attributes: Vec<MetaAttributes>,
}

impl kalosm_sample::Parse for Meta {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        MetaAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Meta { attributes })
    }
}
