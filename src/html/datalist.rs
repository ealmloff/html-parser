use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum DatalistAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for DatalistAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Datalist {
    attributes: Vec<DatalistAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Datalist {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        DatalistAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</datalist>")
            .map_output(|(attributes, body)| Datalist { attributes, body })
    }
}
