use crate::package::*;

pub struct AppTestSuite {
    in_memory_executor_database: InMemoryExecutorDatabase,
}

impl HasExecutorRepository for AppTestSuite {
    type Repository = InMemoryExecutorDatabase;

    fn executor_repository(&self) -> &Self::Repository {
        &self.in_memory_executor_database
    }
}
