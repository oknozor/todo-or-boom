use miette::Report;
use pest::Parser;
use pest_derive::Parser;

use crate::diagnostics::{Diagnostics, FixmeDiagnostic, TodoDiagnostic};

#[derive(Parser)]
#[grammar = "parsers/c_style.pest"]
pub struct CStyle;

pub fn parse(file: &str, filename: &str, diagnostics: &mut Diagnostics) -> anyhow::Result<()> {
    let comments = CStyle::parse(Rule::comments, file)?
        .next()
        .expect("comment parse rules");

    for pair in comments.into_inner() {
        for pair in pair.into_inner() {
            let comment = pair.into_inner().next().expect("line or block comment");

            match comment.as_rule() {
                Rule::line_comment => {
                    let content = comment
                        .into_inner()
                        .next()
                        .expect("expected comment_content");
                    for part in content.into_inner() {
                        match part.as_rule() {
                            Rule::todo => {
                                let span = part.as_span();
                                diagnostics.todos.push(Report::new(TodoDiagnostic::new(
                                    file,
                                    filename,
                                    span.start(),
                                    span.end(),
                                )));
                            }
                            Rule::fixme => {
                                let span = part.as_span();
                                diagnostics.fixmes.push(Report::new(FixmeDiagnostic::new(
                                    file,
                                    filename,
                                    span.start(),
                                    span.end(),
                                )));
                            }
                            _ => {}
                        }
                    }
                }
                Rule::block_comment => {
                    let text = comment.into_inner().next().expect("expected block_text");
                    match text.as_rule() {
                        Rule::todo => {
                            let span = text.as_span();
                            diagnostics.todos.push(Report::new(TodoDiagnostic::new(
                                file,
                                filename,
                                span.start(),
                                span.end(),
                            )));
                        }
                        Rule::fixme => {
                            let span = text.as_span();
                            diagnostics.fixmes.push(Report::new(FixmeDiagnostic::new(
                                file,
                                filename,
                                span.start(),
                                span.end(),
                            )));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::parsers::parse;

    #[test]
    fn should_parse_c_style_comments() -> anyhow::Result<()> {
        let file = r#"
// Hello main
fn main() {
    /* Hello */
    /*
    violet are flowers
    roses are flowers
    blue is not
    */
    // TODO: line todo
    println!("Hello, world!");
    /*
    todo: block todo
    with muliple lines
    */

    // FIXME: do stuff
    /*
    fixme: more fixmes
    */
}
"#;

        let diagnostics = parse(file, "toto.rs")?;
        assert_eq!(diagnostics.todos.len(), 2);
        assert_eq!(diagnostics.fixmes.len(), 2);
        Ok(())
    }
}
