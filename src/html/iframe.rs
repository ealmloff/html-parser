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
    Allow(String),
    Allowfullscreen(String),
    Allowpaymentrequest(String),
    Csp(String),
    Height(String),
    Importance(String),
    Name(String),
    Referrerpolicy(String),
    Sandbox(crate::SbValues),
    Seamless(String),
    Src(String),
    Srcdoc(String),
    Width(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for IframeAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(IframeAttributesName::new_parser()
                .then_lazy(|name| match name {
                    IframeAttributesName::Allow => {
                        String::new_parser().map_output(Self::Allow).boxed()
                    }
                    IframeAttributesName::Allowfullscreen => String::new_parser()
                        .map_output(Self::Allowfullscreen)
                        .boxed(),
                    IframeAttributesName::Allowpaymentrequest => String::new_parser()
                        .map_output(Self::Allowpaymentrequest)
                        .boxed(),
                    IframeAttributesName::Csp => String::new_parser().map_output(Self::Csp).boxed(),
                    IframeAttributesName::Height => {
                        String::new_parser().map_output(Self::Height).boxed()
                    }
                    IframeAttributesName::Importance => {
                        String::new_parser().map_output(Self::Importance).boxed()
                    }
                    IframeAttributesName::Name => {
                        String::new_parser().map_output(Self::Name).boxed()
                    }
                    IframeAttributesName::Referrerpolicy => String::new_parser()
                        .map_output(Self::Referrerpolicy)
                        .boxed(),
                    IframeAttributesName::Sandbox => crate::SbValues::new_parser()
                        .map_output(Self::Sandbox)
                        .boxed(),
                    IframeAttributesName::Seamless => {
                        String::new_parser().map_output(Self::Seamless).boxed()
                    }
                    IframeAttributesName::Src => String::new_parser().map_output(Self::Src).boxed(),
                    IframeAttributesName::Srcdoc => {
                        String::new_parser().map_output(Self::Srcdoc).boxed()
                    }
                    IframeAttributesName::Width => {
                        String::new_parser().map_output(Self::Width).boxed()
                    }
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
