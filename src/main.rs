#![recursion_limit = "512"]

use kalosm_sample::*;

use html::*;
mod html;

fn main() {
    let parser = Div::new_parser();
    println!("size of enum: {}", std::mem::size_of_val(&parser));
    let state = parser.create_parser_state();
    println!("size of state: {}", std::mem::size_of_val(&state));
    let result = parser.parse(&state, b"<div width=\"100\"></div>123");
    match result {
        Ok(ParseStatus::Finished { result, remaining }) => {
            println!("result: {:?}", result);
        }
        Ok(ParseStatus::Incomplete { .. }) => {
            println!("incomplete");
        }
        Err(error) => {
            println!("error: {:?}", error);
        }
    }
}
