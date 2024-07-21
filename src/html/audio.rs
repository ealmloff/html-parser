use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum AudioAttributesName {
    #[parse(rename = " autoplay=")]
    Autoplay,
    #[parse(rename = " controls=")]
    Controls,
    #[parse(rename = " crossorigin=")]
    Crossorigin,
    #[parse(rename = " loop=")]
    Loop,
    #[parse(rename = " mediagroup=")]
    Mediagroup,
    #[parse(rename = " muted=")]
    Muted,
    #[parse(rename = " preload=")]
    Preload,
    #[parse(rename = " src=")]
    Src,
}
#[derive(Debug, Clone)]
pub enum AudioAttributes {
    Autoplay(String),
    Controls(String),
    Crossorigin(crate::XoValues),
    Loop(String),
    Mediagroup(String),
    Muted(String),
    Preload(crate::PlValues),
    Src(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for AudioAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(AudioAttributesName::new_parser()
                .then_lazy(|name| match name {
                    AudioAttributesName::Autoplay => {
                        String::new_parser().map_output(Self::Autoplay).boxed()
                    }
                    AudioAttributesName::Controls => {
                        String::new_parser().map_output(Self::Controls).boxed()
                    }
                    AudioAttributesName::Crossorigin => crate::XoValues::new_parser()
                        .map_output(Self::Crossorigin)
                        .boxed(),
                    AudioAttributesName::Loop => {
                        String::new_parser().map_output(Self::Loop).boxed()
                    }
                    AudioAttributesName::Mediagroup => {
                        String::new_parser().map_output(Self::Mediagroup).boxed()
                    }
                    AudioAttributesName::Muted => {
                        String::new_parser().map_output(Self::Muted).boxed()
                    }
                    AudioAttributesName::Preload => crate::PlValues::new_parser()
                        .map_output(Self::Preload)
                        .boxed(),
                    AudioAttributesName::Src => String::new_parser().map_output(Self::Src).boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Audio {
    attributes: Vec<AudioAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Audio {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        AudioAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</audio>")
            .map_output(|(attributes, body)| Audio { attributes, body })
    }
}
