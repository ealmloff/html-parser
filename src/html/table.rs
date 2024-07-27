use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum TableAttributesName {
    #[parse(rename = " align=")]
    Align,
    #[parse(rename = " border=")]
    Border,
}
#[derive(Debug, Clone)]
pub enum TableAttributes {
    Align(crate::StringAttributeValue),
    Border(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for TableAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(TableAttributesName::new_parser()
                .then_lazy(|name| match name {
                    TableAttributesName::Align => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Align)
                        .boxed(),
                    TableAttributesName::Border => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Border)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Table {
    attributes: Vec<TableAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Table {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        TableAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</table>")
            .map_output(|(attributes, body)| Table { attributes, body })
    }
}
