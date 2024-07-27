use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum MapAttributesName {
    #[parse(rename = " name=")]
    Name,
}
#[derive(Debug, Clone)]
pub enum MapAttributes {
    Name(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for MapAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(MapAttributesName::new_parser()
                .then_lazy(|name| match name {
                    MapAttributesName::Name => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Name)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Map {
    attributes: Vec<MapAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Map {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        MapAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</map>")
            .map_output(|(attributes, body)| Map { attributes, body })
    }
}
