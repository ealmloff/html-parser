use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum CanvasAttributesName {
    #[parse(rename = " height=")]
    Height,
    #[parse(rename = " moz-opaque=")]
    MozOpaque,
    #[parse(rename = " width=")]
    Width,
}
#[derive(Debug, Clone)]
pub enum CanvasAttributes {
    Height(String),
    MozOpaque(String),
    Width(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for CanvasAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(CanvasAttributesName::new_parser()
                .then_lazy(|name| match name {
                    CanvasAttributesName::Height => {
                        String::new_parser().map_output(Self::Height).boxed()
                    }
                    CanvasAttributesName::MozOpaque => {
                        String::new_parser().map_output(Self::MozOpaque).boxed()
                    }
                    CanvasAttributesName::Width => {
                        String::new_parser().map_output(Self::Width).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Canvas {
    attributes: Vec<CanvasAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Canvas {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        CanvasAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</canvas>")
            .map_output(|(attributes, body)| Canvas { attributes, body })
    }
}
