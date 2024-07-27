use kalosm_sample::*;

type CodeAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Code {
    attributes: Vec<CodeAttributes>,
    body: crate::TextNode,
}

impl kalosm_sample::Parse for Code {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        CodeAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal(">")
            .then(crate::AnyTextNodeParser)
            .then_literal("</code>")
            .map_output(|(attributes, body)| Code { attributes, body })
    }
}
