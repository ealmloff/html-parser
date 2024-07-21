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
    Alink(String),
    Background(String),
    Bgcolor(String),
    Bottommargin(String),
    Leftmargin(String),
    Link(String),
    Onafterprint(String),
    Onbeforeprint(String),
    Onbeforeunload(String),
    Onblur(String),
    Onerror(String),
    Onfocus(String),
    Onhashchange(String),
    Onlanguagechange(String),
    Onload(String),
    Onmessage(String),
    Onoffline(String),
    Ononline(String),
    Onpagehide(String),
    Onpageshow(String),
    Onpopstate(String),
    Onredo(String),
    Onresize(String),
    Onstorage(String),
    Onundo(String),
    Onunload(String),
    Rightmargin(String),
    Text(String),
    Topmargin(String),
    Vlink(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for BodyAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(BodyAttributesName::new_parser()
                .then_lazy(|name| match name {
                    BodyAttributesName::Alink => {
                        String::new_parser().map_output(Self::Alink).boxed()
                    }
                    BodyAttributesName::Background => {
                        String::new_parser().map_output(Self::Background).boxed()
                    }
                    BodyAttributesName::Bgcolor => {
                        String::new_parser().map_output(Self::Bgcolor).boxed()
                    }
                    BodyAttributesName::Bottommargin => {
                        String::new_parser().map_output(Self::Bottommargin).boxed()
                    }
                    BodyAttributesName::Leftmargin => {
                        String::new_parser().map_output(Self::Leftmargin).boxed()
                    }
                    BodyAttributesName::Link => String::new_parser().map_output(Self::Link).boxed(),
                    BodyAttributesName::Onafterprint => {
                        String::new_parser().map_output(Self::Onafterprint).boxed()
                    }
                    BodyAttributesName::Onbeforeprint => {
                        String::new_parser().map_output(Self::Onbeforeprint).boxed()
                    }
                    BodyAttributesName::Onbeforeunload => String::new_parser()
                        .map_output(Self::Onbeforeunload)
                        .boxed(),
                    BodyAttributesName::Onblur => {
                        String::new_parser().map_output(Self::Onblur).boxed()
                    }
                    BodyAttributesName::Onerror => {
                        String::new_parser().map_output(Self::Onerror).boxed()
                    }
                    BodyAttributesName::Onfocus => {
                        String::new_parser().map_output(Self::Onfocus).boxed()
                    }
                    BodyAttributesName::Onhashchange => {
                        String::new_parser().map_output(Self::Onhashchange).boxed()
                    }
                    BodyAttributesName::Onlanguagechange => String::new_parser()
                        .map_output(Self::Onlanguagechange)
                        .boxed(),
                    BodyAttributesName::Onload => {
                        String::new_parser().map_output(Self::Onload).boxed()
                    }
                    BodyAttributesName::Onmessage => {
                        String::new_parser().map_output(Self::Onmessage).boxed()
                    }
                    BodyAttributesName::Onoffline => {
                        String::new_parser().map_output(Self::Onoffline).boxed()
                    }
                    BodyAttributesName::Ononline => {
                        String::new_parser().map_output(Self::Ononline).boxed()
                    }
                    BodyAttributesName::Onpagehide => {
                        String::new_parser().map_output(Self::Onpagehide).boxed()
                    }
                    BodyAttributesName::Onpageshow => {
                        String::new_parser().map_output(Self::Onpageshow).boxed()
                    }
                    BodyAttributesName::Onpopstate => {
                        String::new_parser().map_output(Self::Onpopstate).boxed()
                    }
                    BodyAttributesName::Onredo => {
                        String::new_parser().map_output(Self::Onredo).boxed()
                    }
                    BodyAttributesName::Onresize => {
                        String::new_parser().map_output(Self::Onresize).boxed()
                    }
                    BodyAttributesName::Onstorage => {
                        String::new_parser().map_output(Self::Onstorage).boxed()
                    }
                    BodyAttributesName::Onundo => {
                        String::new_parser().map_output(Self::Onundo).boxed()
                    }
                    BodyAttributesName::Onunload => {
                        String::new_parser().map_output(Self::Onunload).boxed()
                    }
                    BodyAttributesName::Rightmargin => {
                        String::new_parser().map_output(Self::Rightmargin).boxed()
                    }
                    BodyAttributesName::Text => String::new_parser().map_output(Self::Text).boxed(),
                    BodyAttributesName::Topmargin => {
                        String::new_parser().map_output(Self::Topmargin).boxed()
                    }
                    BodyAttributesName::Vlink => {
                        String::new_parser().map_output(Self::Vlink).boxed()
                    }
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
