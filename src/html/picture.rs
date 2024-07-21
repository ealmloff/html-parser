use kalosm_sample::*;
type PictureAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Picture {
    attributes: Vec<PictureAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Picture {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        PictureAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</picture>")
            .map_output(|(attributes, body)| Picture { attributes, body })
    }
}
