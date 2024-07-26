#[macro_use]
extern crate rocket;
use std::str::FromStr;

use crate::rocket::tokio::io::AsyncReadExt;
use rocket::fs::TempFile;
use rtf_parser::{document::RtfDocument, lexer::Lexer, tokens::Token};

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[post("/upload/<note>", data = "<file>")]
async fn upload(note: &str, file: TempFile<'_>) -> String {
    let mut rst = String::new();
    file.open()
        .await
        .unwrap()
        .read_to_string(&mut rst)
        .await
        .unwrap();

    let tokens: Vec<Token> = Lexer::scan(&rst).unwrap();

    //println!("{:#?}", tokens);
    doc_to_md(tokens).await;

    return format!("{:#?}", "hey");
}

struct Paragraph {
    text: String,
}

async fn doc_to_md(tokens: Vec<Token<'_>>) -> String {
    let md = String::new();

    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::CRLF => {
                println!("{:#?}", stack);
                stack.clear();
            }
            _ => stack.push(token),
        }
    }

    return String::from_str("Hello").unwrap();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload])
}
