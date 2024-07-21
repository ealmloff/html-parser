use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum TimeAttributesName {
    #[parse(rename = " datetime=")]
    Datetime,
}
#[derive(Debug, Clone)]
pub enum TimeAttributes {
    Datetime(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for TimeAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(TimeAttributesName::new_parser()
                .then_lazy(|name| match name {
                    TimeAttributesName::Datetime => {
                        String::new_parser().map_output(Self::Datetime).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Time {
    attributes: Vec<TimeAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Time {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        TimeAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</time>")
            .map_output(|(attributes, body)| Time { attributes, body })
    }
}
