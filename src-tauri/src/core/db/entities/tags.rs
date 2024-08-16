//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "tags")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	#[sea_orm(column_type = "Text", nullable)]
	pub name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn initialize_default_tag(db: &DatabaseConnection) -> Result<(), DbErr> {
	let default_tag = ActiveModel {
		id: Set(0),
		name: Set(Some("收藏夹".to_string())),
	};

	// 尝试插入默认标签，如果已存在则忽略
	Entity::insert(default_tag)
		.on_conflict(sea_orm::sea_query::OnConflict::column(Column::Id).do_nothing().to_owned())
		.exec(db)
		.await?;

	Ok(())
}
