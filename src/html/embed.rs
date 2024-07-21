use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum EmbedAttributesName {
    #[parse(rename = " height=")]
    Height,
    #[parse(rename = " src=")]
    Src,
    #[parse(rename = " type=")]
    Type,
    #[parse(rename = " width=")]
    Width,
}
#[derive(Debug, Clone)]
pub enum EmbedAttributes {
    Height(String),
    Src(String),
    Type(String),
    Width(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for EmbedAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(EmbedAttributesName::new_parser()
                .then_lazy(|name| match name {
                    EmbedAttributesName::Height => {
                        String::new_parser().map_output(Self::Height).boxed()
                    }
                    EmbedAttributesName::Src => String::new_parser().map_output(Self::Src).boxed(),
                    EmbedAttributesName::Type => {
                        String::new_parser().map_output(Self::Type).boxed()
                    }
                    EmbedAttributesName::Width => {
                        String::new_parser().map_output(Self::Width).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Embed {
    attributes: Vec<EmbedAttributes>,
}

impl kalosm_sample::Parse for Embed {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        EmbedAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Embed { attributes })
    }
}
