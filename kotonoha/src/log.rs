//! Log is a trait that represent kind of log typed some.
/// Log is a trait that represent kind of log typed some.
pub trait Log {
    /// get log message string
    fn log_message(&self) -> Option<String>;
}

/// Show log string by a language like hacker style.
pub struct OneLanguage1337Log {
    kind: OneLanguage1337LogKind,
    content: String,
}

/// 1337Log Kind
pub enum OneLanguage1337LogKind {
    /// error message
    Error,
    /// warn message
    Warn,
    /// success message
    Success,
    /// info message
    Info,
    /// debug message
    Debug,
}

impl OneLanguage1337Log {
    /// constructor
    fn new<S>(kind: OneLanguage1337LogKind, content: S) -> Self
    where
        S: ToString,
    {
        Self {
            kind: kind,
            content: content.to_string(),
        }
    }

    /// error message
    pub fn error<S>(content: S) -> Self
    where
        S: ToString,
    {
        Self::new(OneLanguage1337LogKind::Error, content)
    }

    /// warn message
    pub fn warn<S>(content: S) -> Self
    where
        S: ToString,
    {
        Self::new(OneLanguage1337LogKind::Warn, content)
    }

    /// success message
    pub fn success<S>(content: S) -> Self
    where
        S: ToString,
    {
        Self::new(OneLanguage1337LogKind::Success, content)
    }

    /// info message
    pub fn info<S>(content: S) -> Self
    where
        S: ToString,
    {
        Self::new(OneLanguage1337LogKind::Info, content)
    }

    /// debug message
    pub fn debug<S>(content: S) -> Self
    where
        S: ToString,
    {
        Self::new(OneLanguage1337LogKind::Debug, content)
    }
}

impl Log for OneLanguage1337Log {
    fn log_message(&self) -> Option<String> {
        Some(format!("{} {}", self.kind.head_mark(), &self.content))
    }
}

impl OneLanguage1337LogKind {
    fn head_mark(&self) -> &'static str {
        use OneLanguage1337LogKind::*;
        match self {
            Error => "\\e[1m[x]",
            Warn => "[-]",
            Success => "[+]",
            Info => "[i]",
            Debug => "[d]",
        }
    }
}
