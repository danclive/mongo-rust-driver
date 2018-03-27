use apm::{CommandStarted, CommandResult};
use client::MongoClient;
use error::Result;

pub trait EventRunner {
    fn run_start_hooks(&self, hook: &CommandStarted) -> Result<()>;
    fn run_completion_hooks(&self, hook: &CommandResult) -> Result<()>;
}

impl EventRunner for MongoClient {
    fn run_start_hooks(&self, hook: &CommandStarted) -> Result<()> {
        self.inner.listener.run_start_hooks(self.clone(), hook)
    }

    fn run_completion_hooks(&self, hook: &CommandResult) -> Result<()> {
        self.inner.listener.run_completion_hooks(self.clone(), hook)
    }
}
