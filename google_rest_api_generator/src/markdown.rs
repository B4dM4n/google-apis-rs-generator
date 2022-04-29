use pulldown_cmark::{CodeBlockKind, Parser};
use pulldown_cmark_to_cmark::cmark;

/// Currently does the following
/// * look for code blocks, and rewrite them as 'text'. Sometimes, these are code in any other language, thus far never in Rust.
///   Cargo doc will complain (warn) if the code block is not valid Rust, and we don't want to get into the habit of ignoring warnings.
///   On the bright side, cargo doc does exactly what we do, it interprets these blocks as text in the end.
pub fn sanitize(md: &str) -> String {
    let mut output = String::with_capacity(2048);
    cmark(
        Parser::new_ext(md, pulldown_cmark::Options::all()).map(|e| {
            use pulldown_cmark::Event::*;
            match e {
                Start(ref tag) => {
                    use pulldown_cmark::Tag::*;
                    match tag {
                        CodeBlock(CodeBlockKind::Indented) => {
                            Start(CodeBlock(CodeBlockKind::Fenced("text".into())))
                        }
                        CodeBlock(CodeBlockKind::Fenced(code)) => Start(CodeBlock(
                            CodeBlockKind::Fenced(format!("text{}", code).into()),
                        )),
                        _ => e,
                    }
                }
                _ => e,
            }
        }),
        &mut output,
    )
    .unwrap();
    output
}
