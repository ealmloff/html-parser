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
    Accept(String),
    Alt(String),
    Autocomplete(crate::InputautocompleteValues),
    Autofocus(String),
    Checked(String),
    Dirname(String),
    Disabled(String),
    Form(String),
    Formaction(String),
    Formenctype(crate::EtValues),
    Formmethod(crate::FmValues),
    Formnovalidate(String),
    Formtarget(String),
    Height(String),
    Inputmode(crate::ImValues),
    List(String),
    Max(String),
    Maxlength(String),
    Min(String),
    Minlength(String),
    Multiple(String),
    Name(String),
    Pattern(String),
    Placeholder(String),
    Readonly(String),
    Required(String),
    Size(String),
    Src(String),
    Step(String),
    Type(crate::TValues),
    Value(String),
    Width(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for InputAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(InputAttributesName::new_parser()
                .then_lazy(|name| match name {
                    InputAttributesName::Accept => {
                        String::new_parser().map_output(Self::Accept).boxed()
                    }
                    InputAttributesName::Alt => String::new_parser().map_output(Self::Alt).boxed(),
                    InputAttributesName::Autocomplete => {
                        crate::InputautocompleteValues::new_parser()
                            .map_output(Self::Autocomplete)
                            .boxed()
                    }
                    InputAttributesName::Autofocus => {
                        String::new_parser().map_output(Self::Autofocus).boxed()
                    }
                    InputAttributesName::Checked => {
                        String::new_parser().map_output(Self::Checked).boxed()
                    }
                    InputAttributesName::Dirname => {
                        String::new_parser().map_output(Self::Dirname).boxed()
                    }
                    InputAttributesName::Disabled => {
                        String::new_parser().map_output(Self::Disabled).boxed()
                    }
                    InputAttributesName::Form => {
                        String::new_parser().map_output(Self::Form).boxed()
                    }
                    InputAttributesName::Formaction => {
                        String::new_parser().map_output(Self::Formaction).boxed()
                    }
                    InputAttributesName::Formenctype => crate::EtValues::new_parser()
                        .map_output(Self::Formenctype)
                        .boxed(),
                    InputAttributesName::Formmethod => crate::FmValues::new_parser()
                        .map_output(Self::Formmethod)
                        .boxed(),
                    InputAttributesName::Formnovalidate => String::new_parser()
                        .map_output(Self::Formnovalidate)
                        .boxed(),
                    InputAttributesName::Formtarget => {
                        String::new_parser().map_output(Self::Formtarget).boxed()
                    }
                    InputAttributesName::Height => {
                        String::new_parser().map_output(Self::Height).boxed()
                    }
                    InputAttributesName::Inputmode => crate::ImValues::new_parser()
                        .map_output(Self::Inputmode)
                        .boxed(),
                    InputAttributesName::List => {
                        String::new_parser().map_output(Self::List).boxed()
                    }
                    InputAttributesName::Max => String::new_parser().map_output(Self::Max).boxed(),
                    InputAttributesName::Maxlength => {
                        String::new_parser().map_output(Self::Maxlength).boxed()
                    }
                    InputAttributesName::Min => String::new_parser().map_output(Self::Min).boxed(),
                    InputAttributesName::Minlength => {
                        String::new_parser().map_output(Self::Minlength).boxed()
                    }
                    InputAttributesName::Multiple => {
                        String::new_parser().map_output(Self::Multiple).boxed()
                    }
                    InputAttributesName::Name => {
                        String::new_parser().map_output(Self::Name).boxed()
                    }
                    InputAttributesName::Pattern => {
                        String::new_parser().map_output(Self::Pattern).boxed()
                    }
                    InputAttributesName::Placeholder => {
                        String::new_parser().map_output(Self::Placeholder).boxed()
                    }
                    InputAttributesName::Readonly => {
                        String::new_parser().map_output(Self::Readonly).boxed()
                    }
                    InputAttributesName::Required => {
                        String::new_parser().map_output(Self::Required).boxed()
                    }
                    InputAttributesName::Size => {
                        String::new_parser().map_output(Self::Size).boxed()
                    }
                    InputAttributesName::Src => String::new_parser().map_output(Self::Src).boxed(),
                    InputAttributesName::Step => {
                        String::new_parser().map_output(Self::Step).boxed()
                    }
                    InputAttributesName::Type => {
                        crate::TValues::new_parser().map_output(Self::Type).boxed()
                    }
                    InputAttributesName::Value => {
                        String::new_parser().map_output(Self::Value).boxed()
                    }
                    InputAttributesName::Width => {
                        String::new_parser().map_output(Self::Width).boxed()
                    }
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
