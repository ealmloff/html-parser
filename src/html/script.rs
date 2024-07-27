use kalosm_sample::*;
#[derive(Debug, Clone, Parse)]
#[parse(unquoted)]
pub enum ScriptAttributesName {
    #[parse(rename = " async=")]
    Async,
    #[parse(rename = " charset=")]
    Charset,
    #[parse(rename = " crossorigin=")]
    Crossorigin,
    #[parse(rename = " defer=")]
    Defer,
    #[parse(rename = " integrity=")]
    Integrity,
    #[parse(rename = " nomodule=")]
    Nomodule,
    #[parse(rename = " nonce=")]
    Nonce,
    #[parse(rename = " referrerpolicy=")]
    Referrerpolicy,
    #[parse(rename = " src=")]
    Src,
    #[parse(rename = " text=")]
    Text,
    #[parse(rename = " type=")]
    Type,
}
#[derive(Debug, Clone)]
pub enum ScriptAttributes {
    Async(crate::StringAttributeValue),
    Charset(crate::StringAttributeValue),
    Crossorigin(crate::XoValues),
    Defer(crate::StringAttributeValue),
    Integrity(crate::StringAttributeValue),
    Nomodule(crate::StringAttributeValue),
    Nonce(crate::StringAttributeValue),
    Referrerpolicy(crate::StringAttributeValue),
    Src(crate::StringAttributeValue),
    Text(crate::StringAttributeValue),
    Type(crate::StringAttributeValue),
    GlobalAttribute(crate::GlobalAttribute),
}
impl kalosm_sample::Parse for ScriptAttributes {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        crate::GlobalAttribute::new_parser()
            .map_output(Self::GlobalAttribute)
            .boxed()
            .or(ScriptAttributesName::new_parser()
                .then_lazy(|name| match name {
                    ScriptAttributesName::Async => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Async)
                        .boxed(),
                    ScriptAttributesName::Charset => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Charset)
                        .boxed(),
                    ScriptAttributesName::Crossorigin => crate::XoValues::new_parser()
                        .map_output(Self::Crossorigin)
                        .boxed(),
                    ScriptAttributesName::Defer => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Defer)
                        .boxed(),
                    ScriptAttributesName::Integrity => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Integrity)
                        .boxed(),
                    ScriptAttributesName::Nomodule => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Nomodule)
                        .boxed(),
                    ScriptAttributesName::Nonce => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Nonce)
                        .boxed(),
                    ScriptAttributesName::Referrerpolicy => {
                        crate::StringAttributeValue::new_parser()
                            .map_output(Self::Referrerpolicy)
                            .boxed()
                    }
                    ScriptAttributesName::Src => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Src)
                        .boxed(),
                    ScriptAttributesName::Text => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Text)
                        .boxed(),
                    ScriptAttributesName::Type => crate::StringAttributeValue::new_parser()
                        .map_output(Self::Type)
                        .boxed(),
                })
                .map_output(|(_, attribute)| attribute)
                .boxed())
    }
}
#[derive(Debug, Clone)]
pub struct Script {
    attributes: Vec<ScriptAttributes>,
    body: crate::TextNode,
}

impl kalosm_sample::Parse for Script {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        ScriptAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(crate::AnyTextNodeParser)
            .then_literal("</script>")
            .map_output(|(attributes, body)| Script { attributes, body })
    }
}
