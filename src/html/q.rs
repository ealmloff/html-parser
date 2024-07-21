use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum QAttributesName {
    #[parse(rename = " cite=")]
    Cite,
}
#[derive(Debug, Clone)]
pub enum QAttributes {
    Cite(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for QAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(QAttributesName::new_parser()
                .then_lazy(|name| match name {
                    QAttributesName::Cite => String::new_parser().map_output(Self::Cite).boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Q {
    attributes: Vec<QAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Q {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        QAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</q>")
            .map_output(|(attributes, body)| Q { attributes, body })
    }
}
