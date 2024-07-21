use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum CaptionAttributesName {
    #[parse(rename = " align=")]
    Align,
}
#[derive(Debug, Clone)]
pub enum CaptionAttributes {
    Align(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for CaptionAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(CaptionAttributesName::new_parser()
                .then_lazy(|name| match name {
                    CaptionAttributesName::Align => {
                        String::new_parser().map_output(Self::Align).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Caption {
    attributes: Vec<CaptionAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Caption {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        CaptionAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</caption>")
            .map_output(|(attributes, body)| Caption { attributes, body })
    }
}
