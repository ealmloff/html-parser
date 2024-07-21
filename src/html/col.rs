use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ColAttributesName {
    #[parse(rename = " align=")]
    Align,
    #[parse(rename = " span=")]
    Span,
}
#[derive(Debug, Clone)]
pub enum ColAttributes {
    Align(String),
    Span(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for ColAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(ColAttributesName::new_parser()
                .then_lazy(|name| match name {
                    ColAttributesName::Align => {
                        String::new_parser().map_output(Self::Align).boxed()
                    }
                    ColAttributesName::Span => String::new_parser().map_output(Self::Span).boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Col {
    attributes: Vec<ColAttributes>,
}

impl kalosm_sample::Parse for Col {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ColAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Col { attributes })
    }
}
