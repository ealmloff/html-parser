use kalosm_sample::*;
type HgroupAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Hgroup {
    attributes: Vec<HgroupAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Hgroup {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        HgroupAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</hgroup>")
            .map_output(|(attributes, body)| Hgroup { attributes, body })
    }
}
