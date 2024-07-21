use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum SelectAttributesName {
    #[parse(rename = " autocomplete=")]
    Autocomplete,
    #[parse(rename = " autofocus=")]
    Autofocus,
    #[parse(rename = " disabled=")]
    Disabled,
    #[parse(rename = " form=")]
    Form,
    #[parse(rename = " multiple=")]
    Multiple,
    #[parse(rename = " name=")]
    Name,
    #[parse(rename = " required=")]
    Required,
    #[parse(rename = " size=")]
    Size,
}
#[derive(Debug, Clone)]
pub enum SelectAttributes {
    Autocomplete(crate::InputautocompleteValues),
    Autofocus(String),
    Disabled(String),
    Form(String),
    Multiple(String),
    Name(String),
    Required(String),
    Size(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for SelectAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(SelectAttributesName::new_parser()
                .then_lazy(|name| match name {
                    SelectAttributesName::Autocomplete => {
                        crate::InputautocompleteValues::new_parser()
                            .map_output(Self::Autocomplete)
                            .boxed()
                    }
                    SelectAttributesName::Autofocus => {
                        String::new_parser().map_output(Self::Autofocus).boxed()
                    }
                    SelectAttributesName::Disabled => {
                        String::new_parser().map_output(Self::Disabled).boxed()
                    }
                    SelectAttributesName::Form => {
                        String::new_parser().map_output(Self::Form).boxed()
                    }
                    SelectAttributesName::Multiple => {
                        String::new_parser().map_output(Self::Multiple).boxed()
                    }
                    SelectAttributesName::Name => {
                        String::new_parser().map_output(Self::Name).boxed()
                    }
                    SelectAttributesName::Required => {
                        String::new_parser().map_output(Self::Required).boxed()
                    }
                    SelectAttributesName::Size => {
                        String::new_parser().map_output(Self::Size).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Select {
    attributes: Vec<SelectAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Select {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        SelectAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</select>")
            .map_output(|(attributes, body)| Select { attributes, body })
    }
}
