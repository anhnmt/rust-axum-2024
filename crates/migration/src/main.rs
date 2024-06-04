use sea_orm::ConnectOptions;
use sea_orm_migration::prelude::*;

use migration::sea_orm::Database;

#[async_std::main]
async fn main() {
    let connect_options = ConnectOptions::new("postgres://root:root@localhost/database")
        .to_owned();

    let db = Database::connect(connect_options).await?;

    migration::Migrator::up(&db, None).await?;
}
