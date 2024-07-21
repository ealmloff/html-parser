use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum OptionAttributesName {
    #[parse(rename = " disabled=")]
    Disabled,
    #[parse(rename = " label=")]
    Label,
    #[parse(rename = " selected=")]
    Selected,
    #[parse(rename = " value=")]
    Value,
}
#[derive(Debug, Clone)]
pub enum OptionAttributes {
    Disabled(String),
    Label(String),
    Selected(String),
    Value(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for OptionAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(OptionAttributesName::new_parser()
                .then_lazy(|name| match name {
                    OptionAttributesName::Disabled => {
                        String::new_parser().map_output(Self::Disabled).boxed()
                    }
                    OptionAttributesName::Label => {
                        String::new_parser().map_output(Self::Label).boxed()
                    }
                    OptionAttributesName::Selected => {
                        String::new_parser().map_output(Self::Selected).boxed()
                    }
                    OptionAttributesName::Value => {
                        String::new_parser().map_output(Self::Value).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Option {
    attributes: Vec<OptionAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Option {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        OptionAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</option>")
            .map_output(|(attributes, body)| Option { attributes, body })
    }
}
