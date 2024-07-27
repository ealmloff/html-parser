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
    Autocomplete(crate::StringAttributeValue),
    Autofocus(crate::StringAttributeValue),
    Disabled(crate::StringAttributeValue),
    Form(crate::StringAttributeValue),
    Formaction(crate::StringAttributeValue),
    Formenctype(crate::EtValues),
    Formmethod(crate::FmValues),
    Formnovalidate(crate::StringAttributeValue),
    Formtarget(crate::StringAttributeValue),
    Name(crate::StringAttributeValue),
    Type(crate::BtValues),
    Value(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for ButtonAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(ButtonAttributesName::new_parser()
                .then_lazy(|name| match name {
                    ButtonAttributesName::Autocomplete => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Autocomplete)
                        .boxed(),
                    ButtonAttributesName::Autofocus => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Autofocus)
                        .boxed(),
                    ButtonAttributesName::Disabled => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Disabled)
                        .boxed(),
                    ButtonAttributesName::Form => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Form)
                        .boxed(),
                    ButtonAttributesName::Formaction => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Formaction)
                        .boxed(),
                    ButtonAttributesName::Formenctype => crate::EtValues::new_parser()
                        .map_output(Self::Formenctype)
                        .boxed(),
                    ButtonAttributesName::Formmethod => crate::FmValues::new_parser()
                        .map_output(Self::Formmethod)
                        .boxed(),
                    ButtonAttributesName::Formnovalidate => {
                        crate::StringAttributeValue::new_parser()
                            .map_output(Self::Formnovalidate)
                            .boxed()
                    }
                    ButtonAttributesName::Formtarget => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Formtarget)
                        .boxed(),
                    ButtonAttributesName::Name => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Name)
                        .boxed(),
                    ButtonAttributesName::Type => {
                        crate::BtValues::new_parser().map_output(Self::Type).boxed()
                    }
                    ButtonAttributesName::Value => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Value)
                        .boxed(),
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
