use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum StyleAttributesName {
    #[parse(rename = " media=")]
    Media,
    #[parse(rename = " nonce=")]
    Nonce,
    #[parse(rename = " scoped=")]
    Scoped,
    #[parse(rename = " title=")]
    Title,
    #[parse(rename = " type=")]
    Type,
}
#[derive(Debug, Clone)]
pub enum StyleAttributes {
    Media(String),
    Nonce(String),
    Scoped(String),
    Title(String),
    Type(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for StyleAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(StyleAttributesName::new_parser()
                .then_lazy(|name| match name {
                    StyleAttributesName::Media => {
                        String::new_parser().map_output(Self::Media).boxed()
                    }
                    StyleAttributesName::Nonce => {
                        String::new_parser().map_output(Self::Nonce).boxed()
                    }
                    StyleAttributesName::Scoped => {
                        String::new_parser().map_output(Self::Scoped).boxed()
                    }
                    StyleAttributesName::Title => {
                        String::new_parser().map_output(Self::Title).boxed()
                    }
                    StyleAttributesName::Type => {
                        String::new_parser().map_output(Self::Type).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Style {
    attributes: Vec<StyleAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Style {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        StyleAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</style>")
            .map_output(|(attributes, body)| Style { attributes, body })
    }
}
