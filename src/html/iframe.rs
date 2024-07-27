use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum IframeAttributesName {
    #[parse(rename = " allow=")]
    Allow,
    #[parse(rename = " allowfullscreen=")]
    Allowfullscreen,
    #[parse(rename = " allowpaymentrequest=")]
    Allowpaymentrequest,
    #[parse(rename = " csp=")]
    Csp,
    #[parse(rename = " height=")]
    Height,
    #[parse(rename = " importance=")]
    Importance,
    #[parse(rename = " name=")]
    Name,
    #[parse(rename = " referrerpolicy=")]
    Referrerpolicy,
    #[parse(rename = " sandbox=")]
    Sandbox,
    #[parse(rename = " seamless=")]
    Seamless,
    #[parse(rename = " src=")]
    Src,
    #[parse(rename = " srcdoc=")]
    Srcdoc,
    #[parse(rename = " width=")]
    Width,
}
#[derive(Debug, Clone)]
pub enum IframeAttributes {
    Allow(crate::StringAttributeValue),
    Allowfullscreen(crate::StringAttributeValue),
    Allowpaymentrequest(crate::StringAttributeValue),
    Csp(crate::StringAttributeValue),
    Height(crate::StringAttributeValue),
    Importance(crate::StringAttributeValue),
    Name(crate::StringAttributeValue),
    Referrerpolicy(crate::StringAttributeValue),
    Sandbox(crate::SbValues),
    Seamless(crate::StringAttributeValue),
    Src(crate::StringAttributeValue),
    Srcdoc(crate::StringAttributeValue),
    Width(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for IframeAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(IframeAttributesName::new_parser()
                .then_lazy(|name| match name {
                    IframeAttributesName::Allow => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Allow)
                        .boxed(),
                    IframeAttributesName::Allowfullscreen => {
                        crate::StringAttributeValue::new_parser()
                            .map_output(Self::Allowfullscreen)
                            .boxed()
                    }
                    IframeAttributesName::Allowpaymentrequest => {
                        crate::StringAttributeValue::new_parser()
                            .map_output(Self::Allowpaymentrequest)
                            .boxed()
                    }
                    IframeAttributesName::Csp => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Csp)
                        .boxed(),
                    IframeAttributesName::Height => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Height)
                        .boxed(),
                    IframeAttributesName::Importance => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Importance)
                        .boxed(),
                    IframeAttributesName::Name => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Name)
                        .boxed(),
                    IframeAttributesName::Referrerpolicy => {
                        crate::StringAttributeValue::new_parser()
                            .map_output(Self::Referrerpolicy)
                            .boxed()
                    }
                    IframeAttributesName::Sandbox => crate::SbValues::new_parser()
                        .map_output(Self::Sandbox)
                        .boxed(),
                    IframeAttributesName::Seamless => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Seamless)
                        .boxed(),
                    IframeAttributesName::Src => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Src)
                        .boxed(),
                    IframeAttributesName::Srcdoc => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Srcdoc)
                        .boxed(),
                    IframeAttributesName::Width => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Width)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Iframe {
    attributes: Vec<IframeAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Iframe {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        IframeAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</iframe>")
            .map_output(|(attributes, body)| Iframe { attributes, body })
    }
}
