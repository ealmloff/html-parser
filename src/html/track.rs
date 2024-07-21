use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum TrackAttributesName {
    #[parse(rename = " default=")]
    Default,
    #[parse(rename = " kind=")]
    Kind,
    #[parse(rename = " label=")]
    Label,
    #[parse(rename = " src=")]
    Src,
    #[parse(rename = " srclang=")]
    Srclang,
}
#[derive(Debug, Clone)]
pub enum TrackAttributes {
    Default(String),
    Kind(crate::TkValues),
    Label(String),
    Src(String),
    Srclang(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for TrackAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(TrackAttributesName::new_parser()
                .then_lazy(|name| match name {
                    TrackAttributesName::Default => {
                        String::new_parser().map_output(Self::Default).boxed()
                    }
                    TrackAttributesName::Kind => {
                        crate::TkValues::new_parser().map_output(Self::Kind).boxed()
                    }
                    TrackAttributesName::Label => {
                        String::new_parser().map_output(Self::Label).boxed()
                    }
                    TrackAttributesName::Src => String::new_parser().map_output(Self::Src).boxed(),
                    TrackAttributesName::Srclang => {
                        String::new_parser().map_output(Self::Srclang).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Track {
    attributes: Vec<TrackAttributes>,
}

impl kalosm_sample::Parse for Track {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        TrackAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Track { attributes })
    }
}
