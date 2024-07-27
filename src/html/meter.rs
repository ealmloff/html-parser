use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum MeterAttributesName {
    #[parse(rename = " form=")]
    Form,
    #[parse(rename = " high=")]
    High,
    #[parse(rename = " low=")]
    Low,
    #[parse(rename = " max=")]
    Max,
    #[parse(rename = " min=")]
    Min,
    #[parse(rename = " optimum=")]
    Optimum,
    #[parse(rename = " value=")]
    Value,
}
#[derive(Debug, Clone)]
pub enum MeterAttributes {
    Form(crate::StringAttributeValue),
    High(crate::StringAttributeValue),
    Low(crate::StringAttributeValue),
    Max(crate::StringAttributeValue),
    Min(crate::StringAttributeValue),
    Optimum(crate::StringAttributeValue),
    Value(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for MeterAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(MeterAttributesName::new_parser()
                .then_lazy(|name| match name {
                    MeterAttributesName::Form => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Form)
                        .boxed(),
                    MeterAttributesName::High => crate::StringAttributeValue::new_parser()
                        .map_output(Self::High)
                        .boxed(),
                    MeterAttributesName::Low => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Low)
                        .boxed(),
                    MeterAttributesName::Max => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Max)
                        .boxed(),
                    MeterAttributesName::Min => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Min)
                        .boxed(),
                    MeterAttributesName::Optimum => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Optimum)
                        .boxed(),
                    MeterAttributesName::Value => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Value)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Meter {
    attributes: Vec<MeterAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Meter {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        MeterAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</meter>")
            .map_output(|(attributes, body)| Meter { attributes, body })
    }
}
