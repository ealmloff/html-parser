use kalosm_sample::*;
type WbrAttributes = crate::GlobalAttribute;
#[derive(Debug, Clone)]
pub struct Wbr {
    attributes: Vec<WbrAttributes>,
}

impl kalosm_sample::Parse for Wbr {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        WbrAttributes::new_parser()
            .repeat(0..=10000)
            .then_literal("/>")
            .map_output(|attributes| Wbr { attributes })
    }
}
