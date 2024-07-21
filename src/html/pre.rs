use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum PreAttributesName {
    #[parse(rename = " cols=")]
    Cols,
    #[parse(rename = " width=")]
    Width,
    #[parse(rename = " wrap=")]
    Wrap,
}
#[derive(Debug, Clone)]
pub enum PreAttributes {
    Cols(String),
    Width(String),
    Wrap(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for PreAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(PreAttributesName::new_parser()
                .then_lazy(|name| match name {
                    PreAttributesName::Cols => String::new_parser().map_output(Self::Cols).boxed(),
                    PreAttributesName::Width => {
                        String::new_parser().map_output(Self::Width).boxed()
                    }
                    PreAttributesName::Wrap => String::new_parser().map_output(Self::Wrap).boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Pre {
    attributes: Vec<PreAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Pre {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        PreAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</pre>")
            .map_output(|(attributes, body)| Pre { attributes, body })
    }
}
