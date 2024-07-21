use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum DdAttributesName {
    #[parse(rename = " nowrap=")]
    Nowrap,
}
#[derive(Debug, Clone)]
pub enum DdAttributes {
    Nowrap(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for DdAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(DdAttributesName::new_parser()
                .then_lazy(|name| match name {
                    DdAttributesName::Nowrap => {
                        String::new_parser().map_output(Self::Nowrap).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Dd {
    attributes: Vec<DdAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Dd {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        DdAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</dd>")
            .map_output(|(attributes, body)| Dd { attributes, body })
    }
}
