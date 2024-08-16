use crate::core::db::entities::prelude::Tags;
use crate::core::db::entities::tags;
use crate::core::db::entities::tags::{ActiveModel, Model};
use sea_orm::{DatabaseConnection, DbErr, DeleteResult, EntityTrait, InsertResult, QuerySelect};

pub async fn add_tag(db: &DatabaseConnection, name: String) -> Result<InsertResult<ActiveModel>, DbErr> {
	let tag = tags::ActiveModel {
		name: sea_orm::Set(Some(name)),
		..Default::default()
	};

	Tags::insert(tag).exec(db).await
}

pub async fn get_tags(db: &DatabaseConnection, num: Option<u64>) -> Result<Vec<tags::Model>, DbErr> {
	let query = Tags::find();
	query.limit(num).all(db).await
}

pub async fn update_tag(db: &DatabaseConnection, id: i32, name: String) -> Result<Model, DbErr> {
	Tags::update(tags::ActiveModel {
		id: sea_orm::Set(id),
		name: sea_orm::Set(Some(name)),
		..Default::default()
	})
	.exec(db)
	.await
}

pub async fn delete_tag(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
	Ok(Tags::delete_by_id(id).exec(db).await?)
}
