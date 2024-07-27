use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum HeadAttributesName {
    #[parse(rename = " profile=")]
    Profile,
}
#[derive(Debug, Clone)]
pub enum HeadAttributes {
    Profile(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for HeadAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(HeadAttributesName::new_parser()
                .then_lazy(|name| match name {
                    HeadAttributesName::Profile => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Profile)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Head {
    attributes: Vec<HeadAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Head {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        HeadAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</head>")
            .map_output(|(attributes, body)| Head { attributes, body })
    }
}
