use sea_orm_migration::prelude::*;
use sea_orm::Schema;
use entity::user::Entity;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        let statement = schema
            .create_table_from_entity(Entity)
            .if_not_exists()
            .to_owned();

        manager
            .create_table(statement)
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}