use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum SourceAttributesName {
    #[parse(rename = " media=")]
    Media,
    #[parse(rename = " sizes=")]
    Sizes,
    #[parse(rename = " src=")]
    Src,
    #[parse(rename = " srcset=")]
    Srcset,
    #[parse(rename = " type=")]
    Type,
}
#[derive(Debug, Clone)]
pub enum SourceAttributes {
    Media(String),
    Sizes(String),
    Src(String),
    Srcset(String),
    Type(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for SourceAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(SourceAttributesName::new_parser()
                .then_lazy(|name| match name {
                    SourceAttributesName::Media => {
                        String::new_parser().map_output(Self::Media).boxed()
                    }
                    SourceAttributesName::Sizes => {
                        String::new_parser().map_output(Self::Sizes).boxed()
                    }
                    SourceAttributesName::Src => String::new_parser().map_output(Self::Src).boxed(),
                    SourceAttributesName::Srcset => {
                        String::new_parser().map_output(Self::Srcset).boxed()
                    }
                    SourceAttributesName::Type => {
                        String::new_parser().map_output(Self::Type).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Source {
    attributes: Vec<SourceAttributes>,
}

impl kalosm_sample::Parse for Source {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        SourceAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Source { attributes })
    }
}
