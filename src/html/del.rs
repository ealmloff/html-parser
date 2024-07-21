use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum DelAttributesName {
    #[parse(rename = " cite=")]
    Cite,
    #[parse(rename = " datetime=")]
    Datetime,
}
#[derive(Debug, Clone)]
pub enum DelAttributes {
    Cite(String),
    Datetime(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for DelAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(DelAttributesName::new_parser()
                .then_lazy(|name| match name {
                    DelAttributesName::Cite => String::new_parser().map_output(Self::Cite).boxed(),
                    DelAttributesName::Datetime => {
                        String::new_parser().map_output(Self::Datetime).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Del {
    attributes: Vec<DelAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Del {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        DelAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</del>")
            .map_output(|(attributes, body)| Del { attributes, body })
    }
}
