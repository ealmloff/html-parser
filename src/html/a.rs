use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum AAttributesName {
    #[parse(rename = " download=")]
    Download,
    #[parse(rename = " href=")]
    Href,
    #[parse(rename = " hreflang=")]
    Hreflang,
    #[parse(rename = " ping=")]
    Ping,
    #[parse(rename = " referrerpolicy=")]
    Referrerpolicy,
    #[parse(rename = " rel=")]
    Rel,
    #[parse(rename = " target=")]
    Target,
    #[parse(rename = " type=")]
    Type,
}
#[derive(Debug, Clone)]
pub enum AAttributes {
    Download(crate::StringAttributeValue),
    Href(crate::StringAttributeValue),
    Hreflang(crate::StringAttributeValue),
    Ping(crate::StringAttributeValue),
    Referrerpolicy(crate::StringAttributeValue),
    Rel(crate::StringAttributeValue),
    Target(crate::TargetValues),
    Type(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for AAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(AAttributesName::new_parser()
                .then_lazy(|name| match name {
                    AAttributesName::Download => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Download)
                        .boxed(),
                    AAttributesName::Href => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Href)
                        .boxed(),
                    AAttributesName::Hreflang => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Hreflang)
                        .boxed(),
                    AAttributesName::Ping => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Ping)
                        .boxed(),
                    AAttributesName::Referrerpolicy => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Referrerpolicy)
                        .boxed(),
                    AAttributesName::Rel => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Rel)
                        .boxed(),
                    AAttributesName::Target => crate::TargetValues::new_parser()
                        .map_output(Self::Target)
                        .boxed(),
                    AAttributesName::Type => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Type)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct A {
    attributes: Vec<AAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for A {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        AAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</a>")
            .map_output(|(attributes, body)| A { attributes, body })
    }
}
