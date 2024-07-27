use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum InputAttributesName {
    #[parse(rename = " accept=")]
    Accept,
    #[parse(rename = " alt=")]
    Alt,
    #[parse(rename = " autocomplete=")]
    Autocomplete,
    #[parse(rename = " autofocus=")]
    Autofocus,
    #[parse(rename = " checked=")]
    Checked,
    #[parse(rename = " dirname=")]
    Dirname,
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
    #[parse(rename = " height=")]
    Height,
    #[parse(rename = " inputmode=")]
    Inputmode,
    #[parse(rename = " list=")]
    List,
    #[parse(rename = " max=")]
    Max,
    #[parse(rename = " maxlength=")]
    Maxlength,
    #[parse(rename = " min=")]
    Min,
    #[parse(rename = " minlength=")]
    Minlength,
    #[parse(rename = " multiple=")]
    Multiple,
    #[parse(rename = " name=")]
    Name,
    #[parse(rename = " pattern=")]
    Pattern,
    #[parse(rename = " placeholder=")]
    Placeholder,
    #[parse(rename = " readonly=")]
    Readonly,
    #[parse(rename = " required=")]
    Required,
    #[parse(rename = " size=")]
    Size,
    #[parse(rename = " src=")]
    Src,
    #[parse(rename = " step=")]
    Step,
    #[parse(rename = " type=")]
    Type,
    #[parse(rename = " value=")]
    Value,
    #[parse(rename = " width=")]
    Width,
}
#[derive(Debug, Clone)]
pub enum InputAttributes {
    Accept(crate::StringAttributeValue),
    Alt(crate::StringAttributeValue),
    Autocomplete(crate::InputautocompleteValues),
    Autofocus(crate::StringAttributeValue),
    Checked(crate::StringAttributeValue),
    Dirname(crate::StringAttributeValue),
    Disabled(crate::StringAttributeValue),
    Form(crate::StringAttributeValue),
    Formaction(crate::StringAttributeValue),
    Formenctype(crate::EtValues),
    Formmethod(crate::FmValues),
    Formnovalidate(crate::StringAttributeValue),
    Formtarget(crate::StringAttributeValue),
    Height(crate::StringAttributeValue),
    Inputmode(crate::ImValues),
    List(crate::StringAttributeValue),
    Max(crate::StringAttributeValue),
    Maxlength(crate::StringAttributeValue),
    Min(crate::StringAttributeValue),
    Minlength(crate::StringAttributeValue),
    Multiple(crate::StringAttributeValue),
    Name(crate::StringAttributeValue),
    Pattern(crate::StringAttributeValue),
    Placeholder(crate::StringAttributeValue),
    Readonly(crate::StringAttributeValue),
    Required(crate::StringAttributeValue),
    Size(crate::StringAttributeValue),
    Src(crate::StringAttributeValue),
    Step(crate::StringAttributeValue),
    Type(crate::TValues),
    Value(crate::StringAttributeValue),
    Width(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for InputAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(InputAttributesName::new_parser()
                .then_lazy(|name| match name {
                    InputAttributesName::Accept => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Accept)
                        .boxed(),
                    InputAttributesName::Alt => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Alt)
                        .boxed(),
                    InputAttributesName::Autocomplete => {
                        crate::InputautocompleteValues::new_parser()
                            .map_output(Self::Autocomplete)
                            .boxed()
                    }
                    InputAttributesName::Autofocus => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Autofocus)
                        .boxed(),
                    InputAttributesName::Checked => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Checked)
                        .boxed(),
                    InputAttributesName::Dirname => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Dirname)
                        .boxed(),
                    InputAttributesName::Disabled => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Disabled)
                        .boxed(),
                    InputAttributesName::Form => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Form)
                        .boxed(),
                    InputAttributesName::Formaction => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Formaction)
                        .boxed(),
                    InputAttributesName::Formenctype => crate::EtValues::new_parser()
                        .map_output(Self::Formenctype)
                        .boxed(),
                    InputAttributesName::Formmethod => crate::FmValues::new_parser()
                        .map_output(Self::Formmethod)
                        .boxed(),
                    InputAttributesName::Formnovalidate => {
                        crate::StringAttributeValue::new_parser()
                            .map_output(Self::Formnovalidate)
                            .boxed()
                    }
                    InputAttributesName::Formtarget => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Formtarget)
                        .boxed(),
                    InputAttributesName::Height => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Height)
                        .boxed(),
                    InputAttributesName::Inputmode => crate::ImValues::new_parser()
                        .map_output(Self::Inputmode)
                        .boxed(),
                    InputAttributesName::List => crate::StringAttributeValue::new_parser()
                        .map_output(Self::List)
                        .boxed(),
                    InputAttributesName::Max => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Max)
                        .boxed(),
                    InputAttributesName::Maxlength => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Maxlength)
                        .boxed(),
                    InputAttributesName::Min => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Min)
                        .boxed(),
                    InputAttributesName::Minlength => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Minlength)
                        .boxed(),
                    InputAttributesName::Multiple => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Multiple)
                        .boxed(),
                    InputAttributesName::Name => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Name)
                        .boxed(),
                    InputAttributesName::Pattern => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Pattern)
                        .boxed(),
                    InputAttributesName::Placeholder => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Placeholder)
                        .boxed(),
                    InputAttributesName::Readonly => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Readonly)
                        .boxed(),
                    InputAttributesName::Required => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Required)
                        .boxed(),
                    InputAttributesName::Size => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Size)
                        .boxed(),
                    InputAttributesName::Src => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Src)
                        .boxed(),
                    InputAttributesName::Step => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Step)
                        .boxed(),
                    InputAttributesName::Type => {
                        crate::TValues::new_parser().map_output(Self::Type).boxed()
                    }
                    InputAttributesName::Value => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Value)
                        .boxed(),
                    InputAttributesName::Width => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Width)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Input {
    attributes: Vec<InputAttributes>,
}

impl kalosm_sample::Parse for Input {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        InputAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Input { attributes })
    }
}
