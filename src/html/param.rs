use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ParamAttributesName {
    #[parse(rename = " name=")]
    Name,
    #[parse(rename = " type=")]
    Type,
    #[parse(rename = " value=")]
    Value,
    #[parse(rename = " valuetype=")]
    Valuetype,
}
#[derive(Debug, Clone)]
pub enum ParamAttributes {
    Name(crate::StringAttributeValue),
    Type(crate::StringAttributeValue),
    Value(crate::StringAttributeValue),
    Valuetype(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for ParamAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(ParamAttributesName::new_parser()
                .then_lazy(|name| match name {
                    ParamAttributesName::Name => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Name)
                        .boxed(),
                    ParamAttributesName::Type => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Type)
                        .boxed(),
                    ParamAttributesName::Value => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Value)
                        .boxed(),
                    ParamAttributesName::Valuetype => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Valuetype)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Param {
    attributes: Vec<ParamAttributes>,
}

impl kalosm_sample::Parse for Param {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ParamAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Param { attributes })
    }
}
