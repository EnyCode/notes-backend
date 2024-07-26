#[macro_use]
extern crate rocket;
use std::str::FromStr;

use crate::rocket::tokio::io::AsyncReadExt;
use regex::Regex;
use rocket::fs::TempFile;
use rtf_parser::{document::RtfDocument, lexer::Lexer, parser::StyleBlock, tokens::Token};

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[post("/upload/<note>", data = "<file>")]
async fn upload(note: &str, file: TempFile<'_>) -> String {
    let re = Regex::new(r"\\AppleTypeServices(?:.+? )").unwrap();
    let mut rst = String::new();

    file.open()
        .await
        .unwrap()
        .read_to_string(&mut rst)
        .await
        .unwrap();

    rst = (*re.replace_all(&rst, "")).to_string();

    let doc = match RtfDocument::try_from(rst) {
        Ok(doc) => doc,
        Err(_) => return String::from_str("Error parsing RTF").unwrap(),
    };

    println!("{:#?}", doc.get_text());
    println!("{}", doc_to_md(doc).await);

    return format!("{:#?}", "hey");
}

fn token_to_md(token: &StyleBlock) -> String {
    let trimmed_end = token.text.trim_end();
    let trimmed_start = token.text.trim_start();
    let trimmed = token.text.trim();
    let text = &token.text;

    let start = &text[..(text.len() - trimmed_start.len())];
    let end = &text[trimmed_end.len()..text.len()];

    let mut prefixsuffix = String::new();

    if token.painter.bold {
        prefixsuffix.push_str("**");
    }
    if token.painter.italic {
        prefixsuffix.push_str("*");
    }

    println!(":{}: :{}: and {} ({})", start, end, prefixsuffix, trimmed);

    format!(
        "{}{}{}{}{}",
        start, prefixsuffix, trimmed, prefixsuffix, end
    )
}

async fn doc_to_md(doc: RtfDocument) -> String {
    let mut md = String::new();

    let mut i = 0;
    let len = doc.body.len();
    while i < len {
        let b = &doc.body[i];
        i += 1;
        md.push_str(&token_to_md(b));
    }

    md
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload])
}
