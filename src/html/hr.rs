use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum HrAttributesName {
    #[parse(rename = " align=")]
    Align,
    #[parse(rename = " color=")]
    Color,
    #[parse(rename = " noshade=")]
    Noshade,
    #[parse(rename = " size=")]
    Size,
    #[parse(rename = " width=")]
    Width,
}
#[derive(Debug, Clone)]
pub enum HrAttributes {
    Align(String),
    Color(String),
    Noshade(String),
    Size(String),
    Width(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for HrAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(HrAttributesName::new_parser()
                .then_lazy(|name| match name {
                    HrAttributesName::Align => String::new_parser().map_output(Self::Align).boxed(),
                    HrAttributesName::Color => String::new_parser().map_output(Self::Color).boxed(),
                    HrAttributesName::Noshade => {
                        String::new_parser().map_output(Self::Noshade).boxed()
                    }
                    HrAttributesName::Size => String::new_parser().map_output(Self::Size).boxed(),
                    HrAttributesName::Width => String::new_parser().map_output(Self::Width).boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Hr {
    attributes: Vec<HrAttributes>,
}

impl kalosm_sample::Parse for Hr {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        HrAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Hr { attributes })
    }
}
