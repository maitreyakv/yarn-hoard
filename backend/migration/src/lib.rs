pub use sea_orm_migration::prelude::*;

mod m20250318_233214_create_users_table;
mod m20250406_162007_create_confirmations_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250318_233214_create_users_table::Migration),
            Box::new(m20250406_162007_create_confirmations_table::Migration),
        ]
    }
}
