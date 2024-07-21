use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum TextareaAttributesName {
    #[parse(rename = " autocapitalize=")]
    Autocapitalize,
    #[parse(rename = " autocomplete=")]
    Autocomplete,
    #[parse(rename = " autofocus=")]
    Autofocus,
    #[parse(rename = " cols=")]
    Cols,
    #[parse(rename = " dirname=")]
    Dirname,
    #[parse(rename = " disabled=")]
    Disabled,
    #[parse(rename = " form=")]
    Form,
    #[parse(rename = " inputmode=")]
    Inputmode,
    #[parse(rename = " maxlength=")]
    Maxlength,
    #[parse(rename = " minlength=")]
    Minlength,
    #[parse(rename = " name=")]
    Name,
    #[parse(rename = " placeholder=")]
    Placeholder,
    #[parse(rename = " readonly=")]
    Readonly,
    #[parse(rename = " required=")]
    Required,
    #[parse(rename = " rows=")]
    Rows,
    #[parse(rename = " spellcheck=")]
    Spellcheck,
    #[parse(rename = " wrap=")]
    Wrap,
}
#[derive(Debug, Clone)]
pub enum TextareaAttributes {
    Autocapitalize(String),
    Autocomplete(crate::InputautocompleteValues),
    Autofocus(String),
    Cols(String),
    Dirname(String),
    Disabled(String),
    Form(String),
    Inputmode(crate::ImValues),
    Maxlength(String),
    Minlength(String),
    Name(String),
    Placeholder(String),
    Readonly(String),
    Required(String),
    Rows(String),
    Spellcheck(String),
    Wrap(crate::WValues),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for TextareaAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(TextareaAttributesName::new_parser()
                .then_lazy(|name| match name {
                    TextareaAttributesName::Autocapitalize => String::new_parser()
                        .map_output(Self::Autocapitalize)
                        .boxed(),
                    TextareaAttributesName::Autocomplete => {
                        crate::InputautocompleteValues::new_parser()
                            .map_output(Self::Autocomplete)
                            .boxed()
                    }
                    TextareaAttributesName::Autofocus => {
                        String::new_parser().map_output(Self::Autofocus).boxed()
                    }
                    TextareaAttributesName::Cols => {
                        String::new_parser().map_output(Self::Cols).boxed()
                    }
                    TextareaAttributesName::Dirname => {
                        String::new_parser().map_output(Self::Dirname).boxed()
                    }
                    TextareaAttributesName::Disabled => {
                        String::new_parser().map_output(Self::Disabled).boxed()
                    }
                    TextareaAttributesName::Form => {
                        String::new_parser().map_output(Self::Form).boxed()
                    }
                    TextareaAttributesName::Inputmode => crate::ImValues::new_parser()
                        .map_output(Self::Inputmode)
                        .boxed(),
                    TextareaAttributesName::Maxlength => {
                        String::new_parser().map_output(Self::Maxlength).boxed()
                    }
                    TextareaAttributesName::Minlength => {
                        String::new_parser().map_output(Self::Minlength).boxed()
                    }
                    TextareaAttributesName::Name => {
                        String::new_parser().map_output(Self::Name).boxed()
                    }
                    TextareaAttributesName::Placeholder => {
                        String::new_parser().map_output(Self::Placeholder).boxed()
                    }
                    TextareaAttributesName::Readonly => {
                        String::new_parser().map_output(Self::Readonly).boxed()
                    }
                    TextareaAttributesName::Required => {
                        String::new_parser().map_output(Self::Required).boxed()
                    }
                    TextareaAttributesName::Rows => {
                        String::new_parser().map_output(Self::Rows).boxed()
                    }
                    TextareaAttributesName::Spellcheck => {
                        String::new_parser().map_output(Self::Spellcheck).boxed()
                    }
                    TextareaAttributesName::Wrap => {
                        crate::WValues::new_parser().map_output(Self::Wrap).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Textarea {
    attributes: Vec<TextareaAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Textarea {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        TextareaAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</textarea>")
            .map_output(|(attributes, body)| Textarea { attributes, body })
    }
}
