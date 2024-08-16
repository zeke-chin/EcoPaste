use crate::core::clipboard::clipboard::PasteboardContent;
use crate::core::db::entities::history;
use crate::core::db::entities::history::{ActiveModel, Model};
use crate::core::db::entities::prelude::{History, Tags};
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait, InsertResult, QueryFilter, QuerySelect};

pub async fn add_clipboard_content(db: &DatabaseConnection, item: PasteboardContent) -> Result<InsertResult<ActiveModel>, DbErr> {
	let db_item = history::ActiveModel {
		r#type: sea_orm::Set(item.r#type as i32),
		value: sea_orm::Set(item.value),
		search: sea_orm::Set(item.search),
		hash: sea_orm::Set(item.hash),
		width: sea_orm::Set(item.width),
		height: sea_orm::Set(item.height),
		size: sea_orm::Set(item.size),
		timestamp: sea_orm::Set(item.timestamp),
		tag_id: sea_orm::Set(item.tag_id),
		..Default::default()
	};

	History::insert(db_item).exec(db).await
}

pub async fn get_clipboard_content(
	db: &DatabaseConnection,
	text: Option<&str>,
	num: Option<u64>,
	type_list: Option<Vec<i32>>,
	tag_id: Option<i32>,
	expired_ts: i64,
) -> Result<Vec<history::Model>, DbErr> {
	let mut query = History::find();
	// 添加过期时间
	query = query.filter(history::Column::Timestamp.gt(expired_ts));
	if let Some(text) = text {
		query = query.filter(Expr::cust("LOWER(search)").like(format!("%{}%", text)));
	}
	if let Some(type_list) = type_list {
		query = query.filter(history::Column::Type.is_in(type_list));
	}
	if let Some(tag_id) = tag_id {
		// 判断tag_id是否在tags内
		let tags = Tags::find_by_id(tag_id).one(db).await?;
		if tags.is_none() {
			return Err(DbErr::Custom(format!("tag_id {} not in tags", tag_id)));
		}
		query = query.filter(history::Column::TagId.eq(tag_id));
	}
	query.limit(num).all(db).await
}

pub async fn update_clipboard_content_tag_id(db: &DatabaseConnection, item_id: i32, tag_id: i32) -> Result<Model, DbErr> {
	History::update(history::ActiveModel {
		id: sea_orm::Set(item_id),
		tag_id: sea_orm::Set(Some(tag_id)),
		..Default::default()
	})
	.exec(db)
	.await
}

pub async fn delete_clipboard_content(db: &DatabaseConnection, item_id: i32) -> Result<DeleteResult, DbErr> {
	Ok(History::delete_by_id(item_id).exec(db).await?)
}
