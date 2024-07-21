use kalosm_sample::*;

use html::*;
mod html;

#[derive(Debug, Clone)]
struct ElementBody<Attribute> {
    attributes: Vec<Attribute>,
    children: Vec<html::Element>,
}

fn main() {
    let parser = InputautocompleteValues::new_parser();
    println!("size of enum: {}", std::mem::size_of_val(&parser));
    let state = parser.create_parser_state();
    println!("size of state: {}", std::mem::size_of_val(&state));
    let result = parser.parse(&state, b"\"transaction-currency\"123");
    match result {
        Ok(ParseStatus::Finished { result, remaining }) => {
            assert!(matches!(result, InputautocompleteValues::TransactionCurrency));
            assert_eq!(remaining, b"123");
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
