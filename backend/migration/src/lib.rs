pub use sea_orm_migration::prelude::*;

mod m20250318_233214_create_database_schema;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250318_233214_create_database_schema::Migration)]
    }
}
