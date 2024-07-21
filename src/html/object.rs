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
    Archive(String),
    Border(String),
    Classid(String),
    Codebase(String),
    Codetype(String),
    Data(String),
    Declare(String),
    Form(String),
    Height(String),
    Name(String),
    Standby(String),
    Tabindex(String),
    Type(String),
    Typemustmatch(String),
    Usemap(String),
    Width(String),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for ObjectAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(ObjectAttributesName::new_parser()
                .then_lazy(|name| match name {
                    ObjectAttributesName::Archive => {
                        String::new_parser().map_output(Self::Archive).boxed()
                    }
                    ObjectAttributesName::Border => {
                        String::new_parser().map_output(Self::Border).boxed()
                    }
                    ObjectAttributesName::Classid => {
                        String::new_parser().map_output(Self::Classid).boxed()
                    }
                    ObjectAttributesName::Codebase => {
                        String::new_parser().map_output(Self::Codebase).boxed()
                    }
                    ObjectAttributesName::Codetype => {
                        String::new_parser().map_output(Self::Codetype).boxed()
                    }
                    ObjectAttributesName::Data => {
                        String::new_parser().map_output(Self::Data).boxed()
                    }
                    ObjectAttributesName::Declare => {
                        String::new_parser().map_output(Self::Declare).boxed()
                    }
                    ObjectAttributesName::Form => {
                        String::new_parser().map_output(Self::Form).boxed()
                    }
                    ObjectAttributesName::Height => {
                        String::new_parser().map_output(Self::Height).boxed()
                    }
                    ObjectAttributesName::Name => {
                        String::new_parser().map_output(Self::Name).boxed()
                    }
                    ObjectAttributesName::Standby => {
                        String::new_parser().map_output(Self::Standby).boxed()
                    }
                    ObjectAttributesName::Tabindex => {
                        String::new_parser().map_output(Self::Tabindex).boxed()
                    }
                    ObjectAttributesName::Type => {
                        String::new_parser().map_output(Self::Type).boxed()
                    }
                    ObjectAttributesName::Typemustmatch => {
                        String::new_parser().map_output(Self::Typemustmatch).boxed()
                    }
                    ObjectAttributesName::Usemap => {
                        String::new_parser().map_output(Self::Usemap).boxed()
                    }
                    ObjectAttributesName::Width => {
                        String::new_parser().map_output(Self::Width).boxed()
                    }
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
