use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ImgAttributesName {
    #[parse(rename = " alt=")]
    Alt,
    #[parse(rename = " crossorigin=")]
    Crossorigin,
    #[parse(rename = " decoding=")]
    Decoding,
    #[parse(rename = " height=")]
    Height,
    #[parse(rename = " importance=")]
    Importance,
    #[parse(rename = " intrinsicsize=")]
    Intrinsicsize,
    #[parse(rename = " ismap=")]
    Ismap,
    #[parse(rename = " loading=")]
    Loading,
    #[parse(rename = " referrerpolicy=")]
    Referrerpolicy,
    #[parse(rename = " sizes=")]
    Sizes,
    #[parse(rename = " src=")]
    Src,
    #[parse(rename = " srcset=")]
    Srcset,
    #[parse(rename = " usemap=")]
    Usemap,
    #[parse(rename = " width=")]
    Width,
}
#[derive(Debug, Clone)]
pub enum ImgAttributes {
    Alt(String),
    Crossorigin(crate::XoValues),
    Decoding(crate::DecodingValues),
    Height(String),
    Importance(String),
    Intrinsicsize(String),
    Ismap(String),
    Loading(crate::LoadingValues),
    Referrerpolicy(crate::ReferrerpolicyValues),
    Sizes(String),
    Src(String),
    Srcset(String),
    Usemap(String),
    Width(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for ImgAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(ImgAttributesName::new_parser()
                .then_lazy(|name| match name {
                    ImgAttributesName::Alt => String::new_parser().map_output(Self::Alt).boxed(),
                    ImgAttributesName::Crossorigin => crate::XoValues::new_parser()
                        .map_output(Self::Crossorigin)
                        .boxed(),
                    ImgAttributesName::Decoding => crate::DecodingValues::new_parser()
                        .map_output(Self::Decoding)
                        .boxed(),
                    ImgAttributesName::Height => {
                        String::new_parser().map_output(Self::Height).boxed()
                    }
                    ImgAttributesName::Importance => {
                        String::new_parser().map_output(Self::Importance).boxed()
                    }
                    ImgAttributesName::Intrinsicsize => {
                        String::new_parser().map_output(Self::Intrinsicsize).boxed()
                    }
                    ImgAttributesName::Ismap => {
                        String::new_parser().map_output(Self::Ismap).boxed()
                    }
                    ImgAttributesName::Loading => crate::LoadingValues::new_parser()
                        .map_output(Self::Loading)
                        .boxed(),
                    ImgAttributesName::Referrerpolicy => crate::ReferrerpolicyValues::new_parser()
                        .map_output(Self::Referrerpolicy)
                        .boxed(),
                    ImgAttributesName::Sizes => {
                        String::new_parser().map_output(Self::Sizes).boxed()
                    }
                    ImgAttributesName::Src => String::new_parser().map_output(Self::Src).boxed(),
                    ImgAttributesName::Srcset => {
                        String::new_parser().map_output(Self::Srcset).boxed()
                    }
                    ImgAttributesName::Usemap => {
                        String::new_parser().map_output(Self::Usemap).boxed()
                    }
                    ImgAttributesName::Width => {
                        String::new_parser().map_output(Self::Width).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Img {
    attributes: Vec<ImgAttributes>,
}

impl kalosm_sample::Parse for Img {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ImgAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Img { attributes })
    }
}
