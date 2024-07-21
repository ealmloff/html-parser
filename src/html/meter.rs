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
    Form(String),
    High(String),
    Low(String),
    Max(String),
    Min(String),
    Optimum(String),
    Value(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for MeterAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(MeterAttributesName::new_parser()
                .then_lazy(|name| match name {
                    MeterAttributesName::Form => {
                        String::new_parser().map_output(Self::Form).boxed()
                    }
                    MeterAttributesName::High => {
                        String::new_parser().map_output(Self::High).boxed()
                    }
                    MeterAttributesName::Low => String::new_parser().map_output(Self::Low).boxed(),
                    MeterAttributesName::Max => String::new_parser().map_output(Self::Max).boxed(),
                    MeterAttributesName::Min => String::new_parser().map_output(Self::Min).boxed(),
                    MeterAttributesName::Optimum => {
                        String::new_parser().map_output(Self::Optimum).boxed()
                    }
                    MeterAttributesName::Value => {
                        String::new_parser().map_output(Self::Value).boxed()
                    }
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
