use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum LiAttributesName {
    #[parse(rename = " type=")]
    Type,
    #[parse(rename = " value=")]
    Value,
}
#[derive(Debug, Clone)]
pub enum LiAttributes {
    Type(String),
    Value(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for LiAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(LiAttributesName::new_parser()
                .then_lazy(|name| match name {
                    LiAttributesName::Type => String::new_parser().map_output(Self::Type).boxed(),
                    LiAttributesName::Value => String::new_parser().map_output(Self::Value).boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Li {
    attributes: Vec<LiAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Li {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        LiAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</li>")
            .map_output(|(attributes, body)| Li { attributes, body })
    }
}
