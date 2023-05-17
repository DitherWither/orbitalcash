use crate::services::UserService;
use sqlx::{migrate::MigrateError, PgPool};

use super::{expense_service::ExpenseService, TagService};

pub struct ApplicationService {
    db: PgPool,
    pub users: UserService,
    pub expenses: ExpenseService,
    pub tags: TagService,
}

impl ApplicationService {
    // Create a new application service and run migrations
    pub async fn new(db: PgPool) -> Result<Self, MigrateError> {
        let service = Self {
            db: db.clone(),
            users: UserService::new(db.clone()),
            expenses: ExpenseService::new(db.clone()),
            tags: TagService::new(db.clone()),
        };
        // Run Migrations
        service.run_migrations().await?;

        Ok(service)
    }

    /// Runs database migrations
    pub async fn run_migrations(&self) -> Result<(), MigrateError> {
        sqlx::migrate!("./migrations").run(&self.db).await
    }
}
