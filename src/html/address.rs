use kalosm_sample::*;
type AddressAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Address {
    attributes: Vec<AddressAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Address {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        AddressAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</address>")
            .map_output(|(attributes, body)| Address { attributes, body })
    }
}
