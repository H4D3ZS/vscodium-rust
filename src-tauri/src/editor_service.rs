use crate::domain::EditorCommand;
use crate::repository::{ProjectRepository, ZedProjectRepository};
use gpui::AsyncApp;
use futures::channel::mpsc as futures_mpsc;
use futures::StreamExt;


/// Initializes the Editor Service on the main thread and returns the command sender.
pub async fn init_on_main_thread(cx: &AsyncApp) -> futures_mpsc::UnboundedSender<EditorCommand> {
    println!("VSCodium Rust Editor Service initializing...");
    
    let (cmd_tx, mut cmd_rx) = futures_mpsc::unbounded::<EditorCommand>();
    
    // Repository implementation for Zed/GPUI
    // We use granular updates to avoid re-entrancy panics if spawned tasks (like in UserStore)
    // are polled immediately.
    let mut repo = ZedProjectRepository::create(cx).await;

    println!("VSCodium Rust Editor Service initialized!");

    cx.spawn(async move |cx: &mut AsyncApp| {
        let mut cx = cx.clone();
        // Delay processing to avoid re-entrancy during initial poll.
        cx.background_executor().timer(std::time::Duration::from_millis(50)).await;

        println!("Editor command processor started.");
        while let Some(cmd) = cmd_rx.next().await {
            match cmd {
                EditorCommand::Ping(reply_tx) => {
                    let _ = reply_tx.send("Pong from VSCodium Rust backend!".into());
                }
                EditorCommand::OpenProject(path, reply_tx) => {
                    let result = repo.open_project(&mut cx, path).await;
                    let _ = reply_tx.send(result);
                }
                EditorCommand::GetFileTree(reply_tx) => {
                    let result = repo.get_file_tree(&mut cx).await;
                    let _ = reply_tx.send(result);
                }
            }
        }
    }).detach();
    
    cmd_tx
}
