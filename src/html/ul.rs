use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum UlAttributesName {
    #[parse(rename = " compact=")]
    Compact,
}
#[derive(Debug, Clone)]
pub enum UlAttributes {
    Compact(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for UlAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(UlAttributesName::new_parser()
                .then_lazy(|name| match name {
                    UlAttributesName::Compact => {
                        String::new_parser().map_output(Self::Compact).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Ul {
    attributes: Vec<UlAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Ul {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        UlAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</ul>")
            .map_output(|(attributes, body)| Ul { attributes, body })
    }
}
