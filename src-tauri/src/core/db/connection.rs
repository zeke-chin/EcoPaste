use log::debug;
use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::path::Path;
use std::time::Duration;
use tauri::api::path::app_data_dir;
use tauri::Config;
use crate::core::db::entities::tags::initialize_default_tag;

pub async fn init_db_connection(app_handle: &tauri::AppHandle) -> Result<DatabaseConnection, DbErr> {
	let app_name = app_handle.package_info().name.clone();
	let data_dir = app_data_dir(&Config::default()).expect("无法获取应用数据目录");
	let extension_name = if cfg!(debug_assertions) { "dev.db" } else { "db" };

	let db_path = data_dir.join(format!("{}.{}?mode=rwc", app_name, extension_name));

	let db_url = format!("sqlite:{}", db_path.display());
	debug!("数据库 URL: {}", &db_url);

	establish_connection(&db_url).await
}

async fn establish_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
	let mut opt = ConnectOptions::new(database_url);
	opt.sqlx_logging(true).sqlx_logging_level(log::LevelFilter::Warn);

	let db = Database::connect(opt).await;
	match db {
		Ok(conn) => {
			Migrator::up(&conn, None).await?;
			initialize_default_tag(&conn).await?;
			Ok(conn)
		}
		Err(e) => Err(e),
	}
}
