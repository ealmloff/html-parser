use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum SlotAttributesName {
    #[parse(rename = " name=")]
    Name,
}
#[derive(Debug, Clone)]
pub enum SlotAttributes {
    Name(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for SlotAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(SlotAttributesName::new_parser()
                .then_lazy(|name| match name {
                    SlotAttributesName::Name => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Name)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Slot {
    attributes: Vec<SlotAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Slot {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        SlotAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</slot>")
            .map_output(|(attributes, body)| Slot { attributes, body })
    }
}
