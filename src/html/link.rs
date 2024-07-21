use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum LinkAttributesName {
    #[parse(rename = " as=")]
    As,
    #[parse(rename = " crossorigin=")]
    Crossorigin,
    #[parse(rename = " href=")]
    Href,
    #[parse(rename = " hreflang=")]
    Hreflang,
    #[parse(rename = " importance=")]
    Importance,
    #[parse(rename = " integrity=")]
    Integrity,
    #[parse(rename = " media=")]
    Media,
    #[parse(rename = " referrerpolicy=")]
    Referrerpolicy,
    #[parse(rename = " rel=")]
    Rel,
    #[parse(rename = " sizes=")]
    Sizes,
    #[parse(rename = " title=")]
    Title,
    #[parse(rename = " type=")]
    Type,
}
#[derive(Debug, Clone)]
pub enum LinkAttributes {
    As(String),
    Crossorigin(crate::XoValues),
    Href(String),
    Hreflang(String),
    Importance(String),
    Integrity(String),
    Media(String),
    Referrerpolicy(String),
    Rel(String),
    Sizes(String),
    Title(String),
    Type(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for LinkAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(LinkAttributesName::new_parser()
                .then_lazy(|name| match name {
                    LinkAttributesName::As => String::new_parser().map_output(Self::As).boxed(),
                    LinkAttributesName::Crossorigin => crate::XoValues::new_parser()
                        .map_output(Self::Crossorigin)
                        .boxed(),
                    LinkAttributesName::Href => String::new_parser().map_output(Self::Href).boxed(),
                    LinkAttributesName::Hreflang => {
                        String::new_parser().map_output(Self::Hreflang).boxed()
                    }
                    LinkAttributesName::Importance => {
                        String::new_parser().map_output(Self::Importance).boxed()
                    }
                    LinkAttributesName::Integrity => {
                        String::new_parser().map_output(Self::Integrity).boxed()
                    }
                    LinkAttributesName::Media => {
                        String::new_parser().map_output(Self::Media).boxed()
                    }
                    LinkAttributesName::Referrerpolicy => String::new_parser()
                        .map_output(Self::Referrerpolicy)
                        .boxed(),
                    LinkAttributesName::Rel => String::new_parser().map_output(Self::Rel).boxed(),
                    LinkAttributesName::Sizes => {
                        String::new_parser().map_output(Self::Sizes).boxed()
                    }
                    LinkAttributesName::Title => {
                        String::new_parser().map_output(Self::Title).boxed()
                    }
                    LinkAttributesName::Type => String::new_parser().map_output(Self::Type).boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Link {
    attributes: Vec<LinkAttributes>,
}

impl kalosm_sample::Parse for Link {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        LinkAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Link { attributes })
    }
}
