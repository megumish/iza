pub struct Execution<F>
where
    F: FnOnce() + 'static,
{
    execution_function: F,
}

impl<F> Execution<F>
where
    F: FnOnce() + 'static,
{
    pub fn new(execution_function: F) -> Self {
        Self { execution_function }
    }
}
