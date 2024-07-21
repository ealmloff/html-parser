use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ThAttributesName {
    #[parse(rename = " abbr=")]
    Abbr,
    #[parse(rename = " align=")]
    Align,
    #[parse(rename = " axis=")]
    Axis,
    #[parse(rename = " bgcolor=")]
    Bgcolor,
    #[parse(rename = " colspan=")]
    Colspan,
    #[parse(rename = " headers=")]
    Headers,
    #[parse(rename = " rowspan=")]
    Rowspan,
    #[parse(rename = " scope=")]
    Scope,
    #[parse(rename = " sorted=")]
    Sorted,
}
#[derive(Debug, Clone)]
pub enum ThAttributes {
    Abbr(String),
    Align(String),
    Axis(String),
    Bgcolor(String),
    Colspan(String),
    Headers(String),
    Rowspan(String),
    Scope(crate::SValues),
    Sorted(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for ThAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(ThAttributesName::new_parser()
                .then_lazy(|name| match name {
                    ThAttributesName::Abbr => String::new_parser().map_output(Self::Abbr).boxed(),
                    ThAttributesName::Align => String::new_parser().map_output(Self::Align).boxed(),
                    ThAttributesName::Axis => String::new_parser().map_output(Self::Axis).boxed(),
                    ThAttributesName::Bgcolor => {
                        String::new_parser().map_output(Self::Bgcolor).boxed()
                    }
                    ThAttributesName::Colspan => {
                        String::new_parser().map_output(Self::Colspan).boxed()
                    }
                    ThAttributesName::Headers => {
                        String::new_parser().map_output(Self::Headers).boxed()
                    }
                    ThAttributesName::Rowspan => {
                        String::new_parser().map_output(Self::Rowspan).boxed()
                    }
                    ThAttributesName::Scope => {
                        crate::SValues::new_parser().map_output(Self::Scope).boxed()
                    }
                    ThAttributesName::Sorted => {
                        String::new_parser().map_output(Self::Sorted).boxed()
                    }
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Th {
    attributes: Vec<ThAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Th {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ThAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</th>")
            .map_output(|(attributes, body)| Th { attributes, body })
    }
}
