use crate::domain::FileEntry;
use std::path::PathBuf;
use std::sync::Arc;
use async_trait::async_trait;
use gpui::{Entity, AppContext, AsyncApp};
use project::Project;
use fs::RealFs;
use language::LanguageRegistry;
use client::{UserStore, Client};
use clock::RealSystemClock;
use http_client::{BlockedHttpClient, HttpClientWithUrl};
use node_runtime::NodeRuntime;
use semver::Version;

#[async_trait(?Send)]
pub trait ProjectRepository: Send + Sync {
    async fn open_project(&mut self, cx: &mut AsyncApp, path: PathBuf) -> Result<(), String>;
    async fn get_file_tree(&self, cx: &mut AsyncApp) -> Result<Vec<FileEntry>, String>;
}

pub struct ZedProjectRepository {
    pub client: Arc<Client>,
    pub node_runtime: NodeRuntime,
    pub user_store: Entity<UserStore>,
    pub languages: Arc<LanguageRegistry>,
    pub fs: Arc<RealFs>,
    pub current_project: Option<Entity<Project>>,
}

impl ZedProjectRepository {
    pub async fn create(cx: &AsyncApp) -> Self {
        // Yield to the executor to allow the initial App borrow by Application::run to be released.
        // This avoids a RefCell borrow conflict during GPUI initialization.
        cx.background_executor().timer(std::time::Duration::from_millis(10)).await;

        let (inner_http, http_client, clock): (Arc<BlockedHttpClient>, Arc<HttpClientWithUrl>, Arc<RealSystemClock>) = cx.update(|_cx| {
            let inner_http = Arc::new(BlockedHttpClient::new());
            let http_client = Arc::new(HttpClientWithUrl::new(inner_http.clone(), "https://api.vscodium-rust.com", None));
            let clock = Arc::new(RealSystemClock);
            (inner_http, http_client, clock)
        });
        
        cx.update(|cx| {
            release_channel::init(Version::new(0, 1, 0), cx);
            settings::init(cx);
        });
        cx.background_executor().timer(std::time::Duration::from_millis(1)).await;
        
        let client = cx.update(|cx| {
            client::Client::new(clock, http_client.clone(), cx)
        });
        cx.background_executor().timer(std::time::Duration::from_millis(1)).await;

        let user_store = cx.update(|cx| {
            cx.new(|cx| UserStore::new(client.clone(), cx))
        });
        cx.background_executor().timer(std::time::Duration::from_millis(1)).await;

        let node_runtime = cx.update(|_cx| {
            NodeRuntime::new(inner_http.clone(), None, watch::Receiver::constant(None))
        });

        Self {
            client,
            user_store,
            node_runtime,
            languages: Arc::new(LanguageRegistry::new(cx.background_executor().clone())),
            fs: Arc::new(RealFs::new(None, cx.background_executor().clone())), // fs needs to be initialized here
            current_project: None,
        }
    }
}

#[async_trait(?Send)]
impl ProjectRepository for ZedProjectRepository {
    async fn open_project(&mut self, cx: &mut AsyncApp, path: PathBuf) -> Result<(), String> {
        let project_handle = cx.update(|cx| {
            project::Project::local(
                self.client.clone(),
                self.node_runtime.clone(),
                self.user_store.clone(),
                self.languages.clone(),
                self.fs.clone(),
                None,
                project::LocalProjectFlags::default(),
                cx,
            )
        });

        let result_future = project_handle.update(cx, |p, cx| {
            p.find_or_create_worktree(&path, true, cx)
        });

        match result_future.await {
            Ok(_) => {
                self.current_project = Some(project_handle);
                Ok(())
            }
            Err(e) => Err(format!("Failed to create worktree: {}", e)),
        }
    }

    async fn get_file_tree(&self, cx: &mut AsyncApp) -> Result<Vec<FileEntry>, String> {
        let project_handle = self.current_project.as_ref()
            .ok_or_else(|| "No project open".to_string())?;

        let roots: Vec<FileEntry> = cx.update(|cx| {
            let project = project_handle.read(cx);
            let mut all_roots = Vec::new();

            for worktree in project.worktrees(cx) {
                let worktree = worktree.read(cx);
                let snapshot = worktree.snapshot();
                
                use std::collections::HashMap;
                let mut nodes: HashMap<PathBuf, FileEntry> = HashMap::new();
                let entries = snapshot.entries(false, 0);
                const MAX_ENTRIES: usize = 10_000;
                
                let mut count = 0;
                for entry in entries {
                    count += 1;
                    if count > MAX_ENTRIES {
                        break;
                    }
                    let rel_path = entry.path.as_ref();
                    let abs_path = snapshot.absolutize(rel_path);
                    let name = if rel_path.is_empty() {
                        snapshot.root_name_str().to_string()
                    } else {
                        rel_path.file_name().unwrap_or("").to_string()
                    };
                    
                    nodes.insert(abs_path.clone(), FileEntry {
                        name,
                        path: abs_path.to_string_lossy().to_string(),
                        is_dir: entry.is_dir(),
                        children: if entry.is_dir() { Some(Vec::new()) } else { None },
                    });
                }

                let mut paths: Vec<_> = nodes.keys().cloned().collect();
                paths.sort_by_key(|p| p.components().count());
                paths.reverse();

                let mut worktree_roots = Vec::new();
                for path in paths {
                    let mut entry = nodes.remove(&path).unwrap();
                    if let Some(children) = &mut entry.children {
                        children.sort_by(|a, b| {
                            if a.is_dir != b.is_dir {
                                b.is_dir.cmp(&a.is_dir) 
                            } else {
                                a.name.cmp(&b.name)
                            }
                        });
                    }

                    if let Some(parent_path) = path.parent() {
                        if let Some(parent_entry) = nodes.get_mut(parent_path) {
                            if let Some(children) = &mut parent_entry.children {
                                children.push(entry);
                            }
                            continue;
                        }
                    }
                    worktree_roots.push(entry);
                }
                all_roots.extend(worktree_roots);
            }
            all_roots
        });
        
        Ok(roots)
    }
}
