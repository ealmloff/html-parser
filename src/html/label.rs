use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum LabelAttributesName {
    #[parse(rename = " for=")]
    For,
    #[parse(rename = " form=")]
    Form,
}
#[derive(Debug, Clone)]
pub enum LabelAttributes {
    For(String),
    Form(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for LabelAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(LabelAttributesName::new_parser()
                .then_lazy(|name| match name {
                    LabelAttributesName::For => String::new_parser().map_output(Self::For).boxed(),
                    LabelAttributesName::Form => {
                        String::new_parser().map_output(Self::Form).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Label {
    attributes: Vec<LabelAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Label {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        LabelAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</label>")
            .map_output(|(attributes, body)| Label { attributes, body })
    }
}
