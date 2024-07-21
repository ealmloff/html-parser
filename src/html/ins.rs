use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum InsAttributesName {
    #[parse(rename = " cite=")]
    Cite,
    #[parse(rename = " datetime=")]
    Datetime,
}
#[derive(Debug, Clone)]
pub enum InsAttributes {
    Cite(String),
    Datetime(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for InsAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(InsAttributesName::new_parser()
                .then_lazy(|name| match name {
                    InsAttributesName::Cite => String::new_parser().map_output(Self::Cite).boxed(),
                    InsAttributesName::Datetime => {
                        String::new_parser().map_output(Self::Datetime).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Ins {
    attributes: Vec<InsAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Ins {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        InsAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</ins>")
            .map_output(|(attributes, body)| Ins { attributes, body })
    }
}
