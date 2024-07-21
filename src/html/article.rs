use kalosm_sample::*;
type ArticleAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Article {
    attributes: Vec<ArticleAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Article {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ArticleAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</article>")
            .map_output(|(attributes, body)| Article { attributes, body })
    }
}
