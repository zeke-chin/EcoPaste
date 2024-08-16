use crate::core::clipboard::clipboard::PasteboardContent;
use crate::core::db::connection::init_db_connection;
use clipboard_rs::{ClipboardContext, ClipboardHandler, ClipboardWatcher, ClipboardWatcherContext, WatcherShutdown};
use sea_orm::DatabaseConnection;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use tauri::AppHandle;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

struct ClipboardHelper {
	db: Arc<Mutex<DatabaseConnection>>,
	write_ctx: ClipboardContext,
	watcher_shutdown: WatcherShutdown,
	sender: Sender<PasteboardContent>,
	#[allow(dead_code)]
	receiver_handle: JoinHandle<()>,
	#[allow(dead_code)]
	runtime: Arc<Runtime>,
}

impl ClipboardHelper {
   pub async fn new(app_handle: AppHandle) -> Self {
        let db_connection = init_db_connection(&app_handle)
            .await
            .expect("Failed to connect to database");
        let db = Arc::new(Mutex::new(db_connection));

        let (sender, receiver) = mpsc::channel();
        let runtime = Arc::new(Runtime::new().unwrap());

        let db_clone = db.clone();
        let runtime_clone = runtime.clone();
        let receiver_handle = std::thread::spawn(move || {
            Self::process_receiver(receiver, db_clone, runtime_clone);
        });

        let ctx = ClipboardContext::new().unwrap();

        let mut watcher = ClipboardWatcherContext::new().unwrap();
        let mut helper = Self {
            db: db.clone(),
            ctx: ctx,
            watcher_shutdown: Default::default(), // 临时占位，稍后更新
            sender,
            receiver_handle,
            runtime,
        };

        let watcher_shutdown = watcher
            .add_handler(helper.clone()) // 假设 ClipboardHelper 实现了 Clone
            .get_shutdown_channel();

        // 更新 watcher_shutdown
        helper.watcher_shutdown = watcher_shutdown;

        tokio::spawn(async move {
            watcher.start_watch();
        });

        helper
    }

	fn process_receiver(receiver: Receiver<PasteboardContent>, db: Arc<Mutex<DatabaseConnection>>, runtime: Arc<Runtime>) {
		while let Ok(content) = receiver.recv() {
			// debug!("Received clipboard content: {:?}", content);
			runtime.block_on(async {
				Self::add_clipboard_entry(&db, content).await;
			});
		}
	}
}



impl ClipboardHandler for ClipboardHelper {
    fn on_clipboard_change(&mut self) {
		todo!()
	}
}
