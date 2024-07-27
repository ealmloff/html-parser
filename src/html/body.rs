use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum BodyAttributesName {
    #[parse(rename = " alink=")]
    Alink,
    #[parse(rename = " background=")]
    Background,
    #[parse(rename = " bgcolor=")]
    Bgcolor,
    #[parse(rename = " bottommargin=")]
    Bottommargin,
    #[parse(rename = " leftmargin=")]
    Leftmargin,
    #[parse(rename = " link=")]
    Link,
    #[parse(rename = " onafterprint=")]
    Onafterprint,
    #[parse(rename = " onbeforeprint=")]
    Onbeforeprint,
    #[parse(rename = " onbeforeunload=")]
    Onbeforeunload,
    #[parse(rename = " onblur=")]
    Onblur,
    #[parse(rename = " onerror=")]
    Onerror,
    #[parse(rename = " onfocus=")]
    Onfocus,
    #[parse(rename = " onhashchange=")]
    Onhashchange,
    #[parse(rename = " onlanguagechange=")]
    Onlanguagechange,
    #[parse(rename = " onload=")]
    Onload,
    #[parse(rename = " onmessage=")]
    Onmessage,
    #[parse(rename = " onoffline=")]
    Onoffline,
    #[parse(rename = " ononline=")]
    Ononline,
    #[parse(rename = " onpagehide=")]
    Onpagehide,
    #[parse(rename = " onpageshow=")]
    Onpageshow,
    #[parse(rename = " onpopstate=")]
    Onpopstate,
    #[parse(rename = " onredo=")]
    Onredo,
    #[parse(rename = " onresize=")]
    Onresize,
    #[parse(rename = " onstorage=")]
    Onstorage,
    #[parse(rename = " onundo=")]
    Onundo,
    #[parse(rename = " onunload=")]
    Onunload,
    #[parse(rename = " rightmargin=")]
    Rightmargin,
    #[parse(rename = " text=")]
    Text,
    #[parse(rename = " topmargin=")]
    Topmargin,
    #[parse(rename = " vlink=")]
    Vlink,
}
#[derive(Debug, Clone)]
pub enum BodyAttributes {
    Alink(crate::StringAttributeValue),
    Background(crate::StringAttributeValue),
    Bgcolor(crate::StringAttributeValue),
    Bottommargin(crate::StringAttributeValue),
    Leftmargin(crate::StringAttributeValue),
    Link(crate::StringAttributeValue),
    Onafterprint(crate::StringAttributeValue),
    Onbeforeprint(crate::StringAttributeValue),
    Onbeforeunload(crate::StringAttributeValue),
    Onblur(crate::StringAttributeValue),
    Onerror(crate::StringAttributeValue),
    Onfocus(crate::StringAttributeValue),
    Onhashchange(crate::StringAttributeValue),
    Onlanguagechange(crate::StringAttributeValue),
    Onload(crate::StringAttributeValue),
    Onmessage(crate::StringAttributeValue),
    Onoffline(crate::StringAttributeValue),
    Ononline(crate::StringAttributeValue),
    Onpagehide(crate::StringAttributeValue),
    Onpageshow(crate::StringAttributeValue),
    Onpopstate(crate::StringAttributeValue),
    Onredo(crate::StringAttributeValue),
    Onresize(crate::StringAttributeValue),
    Onstorage(crate::StringAttributeValue),
    Onundo(crate::StringAttributeValue),
    Onunload(crate::StringAttributeValue),
    Rightmargin(crate::StringAttributeValue),
    Text(crate::StringAttributeValue),
    Topmargin(crate::StringAttributeValue),
    Vlink(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for BodyAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(BodyAttributesName::new_parser()
                .then_lazy(|name| match name {
                    BodyAttributesName::Alink => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Alink)
                        .boxed(),
                    BodyAttributesName::Background => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Background)
                        .boxed(),
                    BodyAttributesName::Bgcolor => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Bgcolor)
                        .boxed(),
                    BodyAttributesName::Bottommargin => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Bottommargin)
                        .boxed(),
                    BodyAttributesName::Leftmargin => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Leftmargin)
                        .boxed(),
                    BodyAttributesName::Link => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Link)
                        .boxed(),
                    BodyAttributesName::Onafterprint => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onafterprint)
                        .boxed(),
                    BodyAttributesName::Onbeforeprint => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onbeforeprint)
                        .boxed(),
                    BodyAttributesName::Onbeforeunload => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onbeforeunload)
                        .boxed(),
                    BodyAttributesName::Onblur => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onblur)
                        .boxed(),
                    BodyAttributesName::Onerror => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onerror)
                        .boxed(),
                    BodyAttributesName::Onfocus => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onfocus)
                        .boxed(),
                    BodyAttributesName::Onhashchange => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onhashchange)
                        .boxed(),
                    BodyAttributesName::Onlanguagechange => {
                        crate::StringAttributeValue::new_parser()
                            .map_output(Self::Onlanguagechange)
                            .boxed()
                    }
                    BodyAttributesName::Onload => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onload)
                        .boxed(),
                    BodyAttributesName::Onmessage => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onmessage)
                        .boxed(),
                    BodyAttributesName::Onoffline => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onoffline)
                        .boxed(),
                    BodyAttributesName::Ononline => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Ononline)
                        .boxed(),
                    BodyAttributesName::Onpagehide => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onpagehide)
                        .boxed(),
                    BodyAttributesName::Onpageshow => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onpageshow)
                        .boxed(),
                    BodyAttributesName::Onpopstate => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onpopstate)
                        .boxed(),
                    BodyAttributesName::Onredo => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onredo)
                        .boxed(),
                    BodyAttributesName::Onresize => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onresize)
                        .boxed(),
                    BodyAttributesName::Onstorage => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onstorage)
                        .boxed(),
                    BodyAttributesName::Onundo => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onundo)
                        .boxed(),
                    BodyAttributesName::Onunload => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Onunload)
                        .boxed(),
                    BodyAttributesName::Rightmargin => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Rightmargin)
                        .boxed(),
                    BodyAttributesName::Text => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Text)
                        .boxed(),
                    BodyAttributesName::Topmargin => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Topmargin)
                        .boxed(),
                    BodyAttributesName::Vlink => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Vlink)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Body {
    attributes: Vec<BodyAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Body {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        BodyAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</body>")
            .map_output(|(attributes, body)| Body { attributes, body })
    }
}
