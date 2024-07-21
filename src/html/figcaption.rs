use kalosm_sample::*;
type FigcaptionAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Figcaption {
    attributes: Vec<FigcaptionAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Figcaption {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        FigcaptionAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</figcaption>")
            .map_output(|(attributes, body)| Figcaption { attributes, body })
    }
}
