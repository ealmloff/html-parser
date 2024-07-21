use kalosm_sample::*;
type SectionAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Section {
    attributes: Vec<SectionAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Section {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        SectionAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</section>")
            .map_output(|(attributes, body)| Section { attributes, body })
    }
}
