use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ObjectAttributesName {
    #[parse(rename = " archive=")]
    Archive,
    #[parse(rename = " border=")]
    Border,
    #[parse(rename = " classid=")]
    Classid,
    #[parse(rename = " codebase=")]
    Codebase,
    #[parse(rename = " codetype=")]
    Codetype,
    #[parse(rename = " data=")]
    Data,
    #[parse(rename = " declare=")]
    Declare,
    #[parse(rename = " form=")]
    Form,
    #[parse(rename = " height=")]
    Height,
    #[parse(rename = " name=")]
    Name,
    #[parse(rename = " standby=")]
    Standby,
    #[parse(rename = " tabindex=")]
    Tabindex,
    #[parse(rename = " type=")]
    Type,
    #[parse(rename = " typemustmatch=")]
    Typemustmatch,
    #[parse(rename = " usemap=")]
    Usemap,
    #[parse(rename = " width=")]
    Width,
}
#[derive(Debug, Clone)]
pub enum ObjectAttributes {
    Archive(crate::StringAttributeValue),
    Border(crate::StringAttributeValue),
    Classid(crate::StringAttributeValue),
    Codebase(crate::StringAttributeValue),
    Codetype(crate::StringAttributeValue),
    Data(crate::StringAttributeValue),
    Declare(crate::StringAttributeValue),
    Form(crate::StringAttributeValue),
    Height(crate::StringAttributeValue),
    Name(crate::StringAttributeValue),
    Standby(crate::StringAttributeValue),
    Tabindex(crate::StringAttributeValue),
    Type(crate::StringAttributeValue),
    Typemustmatch(crate::StringAttributeValue),
    Usemap(crate::StringAttributeValue),
    Width(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for ObjectAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(ObjectAttributesName::new_parser()
                .then_lazy(|name| match name {
                    ObjectAttributesName::Archive => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Archive)
                        .boxed(),
                    ObjectAttributesName::Border => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Border)
                        .boxed(),
                    ObjectAttributesName::Classid => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Classid)
                        .boxed(),
                    ObjectAttributesName::Codebase => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Codebase)
                        .boxed(),
                    ObjectAttributesName::Codetype => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Codetype)
                        .boxed(),
                    ObjectAttributesName::Data => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Data)
                        .boxed(),
                    ObjectAttributesName::Declare => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Declare)
                        .boxed(),
                    ObjectAttributesName::Form => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Form)
                        .boxed(),
                    ObjectAttributesName::Height => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Height)
                        .boxed(),
                    ObjectAttributesName::Name => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Name)
                        .boxed(),
                    ObjectAttributesName::Standby => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Standby)
                        .boxed(),
                    ObjectAttributesName::Tabindex => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Tabindex)
                        .boxed(),
                    ObjectAttributesName::Type => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Type)
                        .boxed(),
                    ObjectAttributesName::Typemustmatch => {
                        crate::StringAttributeValue::new_parser()
                            .map_output(Self::Typemustmatch)
                            .boxed()
                    }
                    ObjectAttributesName::Usemap => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Usemap)
                        .boxed(),
                    ObjectAttributesName::Width => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Width)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Object {
    attributes: Vec<ObjectAttributes>,
    body: Vec<crate::Node>,
}

impl kalosm_sample::Parse for Object {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ObjectAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(
                kalosm_sample::LazyParser::new(|| crate::Node::new_parser().boxed())
                    .repeat(0..=10000),
            )
            .then_literal("</object>")
            .map_output(|(attributes, body)| Object { attributes, body })
    }
}
