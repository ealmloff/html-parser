use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum FormAttributesName {
    #[parse(rename = " accept=")]
    Accept,
    #[parse(rename = " accept-charset=")]
    AcceptCharset,
    #[parse(rename = " action=")]
    Action,
    #[parse(rename = " autocapitalize=")]
    Autocapitalize,
    #[parse(rename = " autocomplete=")]
    Autocomplete,
    #[parse(rename = " enctype=")]
    Enctype,
    #[parse(rename = " method=")]
    Method,
    #[parse(rename = " name=")]
    Name,
    #[parse(rename = " novalidate=")]
    Novalidate,
    #[parse(rename = " target=")]
    Target,
}
#[derive(Debug, Clone)]
pub enum FormAttributes {
    Accept(String),
    AcceptCharset(String),
    Action(String),
    Autocapitalize(String),
    Autocomplete(crate::OValues),
    Enctype(crate::EtValues),
    Method(crate::MValues),
    Name(String),
    Novalidate(String),
    Target(crate::TargetValues),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for FormAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(FormAttributesName::new_parser()
                .then_lazy(|name| match name {
                    FormAttributesName::Accept => {
                        String::new_parser().map_output(Self::Accept).boxed()
                    }
                    FormAttributesName::AcceptCharset => {
                        String::new_parser().map_output(Self::AcceptCharset).boxed()
                    }
                    FormAttributesName::Action => {
                        String::new_parser().map_output(Self::Action).boxed()
                    }
                    FormAttributesName::Autocapitalize => String::new_parser()
                        .map_output(Self::Autocapitalize)
                        .boxed(),
                    FormAttributesName::Autocomplete => crate::OValues::new_parser()
                        .map_output(Self::Autocomplete)
                        .boxed(),
                    FormAttributesName::Enctype => crate::EtValues::new_parser()
                        .map_output(Self::Enctype)
                        .boxed(),
                    FormAttributesName::Method => crate::MValues::new_parser()
                        .map_output(Self::Method)
                        .boxed(),
                    FormAttributesName::Name => String::new_parser().map_output(Self::Name).boxed(),
                    FormAttributesName::Novalidate => {
                        String::new_parser().map_output(Self::Novalidate).boxed()
                    }
                    FormAttributesName::Target => crate::TargetValues::new_parser()
                        .map_output(Self::Target)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Form {
    attributes: Vec<FormAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Form {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        FormAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</form>")
            .map_output(|(attributes, body)| Form { attributes, body })
    }
}
