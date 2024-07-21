use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ButtonAttributesName {
    #[parse(rename = " autocomplete=")]
    Autocomplete,
    #[parse(rename = " autofocus=")]
    Autofocus,
    #[parse(rename = " disabled=")]
    Disabled,
    #[parse(rename = " form=")]
    Form,
    #[parse(rename = " formaction=")]
    Formaction,
    #[parse(rename = " formenctype=")]
    Formenctype,
    #[parse(rename = " formmethod=")]
    Formmethod,
    #[parse(rename = " formnovalidate=")]
    Formnovalidate,
    #[parse(rename = " formtarget=")]
    Formtarget,
    #[parse(rename = " name=")]
    Name,
    #[parse(rename = " type=")]
    Type,
    #[parse(rename = " value=")]
    Value,
}
#[derive(Debug, Clone)]
pub enum ButtonAttributes {
    Autocomplete(String),
    Autofocus(String),
    Disabled(String),
    Form(String),
    Formaction(String),
    Formenctype(crate::EtValues),
    Formmethod(crate::FmValues),
    Formnovalidate(String),
    Formtarget(String),
    Name(String),
    Type(crate::BtValues),
    Value(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for ButtonAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(ButtonAttributesName::new_parser()
                .then_lazy(|name| match name {
                    ButtonAttributesName::Autocomplete => {
                        String::new_parser().map_output(Self::Autocomplete).boxed()
                    }
                    ButtonAttributesName::Autofocus => {
                        String::new_parser().map_output(Self::Autofocus).boxed()
                    }
                    ButtonAttributesName::Disabled => {
                        String::new_parser().map_output(Self::Disabled).boxed()
                    }
                    ButtonAttributesName::Form => {
                        String::new_parser().map_output(Self::Form).boxed()
                    }
                    ButtonAttributesName::Formaction => {
                        String::new_parser().map_output(Self::Formaction).boxed()
                    }
                    ButtonAttributesName::Formenctype => crate::EtValues::new_parser()
                        .map_output(Self::Formenctype)
                        .boxed(),
                    ButtonAttributesName::Formmethod => crate::FmValues::new_parser()
                        .map_output(Self::Formmethod)
                        .boxed(),
                    ButtonAttributesName::Formnovalidate => String::new_parser()
                        .map_output(Self::Formnovalidate)
                        .boxed(),
                    ButtonAttributesName::Formtarget => {
                        String::new_parser().map_output(Self::Formtarget).boxed()
                    }
                    ButtonAttributesName::Name => {
                        String::new_parser().map_output(Self::Name).boxed()
                    }
                    ButtonAttributesName::Type => {
                        crate::BtValues::new_parser().map_output(Self::Type).boxed()
                    }
                    ButtonAttributesName::Value => {
                        String::new_parser().map_output(Self::Value).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Button {
    attributes: Vec<ButtonAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Button {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ButtonAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</button>")
            .map_output(|(attributes, body)| Button { attributes, body })
    }
}
