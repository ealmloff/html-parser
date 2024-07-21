use html_parser::*;
use kalosm_sample::*;

fn main() {
    let parser = Element::new_parser();
    println!("size of enum: {}", std::mem::size_of_val(&parser));
    let state = parser.create_parser_state();
    println!("size of state: {}", std::mem::size_of_val(&state));
    loop {
        // read line
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "quit" {
            break;
        }
        let line = line.trim();
        println!("parsing: {}", line);
        let result = parser.parse(&state, line.as_bytes());
        match result {
            Ok(ParseStatus::Finished { result, remaining }) => {
                println!("result: {:?}", result);
            }
            Ok(ParseStatus::Incomplete { new_state, .. }) => {
                println!(
                    "size of partial state: {}",
                    std::mem::size_of_val(&new_state)
                );
                println!("incomplete");
            }
            Err(error) => {
                println!("error: {:?}", error);
            }
        }
    }
}
