use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ProgressAttributesName {
    #[parse(rename = " max=")]
    Max,
    #[parse(rename = " value=")]
    Value,
}
#[derive(Debug, Clone)]
pub enum ProgressAttributes {
    Max(crate::StringAttributeValue),
    Value(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for ProgressAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(ProgressAttributesName::new_parser()
                .then_lazy(|name| match name {
                    ProgressAttributesName::Max => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Max)
                        .boxed(),
                    ProgressAttributesName::Value => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Value)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Progress {
    attributes: Vec<ProgressAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Progress {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ProgressAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</progress>")
            .map_output(|(attributes, body)| Progress { attributes, body })
    }
}
