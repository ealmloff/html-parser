use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum BrAttributesName {
    #[parse(rename = " clear=")]
    Clear,
}
#[derive(Debug, Clone)]
pub enum BrAttributes {
    Clear(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for BrAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(BrAttributesName::new_parser()
                .then_lazy(|name| match name {
                    BrAttributesName::Clear => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Clear)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Br {
    attributes: Vec<BrAttributes>,
}

impl kalosm_sample::Parse for Br {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        BrAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Br { attributes })
    }
}
