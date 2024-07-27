use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum AreaAttributesName {
    #[parse(rename = " accesskey=")]
    Accesskey,
    #[parse(rename = " alt=")]
    Alt,
    #[parse(rename = " coords=")]
    Coords,
    #[parse(rename = " download=")]
    Download,
    #[parse(rename = " href=")]
    Href,
    #[parse(rename = " hreflang=")]
    Hreflang,
    #[parse(rename = " ping=")]
    Ping,
    #[parse(rename = " rel=")]
    Rel,
    #[parse(rename = " shape=")]
    Shape,
    #[parse(rename = " target=")]
    Target,
    #[parse(rename = " type=")]
    Type,
}
#[derive(Debug, Clone)]
pub enum AreaAttributes {
    Accesskey(crate::StringAttributeValue),
    Alt(crate::StringAttributeValue),
    Coords(crate::StringAttributeValue),
    Download(crate::StringAttributeValue),
    Href(crate::StringAttributeValue),
    Hreflang(crate::StringAttributeValue),
    Ping(crate::StringAttributeValue),
    Rel(crate::StringAttributeValue),
    Shape(crate::ShValues),
    Target(crate::TargetValues),
    Type(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for AreaAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(AreaAttributesName::new_parser()
                .then_lazy(|name| match name {
                    AreaAttributesName::Accesskey => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Accesskey)
                        .boxed(),
                    AreaAttributesName::Alt => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Alt)
                        .boxed(),
                    AreaAttributesName::Coords => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Coords)
                        .boxed(),
                    AreaAttributesName::Download => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Download)
                        .boxed(),
                    AreaAttributesName::Href => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Href)
                        .boxed(),
                    AreaAttributesName::Hreflang => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Hreflang)
                        .boxed(),
                    AreaAttributesName::Ping => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Ping)
                        .boxed(),
                    AreaAttributesName::Rel => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Rel)
                        .boxed(),
                    AreaAttributesName::Shape => crate::ShValues::new_parser()
                        .map_output(Self::Shape)
                        .boxed(),
                    AreaAttributesName::Target => crate::TargetValues::new_parser()
                        .map_output(Self::Target)
                        .boxed(),
                    AreaAttributesName::Type => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Type)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Area {
    attributes: Vec<AreaAttributes>,
}

impl kalosm_sample::Parse for Area {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        AreaAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Area { attributes })
    }
}
