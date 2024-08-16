use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create History table
        manager
            .create_table(
                Table::create()
                    .table(History::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(History::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(History::Type).integer().not_null())
                    .col(ColumnDef::new(History::Value).text().not_null())
                    .col(ColumnDef::new(History::Search).text().not_null())
                    .col(ColumnDef::new(History::Hash).text().not_null())
                    .col(ColumnDef::new(History::Width).integer().null())
                    .col(ColumnDef::new(History::Height).integer().null())
                    .col(ColumnDef::new(History::Size).integer().null())
                    .col(ColumnDef::new(History::Timestamp).integer().not_null())
                    .col(ColumnDef::new(History::TagId).integer().null())
                    .to_owned(),
            )
            .await?;

        // Create Tags table
        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tags::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Tags::Name).text().null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop History table
        manager
            .drop_table(Table::drop().table(History::Table).to_owned())
            .await?;

        // Drop Tags table
        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Represents the "history" table
#[derive(Iden)]
enum History {
    Table,
    Id,
    Type,
    Value,
    Search,
    Hash,
    Width,
    Height,
    Size,
    Timestamp,
    TagId,
}

/// Represents the "tags" table
#[derive(Iden)]
enum Tags {
    Table,
    Id,
    Name,
}