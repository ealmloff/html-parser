use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum FieldsetAttributesName {
    #[parse(rename = " disabled=")]
    Disabled,
    #[parse(rename = " form=")]
    Form,
    #[parse(rename = " name=")]
    Name,
}
#[derive(Debug, Clone)]
pub enum FieldsetAttributes {
    Disabled(String),
    Form(String),
    Name(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for FieldsetAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(FieldsetAttributesName::new_parser()
                .then_lazy(|name| match name {
                    FieldsetAttributesName::Disabled => {
                        String::new_parser().map_output(Self::Disabled).boxed()
                    }
                    FieldsetAttributesName::Form => {
                        String::new_parser().map_output(Self::Form).boxed()
                    }
                    FieldsetAttributesName::Name => {
                        String::new_parser().map_output(Self::Name).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Fieldset {
    attributes: Vec<FieldsetAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Fieldset {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        FieldsetAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</fieldset>")
            .map_output(|(attributes, body)| Fieldset { attributes, body })
    }
}
