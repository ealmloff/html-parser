use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum OutputAttributesName {
    #[parse(rename = " for=")]
    For,
    #[parse(rename = " form=")]
    Form,
    #[parse(rename = " name=")]
    Name,
}
#[derive(Debug, Clone)]
pub enum OutputAttributes {
    For(String),
    Form(String),
    Name(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for OutputAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(OutputAttributesName::new_parser()
                .then_lazy(|name| match name {
                    OutputAttributesName::For => String::new_parser().map_output(Self::For).boxed(),
                    OutputAttributesName::Form => {
                        String::new_parser().map_output(Self::Form).boxed()
                    }
                    OutputAttributesName::Name => {
                        String::new_parser().map_output(Self::Name).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Output {
    attributes: Vec<OutputAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Output {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        OutputAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</output>")
            .map_output(|(attributes, body)| Output { attributes, body })
    }
}
