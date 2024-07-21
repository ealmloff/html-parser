use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum OptgroupAttributesName {
    #[parse(rename = " disabled=")]
    Disabled,
    #[parse(rename = " label=")]
    Label,
}
#[derive(Debug, Clone)]
pub enum OptgroupAttributes {
    Disabled(String),
    Label(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for OptgroupAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(OptgroupAttributesName::new_parser()
                .then_lazy(|name| match name {
                    OptgroupAttributesName::Disabled => {
                        String::new_parser().map_output(Self::Disabled).boxed()
                    }
                    OptgroupAttributesName::Label => {
                        String::new_parser().map_output(Self::Label).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Optgroup {
    attributes: Vec<OptgroupAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Optgroup {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        OptgroupAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</optgroup>")
            .map_output(|(attributes, body)| Optgroup { attributes, body })
    }
}
