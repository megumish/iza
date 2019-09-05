//! Log is a trait that represent kind of log typed some.
/// Log is a trait that represent kind of log typed some.
pub trait Log {
    /// get log message string
    fn log_message(&self) -> String;
}

/// Show log string by a language.
pub struct OneLanguageSimpleLog {
    content: &'static str,
}

impl OneLanguageSimpleLog {
    /// constructor
    pub fn new<S>(content: S) -> Self
    where
        S: Into<&'static str>,
    {
        Self {
            content: content.into(),
        }
    }
}

impl Log for OneLanguageSimpleLog {
    fn log_message(&self) -> String {
        self.content.to_owned()
    }
}
