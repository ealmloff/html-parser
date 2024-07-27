use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum VideoAttributesName {
    #[parse(rename = " autoplay=")]
    Autoplay,
    #[parse(rename = " controls=")]
    Controls,
    #[parse(rename = " crossorigin=")]
    Crossorigin,
    #[parse(rename = " height=")]
    Height,
    #[parse(rename = " loop=")]
    Loop,
    #[parse(rename = " mediagroup=")]
    Mediagroup,
    #[parse(rename = " muted=")]
    Muted,
    #[parse(rename = " poster=")]
    Poster,
    #[parse(rename = " preload=")]
    Preload,
    #[parse(rename = " src=")]
    Src,
    #[parse(rename = " width=")]
    Width,
}
#[derive(Debug, Clone)]
pub enum VideoAttributes {
    Autoplay(crate::StringAttributeValue),
    Controls(crate::StringAttributeValue),
    Crossorigin(crate::XoValues),
    Height(crate::StringAttributeValue),
    Loop(crate::StringAttributeValue),
    Mediagroup(crate::StringAttributeValue),
    Muted(crate::StringAttributeValue),
    Poster(crate::StringAttributeValue),
    Preload(crate::PlValues),
    Src(crate::StringAttributeValue),
    Width(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for VideoAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(VideoAttributesName::new_parser()
                .then_lazy(|name| match name {
                    VideoAttributesName::Autoplay => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Autoplay)
                        .boxed(),
                    VideoAttributesName::Controls => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Controls)
                        .boxed(),
                    VideoAttributesName::Crossorigin => crate::XoValues::new_parser()
                        .map_output(Self::Crossorigin)
                        .boxed(),
                    VideoAttributesName::Height => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Height)
                        .boxed(),
                    VideoAttributesName::Loop => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Loop)
                        .boxed(),
                    VideoAttributesName::Mediagroup => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Mediagroup)
                        .boxed(),
                    VideoAttributesName::Muted => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Muted)
                        .boxed(),
                    VideoAttributesName::Poster => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Poster)
                        .boxed(),
                    VideoAttributesName::Preload => crate::PlValues::new_parser()
                        .map_output(Self::Preload)
                        .boxed(),
                    VideoAttributesName::Src => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Src)
                        .boxed(),
                    VideoAttributesName::Width => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Width)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Video {
    attributes: Vec<VideoAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Video {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        VideoAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</video>")
            .map_output(|(attributes, body)| Video { attributes, body })
    }
}
