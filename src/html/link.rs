use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum LinkAttributesName {
    #[parse(rename = " as=")]
    As,
    #[parse(rename = " crossorigin=")]
    Crossorigin,
    #[parse(rename = " href=")]
    Href,
    #[parse(rename = " hreflang=")]
    Hreflang,
    #[parse(rename = " importance=")]
    Importance,
    #[parse(rename = " integrity=")]
    Integrity,
    #[parse(rename = " media=")]
    Media,
    #[parse(rename = " referrerpolicy=")]
    Referrerpolicy,
    #[parse(rename = " rel=")]
    Rel,
    #[parse(rename = " sizes=")]
    Sizes,
    #[parse(rename = " title=")]
    Title,
    #[parse(rename = " type=")]
    Type,
}
#[derive(Debug, Clone)]
pub enum LinkAttributes {
    As(crate::StringAttributeValue),
    Crossorigin(crate::XoValues),
    Href(crate::StringAttributeValue),
    Hreflang(crate::StringAttributeValue),
    Importance(crate::StringAttributeValue),
    Integrity(crate::StringAttributeValue),
    Media(crate::StringAttributeValue),
    Referrerpolicy(crate::StringAttributeValue),
    Rel(crate::StringAttributeValue),
    Sizes(crate::StringAttributeValue),
    Title(crate::StringAttributeValue),
    Type(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for LinkAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(LinkAttributesName::new_parser()
                .then_lazy(|name| match name {
                    LinkAttributesName::As => crate::StringAttributeValue::new_parser()
                        .map_output(Self::As)
                        .boxed(),
                    LinkAttributesName::Crossorigin => crate::XoValues::new_parser()
                        .map_output(Self::Crossorigin)
                        .boxed(),
                    LinkAttributesName::Href => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Href)
                        .boxed(),
                    LinkAttributesName::Hreflang => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Hreflang)
                        .boxed(),
                    LinkAttributesName::Importance => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Importance)
                        .boxed(),
                    LinkAttributesName::Integrity => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Integrity)
                        .boxed(),
                    LinkAttributesName::Media => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Media)
                        .boxed(),
                    LinkAttributesName::Referrerpolicy => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Referrerpolicy)
                        .boxed(),
                    LinkAttributesName::Rel => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Rel)
                        .boxed(),
                    LinkAttributesName::Sizes => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Sizes)
                        .boxed(),
                    LinkAttributesName::Title => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Title)
                        .boxed(),
                    LinkAttributesName::Type => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Type)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Link {
    attributes: Vec<LinkAttributes>,
}

impl kalosm_sample::Parse for Link {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        LinkAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Link { attributes })
    }
}
