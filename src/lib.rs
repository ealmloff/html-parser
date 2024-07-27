#![recursion_limit = "512"]

pub use html::*;
mod html;

use kalosm_sample::*;

#[derive(Debug, Clone)]
pub enum Node {
    Element(Element),
    Text(TextNode),
    Comment(CommentNode),
}

impl Parse for Node {
    fn new_parser() -> impl SendCreateParserState<Output = Self> {
        Element::new_parser()
            .map_output(Self::Element)
            .or(TextNodeParser.map_output(Self::Text))
            .or(CommentNodeParser.map_output(Self::Comment))
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
                b'<' => {
                    if state.is_empty() {
                        bail!("text node cannot be empty");
                    }
                    return Ok(ParseStatus::Finished {
                        result: TextNode(String::from_utf8_lossy(&state).to_string()),
                        remaining: &input[i..],
                    });
                }
                b'>' => {
                    bail!("> must be escaped");
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

const COMMENT_START: &[u8] = b"<!--";
const COMMENT_END: &[u8] = b"-->";

#[derive(Debug, Clone, PartialEq)]
pub struct CommentNode(pub String);

#[derive(Debug, Clone)]
struct CommentNodeParser;

impl CreateParserState for CommentNodeParser {
    fn create_parser_state(&self) -> Self::PartialState {
        Vec::new()
    }
}

impl Parser for CommentNodeParser {
    type Output = CommentNode;
    type PartialState = Vec<u8>;

    fn parse<'a>(
        &self,
        state: &Self::PartialState,
        input: &'a [u8],
    ) -> ParseResult<ParseStatus<'a, Self::PartialState, Self::Output>> {
        let mut state = state.clone();
        for (i, &c) in input.iter().enumerate() {
            if state.len() < COMMENT_START.len() {
                if c != COMMENT_START[state.len()] {
                    bail!("comment node must start with <!--");
                }
                state.push(c);
                continue;
            } else {
                let start = COMMENT_START.len().max(state.len() - 2);
                match state[start..] {
                    [b'-', b'-'] => {
                        if c == b'>' {
                            return Ok(ParseStatus::Finished {
                                result: CommentNode(
                                    String::from_utf8_lossy(
                                        &state[COMMENT_START.len()
                                            ..state.len() - (COMMENT_END.len() - 1)],
                                    )
                                    .to_string(),
                                ),
                                remaining: &input[i + 1..],
                            });
                        }
                        bail!("comment node must end with -->");
                    }
                    [b'-'] | [_, b'-'] => {
                        if c != b'-' {
                            bail!("comment node must end with -->");
                        }
                    }
                    _ => {}
                }
            }
            if !matches!(c, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b' ' | b'-') {
                bail!("comment node must be plain text");
            }
            state.push(c);
        }

        Ok(ParseStatus::Incomplete {
            new_state: state,
            required_next: std::borrow::Cow::Borrowed(""),
        })
    }
}

#[test]
fn parse_comment() {
    let parser = CommentNodeParser;
    let state = parser.create_parser_state();
    assert_eq!(
        parser.parse(&state, b"<!--comment-->").unwrap(),
        ParseStatus::Finished {
            result: CommentNode(String::from("comment")),
            remaining: &[]
        }
    );
    let parser = CommentNodeParser;
    let state = parser.create_parser_state();
    assert_eq!(
        parser.parse(&state, b"<!--1-->").unwrap(),
        ParseStatus::Finished {
            result: CommentNode(String::from("1")),
            remaining: &[]
        }
    );
    let parser = CommentNodeParser;
    let state = parser.create_parser_state();
    assert!(parser.parse(&state, b"<!---1-->").is_err(),);
    let parser = CommentNodeParser;
    let state = parser.create_parser_state();
    assert_eq!(
        parser.parse(&state, b"<!---->").unwrap(),
        ParseStatus::Finished {
            result: CommentNode(String::from("")),
            remaining: &[]
        }
    );
}
