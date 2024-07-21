use kalosm_sample::*;
#[derive(Debug, Clone)]
pub enum TemplateAttributes {
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for TemplateAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser().map_output(Self::GlobalAttribute)
    }
}
#[derive(Debug, Clone)]
pub struct Template {
    attributes: Vec<TemplateAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Template {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        TemplateAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</template>")
            .map_output(|(attributes, body)| Template { attributes, body })
    }
}
