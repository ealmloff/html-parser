#![recursion_limit = "512"]

pub use html::*;
mod html;

use kalosm_sample::*;

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(TextNode),
}

impl Parse for Node {
    fn new_parser() -> impl SendCreateParserState<Output = Self> {
        Element::new_parser()
            .map_output(Self::Element)
            .or(TextNodeParser.map_output(Self::Text))
    }
}

#[derive(Debug, Clone)]
pub struct TextNode(pub String);

#[derive(Debug, Clone)]
struct TextNodeParser;

impl CreateParserState for TextNodeParser {
    fn create_parser_state(&self) -> Self::PartialState {
        Vec::new()
    }
}

impl Parser for TextNodeParser {
    type Output = TextNode;
    type PartialState = Vec<u8>;

    fn parse<'a>(
        &self,
        state: &Self::PartialState,
        input: &'a [u8],
    ) -> ParseResult<ParseStatus<'a, Self::PartialState, Self::Output>> {
        let mut state = state.clone();
        for (i, c) in input.iter().enumerate() {
            match c {
                // < and > need to be escaped
                b'<' | b'>' => {
                    if state.is_empty() {
                        bail!("text node cannot be empty");
                    }
                    return Ok(ParseStatus::Finished {
                        result: TextNode(String::from_utf8_lossy(&state).to_string()),
                        remaining: &input[i..],
                    });
                }
                _ => {
                    state.push(*c);
                }
            }
        }

        Ok(ParseStatus::Incomplete {
            new_state: state,
            required_next: std::borrow::Cow::Borrowed(""),
        })
    }
}
