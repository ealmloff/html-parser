use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum DialogAttributesName {
    #[parse(rename = " open=")]
    Open,
}
#[derive(Debug, Clone)]
pub enum DialogAttributes {
    Open(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for DialogAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(DialogAttributesName::new_parser()
                .then_lazy(|name| match name {
                    DialogAttributesName::Open => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Open)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Dialog {
    attributes: Vec<DialogAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Dialog {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        DialogAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</dialog>")
            .map_output(|(attributes, body)| Dialog { attributes, body })
    }
}
