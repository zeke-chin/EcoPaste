use std::fs;
use clipboard_rs::{Clipboard, ClipboardContext, ClipboardHandler, RustImageData};
use log::{error, warn};
use sea_orm::sqlx::types::chrono;
use serde_json::json;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use twox_hash::xxh3::hash64;
use url::Url;

#[derive(Debug, Clone)]
pub enum ContentType {
	Files = 1,
	Image = 2,
	RichText = 3,
	Html = 4,
	Text = 5,
}

pub struct PasteboardContent {
	pub r#type: ContentType,
	pub value: String,
	pub search: String,
	pub hash: String,
	pub width: Option<i32>,
	pub height: Option<i32>,
	pub size: Option<i32>,
	pub timestamp: i64,
	pub tag_id: Option<i32>,
}

impl PasteboardContent {
	fn new(
		r#type: ContentType,
		value: String,
		search: String,
		hash: String,
		width: Option<i32>,
		height: Option<i32>,
		size: Option<i32>,
		tag_id: Option<i32>,
	) -> Self {
		PasteboardContent {
			r#type,
			value,
			search,
			hash,
			width,
			height,
			size,
			timestamp: chrono::Utc::now().timestamp_millis(),
			tag_id,
		}
	}

	fn new_files(file_urls: Vec<String>) -> PasteboardContent {
		let paths: Vec<PathBuf> = file_urls
			.iter()
			.map(|url| {
				let url = Url::parse(url).expect("Invalid URL");
				let decoded_path = urlencoding::decode(url.path()).expect("UTF-8");
				PathBuf::from(decoded_path.as_ref())
			})
			.collect();
		let search_content: String = paths
			.iter()
			.filter_map(|path| path.file_name().and_then(|name| name.to_str()))
			.collect::<Vec<_>>()
			.join(", ");
		let value: Vec<String> = paths
			.into_iter()
			.filter_map(|path| path.to_str().map(String::from))
			.collect();

		let hash = hash_str(&value.join(""));
		PasteboardContent::new(
			ContentType::Files,
			json!(value).to_string(),
			search_content,
			hash,
			None,
			None,
			Some(files_size(value)),
			None,
		)
	}
	fn new_img(img: RustImageData) -> PasteboardContent {
		todo!()
	}
	fn new_html(text_html: String, text: String) -> PasteboardContent {
		todo!()
	}
	fn new_rich(text_rich: String) -> PasteboardContent {
		todo!()
	}
	fn new_text(text: String) -> PasteboardContent {
		todo!()
	}
}

pub struct ClipboardManager {
	ctx: ClipboardContext,
	sender: Sender<PasteboardContent>,
	last_hash: String,
}

impl ClipboardManager {
	pub fn new(sender: Sender<PasteboardContent>) -> Self {
		let ctx = ClipboardContext::new().unwrap();
		ClipboardManager {
			ctx,
			sender,
			last_hash: "".to_string(),
		}
	}

	fn send(&self, content: PasteboardContent) -> Result<(), String> {
		self.sender.send(content).map_err(|e| e.to_string())
	}

	fn check_last_hash(&mut self, new_hash: String) -> bool {
		if new_hash == self.last_hash {
			false
		} else {
			self.last_hash = new_hash;
			true
		}
	}
}

impl ClipboardHandler for ClipboardManager {
	fn on_clipboard_change(&mut self) {
		let mut content = None;

		let mut have_files = false;
		match self.ctx.get_files() {
			Ok(file_urls) if !file_urls.is_empty() => {
				have_files = true;
				let cp = PasteboardContent::new_files(file_urls);
				if self.check_last_hash(cp.hash.clone()) {
					content = Some(cp)
				}
			}
			Ok(_) => {}
			Err(e) => {
				#[cfg(target_os = "windows")]
				{
					warn!("Error getting files from clipboard: {}", e);
				}

				#[cfg(any(target_os = "macos", target_os = "linux"))]
				{
					error!("Error getting files from clipboard: {}", e);
				}
			}
		};
		if !have_files && content.is_none() {
			if let Ok(img) = self.ctx.get_image() {
				let cp = PasteboardContent::new_img(img);
				if self.check_last_hash(cp.hash.clone()) {
					content = Some(cp)
				}
			} else if let Ok(text_html) = self.ctx.get_html() {
				let text = self.ctx.get_text().unwrap();
				let cp = PasteboardContent::new_html(text_html, text);
				if self.check_last_hash(cp.hash.clone()) {
					content = Some(cp)
				}
			} else if let Ok(text_rich) = self.ctx.get_rich_text() {
				let cp = PasteboardContent::new_rich(text_rich);
				if self.check_last_hash(cp.hash.clone()) {
					content = Some(cp)
				}
			} else if let Ok(text) = self.ctx.get_text() {
				let cp = PasteboardContent::new_text(text);
				if self.check_last_hash(cp.hash.clone()) {
					content = Some(cp)
				}
			}
		}
		// 将content push
		if let Some(content) = content {
			let _ = self.sender.send(content);
		}
	}
}

pub fn hash_str(input: &str) -> String {
	format!("{:x}", hash64(input.as_bytes()))
}

pub fn hash_vec(input: &[u8]) -> String {
	format!("{:x}", hash64(input))
}

pub fn files_size(path_list: Vec<String>) -> i32 {
	let mut size = 0;
	for path in path_list {
		let metadata = fs::metadata(path).unwrap();
		size += metadata.len();
	}
	size as i32
}
