use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum DetailsAttributesName {
    #[parse(rename = " open=")]
    Open,
}
#[derive(Debug, Clone)]
pub enum DetailsAttributes {
    Open(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for DetailsAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(DetailsAttributesName::new_parser()
                .then_lazy(|name| match name {
                    DetailsAttributesName::Open => {
                        String::new_parser().map_output(Self::Open).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Details {
    attributes: Vec<DetailsAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Details {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        DetailsAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</details>")
            .map_output(|(attributes, body)| Details { attributes, body })
    }
}
