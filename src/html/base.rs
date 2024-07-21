use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum BaseAttributesName {
    #[parse(rename = " href=")]
    Href,
    #[parse(rename = " target=")]
    Target,
}
#[derive(Debug, Clone)]
pub enum BaseAttributes {
    Href(String),
    Target(crate::TargetValues),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for BaseAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(BaseAttributesName::new_parser()
                .then_lazy(|name| match name {
                    BaseAttributesName::Href => String::new_parser().map_output(Self::Href).boxed(),
                    BaseAttributesName::Target => crate::TargetValues::new_parser()
                        .map_output(Self::Target)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Base {
    attributes: Vec<BaseAttributes>,
}

impl kalosm_sample::Parse for Base {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        BaseAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Base { attributes })
    }
}
