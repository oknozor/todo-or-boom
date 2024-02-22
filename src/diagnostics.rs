use std::fmt;
use std::fmt::Formatter;

use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use owo_colors::OwoColorize;
use thiserror::Error;

#[derive(Default)]
pub struct Diagnostics {
    pub todos: Vec<Report>,
    pub fixmes: Vec<Report>,
}

impl Diagnostics {
    pub fn is_err(&self) -> bool {
        !self.todos.is_empty() || !self.fixmes.is_empty()
    }
}

impl fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.todos.is_empty() && self.fixmes.is_empty() {
            writeln!(f, "{}️ No Todos or Fixme found", "✔".green())
        } else {
            writeln!(f, "Found {} Fixmes in sources", self.fixmes.len())?;
            for fixme in &self.fixmes {
                writeln!(f, "{:?}", fixme)?
            }

            writeln!(f, "Found {} Todos in sources", self.todos.len())?;
            for todo in &self.todos {
                writeln!(f, "{:?}", todo)?;
            }

            writeln!(f, "{} Found {} errors:", "×".red(), self.fixmes.len() + self.todos.len())?;
            writeln!(f, "  - {} {} in sources", self.fixmes.len(), "Fixmes".red())?;
            writeln!(f, "  - {} {} in sources", self.todos.len(), "Todos".yellow())?;

            Ok(())
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("Todo")]
#[diagnostic()]
pub struct TodoDiagnostic {
    #[source_code]
    src: NamedSource,
    #[label("Unresolved todo")]
    bad_bit: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("Fixme")]
#[diagnostic()]
pub struct FixmeDiagnostic {
    #[source_code]
    src: NamedSource,
    #[label("Unresolved fixme")]
    bad_bit: SourceSpan,
}

impl FixmeDiagnostic {
    pub fn new(content: &str, filename: &str, start: usize, end: usize) -> Self {
        Self {
            src: NamedSource::new(filename, content.to_string()),
            bad_bit: (start, end - start).into(),
        }
    }
}

impl TodoDiagnostic {
    pub fn new(content: &str, filename: &str, start: usize, end: usize) -> Self {
        Self {
            src: NamedSource::new(filename, content.to_string()),
            bad_bit: (start, end - start).into(),
        }
    }
}
