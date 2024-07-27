use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum HtmlAttributesName {
    #[parse(rename = " manifest=")]
    Manifest,
    #[parse(rename = " version=")]
    Version,
    #[parse(rename = " xmlns=")]
    Xmlns,
}
#[derive(Debug, Clone)]
pub enum HtmlAttributes {
    Manifest(crate::StringAttributeValue),
    Version(crate::StringAttributeValue),
    Xmlns(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for HtmlAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(HtmlAttributesName::new_parser()
                .then_lazy(|name| match name {
                    HtmlAttributesName::Manifest => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Manifest)
                        .boxed(),
                    HtmlAttributesName::Version => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Version)
                        .boxed(),
                    HtmlAttributesName::Xmlns => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Xmlns)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Html {
    attributes: Vec<HtmlAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Html {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        HtmlAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</html>")
            .map_output(|(attributes, body)| Html { attributes, body })
    }
}
