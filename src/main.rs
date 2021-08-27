use pulldown_cmark::{Parser, Event::*, Tag};
use std::io::*;

fn main() -> Result<()> {
    let mut args = std::env::args();
    let source = if let Some(filename) = args.nth(1) {
        let mut file = std::fs::File::open(std::path::Path::new(&filename))
            .expect(&format!("Failed to open file at:\n {}", filename));
        let mut contents = String::new(); file.read_to_string(&mut contents)?;
        contents
    } else {
       String::from("### Hello, this is h3 \n with some _italics_ and `code` \n and \n ``` \n some\n multiline\n code\n```")
    };

    let parser = Parser::new(&source);
    let mut skip_once: bool = false;
    let jira = parser.map(|event| match event {
        Start(Tag::Heading(x)) if x >= 1 && x <= 6 => format!("h{}. ", x),
        Start(Tag::Emphasis) | End(Tag::Emphasis) => format!("_"),
        Start(Tag::Strong) | End(Tag::Strong) => format!("*"),
        Start(Tag::Link(_, url, title)) => {
            skip_once = true;
            format!("[{}|{}]", title, url)
        },
        End(Tag::Link(_,_,_)) => format!(""),
        Start(Tag::Item) => format!("- "),
        Text(_) if skip_once => {
            skip_once = false;
            "".to_owned()
        },
        Text(text) => text.into_string(),
        Code(text) => format!("{{{{{}}}}}", text),
        Start(Tag::CodeBlock(_)) | End(Tag::CodeBlock(_)) => format!("{{code}}\n"),
        End(_) | SoftBreak => format!("\n"),
        _ => "".to_owned(),
    })
    .fold(String::new(), |mut acc, item| {
        acc.push_str(&item); 
        acc
    });

    let mut out = stdout();
    write!(&mut out, "{}", jira)?;
    Ok(())
}
