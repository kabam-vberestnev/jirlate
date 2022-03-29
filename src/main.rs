use pulldown_cmark::{Event::*, Options, Parser, Tag};
use std::io::*;

fn main() -> Result<()> {
    let mut source = String::new();
    BufReader::new(stdin())
        .read_to_string(&mut source)?;
    
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&source, options);

    let mut skip_once: bool = false;
    let jira = parser.map(|event| match event {
        Start(Tag::Heading(x)) if (1..=6).contains(&x) => format!("h{}. ", x),
        Start(Tag::Strikethrough) | End(Tag::Strikethrough) => "-".to_string(),
        Start(Tag::Emphasis) | End(Tag::Emphasis) => "_".to_string(),
        Start(Tag::Strong) | End(Tag::Strong) => "*".to_string(),
        Start(Tag::Link(_, url, title)) => {
            skip_once = true;
            format!("[{}|{}]", title, url)
        },
        End(Tag::Link(_,_,_)) => String::new(),
        Start(Tag::Item) => "- ".to_string(),
        Text(_) if skip_once => {
            skip_once = false;
            "".to_owned()
        },
        Text(text) => text.into_string(),
        Code(text) => format!("{{{{{}}}}}", text),
        Start(Tag::CodeBlock(_)) | End(Tag::CodeBlock(_)) => "{code}\n".to_string(),
        End(_) | SoftBreak => "\n".to_string(),
        _ => "".to_owned(),
    });

    let mut out = BufWriter::new(stdout());
    for item in jira {
        write!(&mut out, "{}", item)?;
    }
    
    Ok(())
}
