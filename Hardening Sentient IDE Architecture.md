# Chat Conversation

Note: _This is purely the output of the chat conversation and does not contain any raw data, codebase snippets, etc. used to generate the output._

*Viewed [context_indexer.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/context_indexer.rs) *

*Edited relevant file*

*Edited relevant file*

*User accepted the command `cargo check`*

*Checked command status*

*Viewed [context_indexer.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/context_indexer.rs) *

*Checked command status*

### User Input

C:\Users\HADES\Desktop\vscodium-rust\src-tauri>cargo check
    Checking vscode-rust-app v0.1.0 (C:\Users\HADES\Desktop\vscodium-rust\src-tauri)
error[E0405]: cannot find trait `Write` in this scope
   --> src\lib.rs:121:53
    |
121 |     terminal_writers: Mutex<HashMap<String, Box<dyn Write + Send>>>,
    |                                                     ^^^^^ not found in this scope
    |
help: consider importing one of these traits
    |
  1 + use std::fmt::Write;
    |
  1 + use std::io::Write;
    |
  1 + use core::fmt::Write;
    |

error[E0425]: cannot find type `PathBuf` in this scope
   --> src\lib.rs:132:17
    |
132 |     config_dir: PathBuf,
    |                 ^^^^^^^ not found in this scope
    |
help: consider importing this struct
    |
  1 + use std::path::PathBuf;
    |

error[E0425]: cannot find type `PathBuf` in this scope
   --> src\lib.rs:133:31
    |
133 |     active_root: Mutex<Option<PathBuf>>,
    |                               ^^^^^^^ not found in this scope
    |
help: consider importing this struct
    |
  1 + use std::path::PathBuf;
    |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
   --> src\lib.rs:177:33
    |
177 |             .unwrap_or_else(|_| PathBuf::from(".config"));
    |                                 ^^^^^^^ use of undeclared type `PathBuf`
    |
help: consider importing this struct
    |
  1 + use std::path::PathBuf;
    |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
   --> src\lib.rs:183:67
    |
183 |         let mut root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    |                                                                   ^^^^^^^ use of undeclared type `PathBuf`
    |
help: consider importing this struct
    |
  1 + use std::path::PathBuf;
    |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
   --> src\lib.rs:351:49
    |
351 |     state.ai_engine.memory_store.load_from_path(PathBuf::from(path)).await;
    |                                                 ^^^^^^^ use of undeclared type `PathBuf`
    |
help: consider importing this struct
    |
  1 + use std::path::PathBuf;
    |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
   --> src\lib.rs:487:44
    |
487 |                 u.to_file_path().unwrap_or(PathBuf::from(u.path()))
    |                                            ^^^^^^^ use of undeclared type `PathBuf`
    |
help: consider importing this struct
    |
  1 + use std::path::PathBuf;
    |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:1038:24
     |
1038 |         let path_buf = PathBuf::from(p);
     |                        ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0425]: cannot find type `PathBuf` in this scope
    --> src\lib.rs:1080:46
     |
1080 | fn is_path_valid(state: &EditorState, path: &PathBuf) -> Result<(), String> {
     |                                              ^^^^^^^ not found in this scope
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0425]: cannot find type `PathBuf` in this scope
    --> src\lib.rs:1093:55
     |
1093 | fn validate_path(state: State<'_, EditorState>, path: PathBuf) -> Result<(), String> {
     |                                                       ^^^^^^^ not found in this scope
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0425]: cannot find type `PathBuf` in this scope
    --> src\lib.rs:1113:24
     |
1113 | fn list_dir_flat(path: PathBuf) -> Result<Vec<FileEntry>, String> {
     |                        ^^^^^^^ not found in this scope
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:1174:20
     |
1174 |     let path_buf = PathBuf::from(&path);
     |                    ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:1181:20
     |
1181 |     let path_buf = PathBuf::from(&path);
     |                    ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:1192:20
     |
1192 |     let path_buf = PathBuf::from(&path);
     |                    ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:1207:28
     |
1207 |         .unwrap_or_else(|| PathBuf::from("."));
     |                            ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `Command`
    --> src\lib.rs:1270:18
     |
1270 |     let output = Command::new("git")
     |                  ^^^^^^^ use of undeclared type `Command`
     |
help: consider importing one of these structs
     |
   1 + use std::process::Command;
     |
   1 + use lsp_types::Command;
     |
   1 + use tauri_plugin_shell::process::Command;
     |
   1 + use tokio::process::Command;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:1300:28
     |
1300 |         .unwrap_or_else(|| PathBuf::from("."));
     |                            ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:1383:9
     |
1383 |         PathBuf::from(p)
     |         ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:1385:69
     |
1385 |         state.active_root.lock().unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
     |                                                                     ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:1418:9
     |
1418 |         PathBuf::from(p)
     |         ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:1420:69
     |
1420 |         state.active_root.lock().unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
     |                                                                     ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:1492:13
     |
1492 |     let p = PathBuf::from(&path);
     |             ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:2095:31
     |
2095 |     let working_dir = cwd.map(PathBuf::from).unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
     |                               ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:2141:20
     |
2141 |     let path_buf = PathBuf::from(&path);
     |                    ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:2174:20
     |
2174 |     let path_buf = PathBuf::from(&path);
     |                    ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
    --> src\lib.rs:2183:20
     |
2183 |     let path_buf = PathBuf::from(&path);
     |                    ^^^^^^^ use of undeclared type `PathBuf`
     |
help: consider importing this struct
     |
   1 + use std::path::PathBuf;
     |

Some errors have detailed explanations: E0405, E0425, E0433.
For more information about an error, try `rustc --explain E0405`.
error: could not compile `vscode-rust-app` (lib) due to 26 previous errors

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [task.md](file:///C:/Users/HADES/.gemini/antigravity/brain/17dc6d5a-04e0-4580-8f68-2bc579d661d9/task.md) *

*Viewed [implementation_plan.md](file:///C:/Users/HADES/.gemini/antigravity/brain/17dc6d5a-04e0-4580-8f68-2bc579d661d9/implementation_plan.md) *

*Edited relevant file*

*Viewed [context_indexer.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/context_indexer.rs) *

*Edited relevant file*

*User accepted the command `cargo check`*

*Checked command status*

*Checked command status*

*Checked command status*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Checked command status*

### User Input

fix the errors and warnings properly, not a fucking band aid solution.

C:\Users\HADES\Desktop\vscodium-rust\src-tauri>cargo check
    Checking vscode-rust-app v0.1.0 (C:\Users\HADES\Desktop\vscodium-rust\src-tauri)
warning: unused import: `Path`
 --> src\lib.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

error: future cannot be sent between threads safely
    --> src\ai_engine.rs:1096:25
     |
1096 | /                         tauri::async_runtime::spawn(async move {
1097 | |                             let _ = h_arc.execute_agent_step("Final Mission Review").await;
1098 | |                         });
     | |__________________________^ future created by async block is not `Send`
     |
     = note: cannot satisfy `impl futures::Future<Output = Result<std::string::String, anyhow::Error>>: std::marker::Send`
note: future is not `Send` as it awaits another future which is not `Send`
    --> src\ai_engine.rs:402:22
     |
 402 |         let result = self.autonomous_loop(req, on_chunk).await?;
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here on type `impl futures::Future<Output = Result<std::string::String, anyhow::Error>>`, which is not `Send`
note: required by a bound in `tauri::async_runtime::spawn`
    --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tauri-2.10.3\src\async_runtime.rs:270:15
     |
 268 | pub fn spawn<F>(task: F) -> JoinHandle<F::Output>
     |        ----- required by a bound in this function
 269 | where
 270 |   F: Future + Send + 'static,
     |               ^^^^ required by this bound in `spawn`

warning: variable does not need to be mutable
   --> src\lib.rs:251:17
    |
251 |             let mut ci = ci_for_spawn.lock().unwrap();
    |                 ----^^
    |                 |
    |                 help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default

error: future cannot be sent between threads safely
   --> src\lib.rs:248:9
    |
248 | /         tauri::async_runtime::spawn(async move {
249 | |             // We do NOT hold the lock across the await.
250 | |             // The method itself should handle its own locking if needed.
251 | |             let mut ci = ci_for_spawn.lock().unwrap();
252 | |             ci.start_background_indexing().await;
253 | |         });
    | |__________^ future created by async block is not `Send`
    |
    = help: within `{async block@src\lib.rs:248:37: 248:47}`, the trait `std::marker::Send` is not implemented for `std::sync::MutexGuard<'_, ContextIndexer>`
note: future is not `Send` as this value is used across an await
   --> src\lib.rs:252:44
    |
251 |             let mut ci = ci_for_spawn.lock().unwrap();
    |                 ------ has type `std::sync::MutexGuard<'_, ContextIndexer>` which is not `Send`
252 |             ci.start_background_indexing().await;
    |                                            ^^^^^ await occurs here, with `mut ci` maybe used later
note: required by a bound in `tauri::async_runtime::spawn`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tauri-2.10.3\src\async_runtime.rs:270:15
    |
268 | pub fn spawn<F>(task: F) -> JoinHandle<F::Output>
    |        ----- required by a bound in this function
269 | where
270 |   F: Future + Send + 'static,
    |               ^^^^ required by this bound in `spawn`

warning: `vscode-rust-app` (lib) generated 2 warnings
error: could not compile `vscode-rust-app` (lib) due to 2 previous errors; 2 warnings emitted

C:\Users\HADES\Desktop\vscodium-rust\src-tauri>

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Edited relevant file*

*Edited relevant file*

*Grep searched codebase*

*Edited relevant file*

*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

*Edited relevant file*

*Grep searched codebase*

*User accepted the command `grep -n ".lock().unwrap()" src\ai_engine.rs`*

*Checked command status*

*Viewed [task.md](file:///C:/Users/HADES/.gemini/antigravity/brain/17dc6d5a-04e0-4580-8f68-2bc579d661d9/task.md) *

*Checked command status*

### User Input

C:\Users\HADES\Desktop\vscodium-rust\src-tauri>cargo check
    Checking vscode-rust-app v0.1.0 (C:\Users\HADES\Desktop\vscodium-rust\src-tauri)
warning: unused import: `Path`
 --> src\lib.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\ai_engine.rs:385:44
    |
385 |             let u = self.ollama_url.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\ai_engine.rs:457:44
    |
457 |             let u = self.ollama_url.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\ai_engine.rs:473:44
    |
473 |             let u = self.ollama_url.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\ai_engine.rs:515:61
    |
515 |             let mut cache = self.project_files_cache.lock().unwrap();
    |                                                             ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
515 |             let mut cache = self.project_files_cache.lock().await.unwrap();
    |                                                             ++++++

error[E0282]: type annotations needed
   --> src\ai_engine.rs:517:17
    |
517 |                 c.clone()
    |                 ^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\ai_engine.rs:537:64
    |
537 |             let mut cache = self.workspace_memory_cache.lock().unwrap();
    |                                                                ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
537 |             let mut cache = self.workspace_memory_cache.lock().await.unwrap();
    |                                                                ++++++

error[E0282]: type annotations needed
   --> src\ai_engine.rs:539:17
    |
539 |                 c.clone()
    |                 ^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\ai_engine.rs:558:60
    |
558 |             let mut cache = self.global_brain_cache.lock().unwrap();
    |                                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
558 |             let mut cache = self.global_brain_cache.lock().await.unwrap();
    |                                                            ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\ai_engine.rs:642:65
    |
642 |                         let current = self.advisor_model.lock().unwrap();
    |                                                                 ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
642 |                         let current = self.advisor_model.lock().await.unwrap();
    |                                                                 ++++++

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src\ai_engine.rs:844:17
    |
842 | ...   "\n### DYNAMIC ENVIRONMENT CONTEXT:\n- **Current OS**: {}\n- **Project Root**: {}\n- **Timestamp**: {}\n- **File System Awareness**: You are empowered to use `list_files` and `searc...
    |                                                                                      -- required by this formatting parameter
843 | ...   std::env::consts::OS,
844 | ...   project_path,
    |       ^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `Sized` is not implemented for `str`
    = note: this error originates in the macro `$crate::__export::format_args` which comes from the expansion of the macro `format` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\ai_engine.rs:898:57
    |
898 |                 let advisor = self.advisor_model.lock().unwrap();
    |                                                         ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
898 |                 let advisor = self.advisor_model.lock().await.unwrap();
    |                                                         ++++++

error[E0282]: type annotations needed
   --> src\ai_engine.rs:904:36
    |
904 |                     active_model = model.clone();
    |                                    ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_engine.rs:1086:44
     |
1086 |                     let h_arc_opt = if let Ok(h_lock) = self.app_handle.lock() {
     |                                            ^^^^^^^^^^   ---------------------- this expression has type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
     |                                            |
     |                                            expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`

error[E0282]: type annotations needed
    --> src\ai_engine.rs:1087:46
     |
1087 |                         h_lock.as_ref().map(|h| {
     |                                              ^
1088 |                             let state: tauri::State<crate::EditorState> = h.state();
     |                                                                           - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
1087 |                         h_lock.as_ref().map(|h: /* Type */| {
     |                                               ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_engine.rs:1095:37
     |
1095 |                         let h_arc = h_arc.clone();
     |                                     ^^^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
    --> src\ai_engine.rs:1716:47
     |
1716 |             let base = self.ollama_url.lock().unwrap().clone();
     |                                               ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
    --> src\ai_engine.rs:2065:54
     |
2065 |             let cache = self.memory_aim_cache.lock().unwrap();
     |                                                      ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2065 |             let cache = self.memory_aim_cache.lock().await.unwrap();
     |                                                      ++++++

error[E0282]: type annotations needed
    --> src\ai_engine.rs:2067:24
     |
2067 |                 return c.clone();
     |                        ^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
    --> src\ai_engine.rs:2122:58
     |
2122 |             let mut cache = self.memory_aim_cache.lock().unwrap();
     |                                                          ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2122 |             let mut cache = self.memory_aim_cache.lock().await.unwrap();
     |                                                          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
   --> src\lib.rs:495:49
    |
495 |         let mut root = state.active_root.lock().unwrap();
    |                                                 ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
495 |         let mut root = state.active_root.lock().await.unwrap();
    |                                                 ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:606:40
    |
606 |         let eh = state.ext_host.lock().unwrap();
    |                                        ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:636:48
    |
636 |             let mut eh = state.ext_host.lock().unwrap();
    |                                                ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:672:40
    |
672 |     let mut eh = state.ext_host.lock().unwrap();
    |                                        ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1163:51
     |
1163 |         let root_guard = state.active_root.lock().unwrap();
     |                                                   ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1163 |         let root_guard = state.active_root.lock().await.unwrap();
     |                                                   ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
    --> src\lib.rs:1861:44
     |
1861 |         let m = state.current_model.lock().unwrap();
     |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
    --> src\lib.rs:1885:41
     |
1885 |         let u = state.ollama_url.lock().unwrap();
     |                                         ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PatchEngine>>` in the current scope
    --> src\lib.rs:2182:48
     |
2182 |     let mut engine = state.patch_engine.lock().map_err(|e| e.to_string())?;
     |                                                ^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PatchEngine>>`

error[E0282]: type annotations needed
    --> src\lib.rs:2182:57
     |
2182 |     let mut engine = state.patch_engine.lock().map_err(|e| e.to_string())?;
     |                                                         ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2182 |     let mut engine = state.patch_engine.lock().map_err(|e: /* Type */| e.to_string())?;
     |                                                          ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2183:46
     |
2183 |     engine.commit_shadow(&path_buf).map_err(|e| e.to_string())
     |                                              ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2183 |     engine.commit_shadow(&path_buf).map_err(|e: /* Type */| e.to_string())
     |                                               ++++++++++++

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PatchEngine>>` in the current scope
    --> src\lib.rs:2191:48
     |
2191 |     let mut engine = state.patch_engine.lock().map_err(|e| e.to_string())?;
     |                                                ^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PatchEngine>>`

error[E0282]: type annotations needed
    --> src\lib.rs:2191:57
     |
2191 |     let mut engine = state.patch_engine.lock().map_err(|e| e.to_string())?;
     |                                                         ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2191 |     let mut engine = state.patch_engine.lock().map_err(|e: /* Type */| e.to_string())?;
     |                                                          ++++++++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
    --> src\lib.rs:2691:51
     |
2691 |         let mut current = state.ollama_url.lock().unwrap();
     |                                                   ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2831:10
     |
2828 |       let root = state
     |  ________________-
2829 | |         .active_root
2830 | |         .lock()
2831 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     | |_________|
     |
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2831 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2842:10
     |
2839 |       let root = state
     |  ________________-
2840 | |         .active_root
2841 | |         .lock()
2842 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     | |_________|
     |
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2842 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2853:10
     |
2850 |       let root = state
     |  ________________-
2851 | |         .active_root
2852 | |         .lock()
2853 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     | |_________|
     |
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2853 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2864:10
     |
2861 |       let root = state
     |  ________________-
2862 | |         .active_root
2863 | |         .lock()
2864 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     | |_________|
     |
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2864 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\ai_engine.rs:256:44
    |
256 |         let mut u = self.ollama_url.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\ai_engine.rs:261:47
    |
261 |         let mut m = self.advisor_model.lock().unwrap();
    |                                               ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
261 |         let mut m = self.advisor_model.lock().await.unwrap();
    |                                               ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>` in the current scope
   --> src\ai_engine.rs:270:44
    |
270 |         let mut h = self.app_handle.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
270 |         let mut h = self.app_handle.lock().await.unwrap();
    |                                            ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>` in the current scope
    --> src\ai_engine.rs:2252:54
     |
2252 |         if let Some(handle) = self.app_handle.lock().unwrap().as_ref() {
     |                                                      ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2252 |         if let Some(handle) = self.app_handle.lock().await.unwrap().as_ref() {
     |                                                      ++++++

error[E0282]: type annotations needed
    --> src\ai_engine.rs:2253:21
     |
2253 |             let _ = handle.emit(event, payload);
     |                     ^^^^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>` in the current scope
    --> src\ai_tools.rs:2088:61
     |
2088 |         let mut processes = state.terminal_processes.lock().unwrap();
     |                                                             ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-9788565293280801661.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2090:21
     |
2090 |             let _ = child.kill();
     |                     ^^^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn MasterPty + Send>>>>` in the current scope
    --> src\ai_tools.rs:2091:43
     |
2091 |             state.terminal_masters.lock().unwrap().remove(term_id);
     |                                           ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn MasterPty + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-16463395251749176000.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>` in the current scope
    --> src\ai_tools.rs:2092:43
     |
2092 |             state.terminal_writers.lock().unwrap().remove(term_id);
     |                                           ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-17153415940817161940.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>` in the current scope
    --> src\ai_tools.rs:2113:61
     |
2113 |         let mut processes = state.terminal_processes.lock().unwrap();
     |                                                             ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-9788565293280801661.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2115:19
     |
2115 |             match child.try_wait() {
     |                   ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2117:57
     |
2117 |                     json!({ "active": false, "success": status.success(), "status": if status.success() { "success" } else { "failed" } }),
     |                                                         ^^^^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
    --> src\ai_tools.rs:2446:52
     |
2446 |         let active_path = state.active_path.lock().unwrap().clone();
     |                                                    ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2446 |         let active_path = state.active_path.lock().await.unwrap().clone();
     |                                                    ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>` in the current scope
    --> src\ai_tools.rs:2447:57
     |
2447 |         let terminals = state.terminal_processes.lock().unwrap().keys().cloned().collect::<Vec<String>>();
     |                                                         ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2544545850145924170.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>` in the current scope
    --> src\ai_tools.rs:2893:57
     |
2893 |         let mut writers = state.terminal_writers.lock().unwrap();
     |                                                         ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13638165419078577780.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>` in the current scope
    --> src\ai_tools.rs:2900:53
     |
2900 |             writers = state.terminal_writers.lock().unwrap();
     |                                                     ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-4517963720717954487.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2916:17
     |
2916 |                 writer.write_all(payload.as_bytes())?;
     |                 ^^^^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>` in the current scope
    --> src\ai_tools.rs:2944:53
     |
2944 |         let writers = state.terminal_writers.lock().unwrap();
     |                                                     ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2346182610915678317.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Vec<std::string::String>>>>` in the current scope
    --> src\ai_tools.rs:2996:58
     |
2996 |         let term_buffers = state.terminal_buffers.lock().unwrap();
     |                                                          ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Vec<std::string::String>>>>`

error[E0282]: type annotations needed for `(_, _)`
    --> src\ai_tools.rs:3004:24
     |
3004 |                 .find(|(_, buf)| !buf.is_empty())
     |                        ^^^^^^^^   --- type must be known at this point
     |
help: consider giving this closure parameter an explicit type, where the placeholders `_` are specified
     |
3004 |                 .find(|(_, buf): (_, _)| !buf.is_empty())
     |                                ++++++++

error[E0282]: type annotations needed for `(_, _)`
    --> src\ai_tools.rs:3005:23
     |
3005 |                 .map(|(id, _)| id.clone())
     |                       ^^^^^^^  -- type must be known at this point
     |
help: consider giving this closure parameter an explicit type, where the placeholders `_` are specified
     |
3005 |                 .map(|(id, _): (_, _)| id.clone())
     |                              ++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3012:31
     |
3012 |                     "output": buffer.join("")
     |                               ^^^^^^ cannot infer type

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
    --> src\ai_tools.rs:3077:18
     |
3074 |               let active_path = state
     |  _______________________________-
3075 | |                 .active_path
3076 | |                 .lock()
3077 | |                 .map_err(|_| anyhow!("Lock error"))?;
     | |                 -^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
     | |_________________|
     |

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Vec<std::string::String>>>>` in the current scope
   --> src\lib.rs:166:52
    |
166 |         let buffers = self.terminal_buffers.lock().map_err(|e| e.to_string())?;
    |                                                    ^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Vec<std::string::String>>>>`

error[E0282]: type annotations needed
   --> src\lib.rs:166:61
    |
166 |         let buffers = self.terminal_buffers.lock().map_err(|e| e.to_string())?;
    |                                                             ^  - type must be known at this point
    |
help: consider giving this closure parameter an explicit type
    |
166 |         let buffers = self.terminal_buffers.lock().map_err(|e: /* Type */| e.to_string())?;
    |                                                              ++++++++++++

error[E0277]: `Result<std::sync::MutexGuard<'_, PatchEngine>, PoisonError<std::sync::MutexGuard<'_, PatchEngine>>>` is not a future
   --> src\lib.rs:243:23
    |
243 |             pe.lock().await.set_app_handle(app_clone);
    |                       ^^^^^ `Result<std::sync::MutexGuard<'_, PatchEngine>, PoisonError<std::sync::MutexGuard<'_, PatchEngine>>>` is not a future
    |
    = help: the trait `futures::Future` is not implemented for `Result<std::sync::MutexGuard<'_, PatchEngine>, PoisonError<std::sync::MutexGuard<'_, PatchEngine>>>`
    = note: Result<std::sync::MutexGuard<'_, PatchEngine>, PoisonError<std::sync::MutexGuard<'_, PatchEngine>>> must be a future or must implement `IntoFuture` to be awaited
    = note: required for `Result<std::sync::MutexGuard<'_, PatchEngine>, PoisonError<std::sync::MutexGuard<'_, PatchEngine>>>` to implement `std::future::IntoFuture`
help: remove the `.await`
    |
243 -             pe.lock().await.set_app_handle(app_clone);
243 +             pe.lock().set_app_handle(app_clone);
    |

error[E0282]: type annotations needed
   --> src\lib.rs:243:13
    |
243 |             pe.lock().await.set_app_handle(app_clone);
    |             ^^^^^^^^^^^^^^^ cannot infer type

error[E0308]: mismatched types
   --> src\lib.rs:274:13
    |
273 |         let mut kairos = kairos::KairosEngine::new(
    |                          ------------------------- arguments to this function are incorrect
274 |             context_indexer.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^^^^ expected `std::sync::Mutex<ContextIndexer>`, found `tauri::async_runtime::Mutex<ContextIndexer>`
    |
    = note: `tauri::async_runtime::Mutex<ContextIndexer>` and `std::sync::Mutex<ContextIndexer>` have similar names, but are actually distinct types
note: `tauri::async_runtime::Mutex<ContextIndexer>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `std::sync::Mutex<ContextIndexer>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: associated function defined here
   --> src\kairos.rs:19:12
    |
 19 |     pub fn new(
    |            ^^^
 20 |         indexer: Arc<Mutex<ContextIndexer>>,
    |         -----------------------------------

error[E0308]: mismatched types
   --> src\lib.rs:293:22
    |
293 |             buffers: Mutex::new(HashMap::new()),
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Mutex<HashMap<String, Rope>>`, found `Mutex<HashMap<_, _>>`
    |
    = note: `std::sync::Mutex<HashMap<_, _>>` and `Mutex<HashMap<String, Rope>>` have similar names, but are actually distinct types
note: `std::sync::Mutex<HashMap<_, _>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<HashMap<String, Rope>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:294:26
    |
294 |             active_path: Mutex::new(None),
    |                          ^^^^^^^^^^^^^^^^ expected `Mutex<Option<String>>`, found `Mutex<Option<_>>`
    |
    = note: `std::sync::Mutex<std::option::Option<_>>` and `Mutex<Option<String>>` have similar names, but are actually distinct types
note: `std::sync::Mutex<std::option::Option<_>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<Option<String>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:295:23
    |
295 |               settings: Mutex::new(Settings {
    |  _______________________^
296 | |                 theme: "vs-dark".to_string(),
297 | |                 font_size: 14,
298 | |             }),
    | |______________^ expected `tauri::async_runtime::Mutex<Settings>`, found `std::sync::Mutex<Settings>`
    |
    = note: `std::sync::Mutex<Settings>` and `tauri::async_runtime::Mutex<Settings>` have similar names, but are actually distinct types
note: `std::sync::Mutex<Settings>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<Settings>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:299:31
    |
299 |             terminal_masters: Mutex::new(HashMap::new()),
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Mutex<HashMap<String, Box<dyn MasterPty + Send>>>`, found `Mutex<HashMap<_, _>>`
    |
    = note: `std::sync::Mutex<HashMap<_, _>>` and `Mutex<HashMap<String, Box<dyn MasterPty + Send>>>` have similar names, but are actually distinct types
note: `std::sync::Mutex<HashMap<_, _>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<HashMap<String, Box<dyn MasterPty + Send>>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:300:31
    |
300 |             terminal_writers: Mutex::new(HashMap::new()),
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Mutex<HashMap<String, Box<dyn Write + Send>>>`, found `Mutex<HashMap<_, _>>`
    |
    = note: `std::sync::Mutex<HashMap<_, _>>` and `Mutex<HashMap<String, Box<dyn Write + Send>>>` have similar names, but are actually distinct types
note: `std::sync::Mutex<HashMap<_, _>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<HashMap<String, Box<dyn Write + Send>>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:301:33
    |
301 |             terminal_processes: Mutex::new(HashMap::new()),
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Mutex<HashMap<String, Box<dyn Child + Send>>>`, found `Mutex<HashMap<_, _>>`
    |
    = note: `std::sync::Mutex<HashMap<_, _>>` and `Mutex<HashMap<String, Box<dyn Child + Send>>>` have similar names, but are actually distinct types
note: `std::sync::Mutex<HashMap<_, _>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<HashMap<String, Box<dyn Child + Send>>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:302:34
    |
302 |             lsp_client: Arc::new(Mutex::new(LspClient::new())),
    |                         -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<LspClient>`, found `std::sync::Mutex<LspClient>`
    |                         |
    |                         arguments to this function are incorrect
    |
    = note: `std::sync::Mutex<LspClient>` and `tauri::async_runtime::Mutex<LspClient>` have similar names, but are actually distinct types
note: `std::sync::Mutex<LspClient>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<LspClient>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: associated function defined here
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\alloc\src\sync.rs:419:11

error[E0308]: mismatched types
   --> src\lib.rs:304:32
    |
304 |             ext_host: Arc::new(Mutex::new(ExtensionHostManager::new(ext_dirs))),
    |                       -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<ExtensionHostManager>`, found `std::sync::Mutex<ExtensionHostManager>`
    |                       |
    |                       arguments to this function are incorrect
    |
    = note: `std::sync::Mutex<ExtensionHostManager>` and `tauri::async_runtime::Mutex<ExtensionHostManager>` have similar names, but are actually distinct types
note: `std::sync::Mutex<ExtensionHostManager>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<ExtensionHostManager>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: associated function defined here
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\alloc\src\sync.rs:419:11

error[E0308]: mismatched types
   --> src\lib.rs:305:35
    |
305 |             keybindings: Arc::new(Mutex::new(KeybindingRegistry::new())),
    |                          -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<KeybindingRegistry>`, found `std::sync::Mutex<KeybindingRegistry>`
    |                          |
    |                          arguments to this function are incorrect
    |
    = note: `std::sync::Mutex<KeybindingRegistry>` and `tauri::async_runtime::Mutex<KeybindingRegistry>` have similar names, but are actually distinct types
note: `std::sync::Mutex<KeybindingRegistry>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<KeybindingRegistry>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: associated function defined here
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\alloc\src\sync.rs:419:11

error[E0308]: mismatched types
   --> src\lib.rs:306:37
    |
306 |             debug_manager: Arc::new(Mutex::new(DebugManager::new())),
    |                            -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<DebugManager>`, found `std::sync::Mutex<DebugManager>`
    |                            |
    |                            arguments to this function are incorrect
    |
    = note: `std::sync::Mutex<DebugManager>` and `tauri::async_runtime::Mutex<DebugManager>` have similar names, but are actually distinct types
note: `std::sync::Mutex<DebugManager>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<DebugManager>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: associated function defined here
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\alloc\src\sync.rs:419:11

error[E0308]: mismatched types
   --> src\lib.rs:307:42
    |
307 |             activation_manager: Arc::new(Mutex::new(ActivationManager::new())),
    |                                 -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<ActivationManager>`, found `std::sync::Mutex<ActivationManager>`
    |                                 |
    |                                 arguments to this function are incorrect
    |
    = note: `std::sync::Mutex<ActivationManager>` and `tauri::async_runtime::Mutex<ActivationManager>` have similar names, but are actually distinct types
note: `std::sync::Mutex<ActivationManager>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<ActivationManager>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: associated function defined here
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\alloc\src\sync.rs:419:11

error[E0308]: mismatched types
   --> src\lib.rs:310:25
    |
310 |             ollama_url: Mutex::new("http://127.0.0.1:1536".to_string()),
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<std::string::String>`, found `std::sync::Mutex<std::string::String>`
    |
    = note: `std::sync::Mutex<std::string::String>` and `tauri::async_runtime::Mutex<std::string::String>` have similar names, but are actually distinct types
note: `std::sync::Mutex<std::string::String>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<std::string::String>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:312:26
    |
312 |             active_root: Mutex::new(Some(root)),
    |                          ^^^^^^^^^^^^^^^^^^^^^^ expected `Mutex<Option<PathBuf>>`, found `std::sync::Mutex<std::option::Option<PathBuf>>`
    |
    = note: `std::sync::Mutex<std::option::Option<PathBuf>>` and `Mutex<Option<PathBuf>>` have similar names, but are actually distinct types
note: `std::sync::Mutex<std::option::Option<PathBuf>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<Option<PathBuf>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:313:28
    |
313 |             current_model: Mutex::new("gpt-4o".to_string()),
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<std::string::String>`, found `std::sync::Mutex<std::string::String>`
    |
    = note: `std::sync::Mutex<std::string::String>` and `tauri::async_runtime::Mutex<std::string::String>` have similar names, but are actually distinct types
note: `std::sync::Mutex<std::string::String>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<std::string::String>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:314:28
    |
314 |             active_device: Mutex::new(None),
    |                            ^^^^^^^^^^^^^^^^ expected `Mutex<Option<String>>`, found `Mutex<Option<_>>`
    |
    = note: `std::sync::Mutex<std::option::Option<_>>` and `Mutex<Option<String>>` have similar names, but are actually distinct types
note: `std::sync::Mutex<std::option::Option<_>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<Option<String>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:315:31
    |
315 |             android_sdk_path: Mutex::new(None),
    |                               ^^^^^^^^^^^^^^^^ expected `Mutex<Option<String>>`, found `Mutex<Option<_>>`
    |
    = note: `std::sync::Mutex<std::option::Option<_>>` and `Mutex<Option<String>>` have similar names, but are actually distinct types
note: `std::sync::Mutex<std::option::Option<_>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<Option<String>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:319:31
    |
319 |             terminal_buffers: Mutex::new(HashMap::new()),
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Mutex<HashMap<String, Vec<String>>>`, found `Mutex<HashMap<_, _>>`
    |
    = note: `std::sync::Mutex<HashMap<_, _>>` and `Mutex<HashMap<String, Vec<String>>>` have similar names, but are actually distinct types
note: `std::sync::Mutex<HashMap<_, _>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<HashMap<String, Vec<String>>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:321:28
    |
321 |             advisor_model: Mutex::new(None),
    |                            ^^^^^^^^^^^^^^^^ expected `Mutex<Option<String>>`, found `Mutex<Option<_>>`
    |
    = note: `std::sync::Mutex<std::option::Option<_>>` and `Mutex<Option<String>>` have similar names, but are actually distinct types
note: `std::sync::Mutex<std::option::Option<_>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<Option<String>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0308]: mismatched types
   --> src\lib.rs:326:13
    |
326 |             patch_engine,
    |             ^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<PatchEngine>`, found `std::sync::Mutex<PatchEngine>`
    |
    = note: `std::sync::Mutex<PatchEngine>` and `tauri::async_runtime::Mutex<PatchEngine>` have similar names, but are actually distinct types
note: `std::sync::Mutex<PatchEngine>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<PatchEngine>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Rope>>>` in the current scope
   --> src\lib.rs:414:44
    |
414 |     let mut buffers = state.buffers.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Rope>>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\lib.rs:417:47
    |
417 |     let mut active = state.active_path.lock().unwrap();
    |                                               ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
417 |     let mut active = state.active_path.lock().await.unwrap();
    |                                               ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Rope>>>` in the current scope
   --> src\lib.rs:426:44
    |
426 |     let mut buffers = state.buffers.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Rope>>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Rope>>>` in the current scope
   --> src\lib.rs:505:40
    |
505 |     let buffers = state.buffers.lock().unwrap();
    |                                        ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Rope>>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\lib.rs:507:51
    |
507 |         let mut active = state.active_path.lock().unwrap();
    |                                                   ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
507 |         let mut active = state.active_path.lock().await.unwrap();
    |                                                   ++++++

error[E0282]: type annotations needed
   --> src\lib.rs:509:12
    |
509 |         Ok(rope.to_string())
    |            ^^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, Settings>>` in the current scope
   --> src\lib.rs:517:27
    |
517 |     state.settings.lock().unwrap().clone()
    |                           ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, Settings>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, Settings>>` in the current scope
   --> src\lib.rs:522:39
    |
522 |     let mut s = state.settings.lock().unwrap();
    |                                       ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, Settings>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, LspClient>>` in the current scope
   --> src\lib.rs:532:43
    |
532 |     let mut lsp = state.lsp_client.lock().unwrap();
    |                                           ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, LspClient>>`

error[E0282]: type annotations needed
   --> src\lib.rs:533:39
    |
533 |     lsp.start(&command, app).map_err(|e| e.to_string())
    |                                       ^  - type must be known at this point
    |
help: consider giving this closure parameter an explicit type
    |
533 |     lsp.start(&command, app).map_err(|e: /* Type */| e.to_string())
    |                                        ++++++++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, LspClient>>` in the current scope
   --> src\lib.rs:543:43
    |
543 |     let mut lsp = state.lsp_client.lock().unwrap();
    |                                           ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, LspClient>>`

error[E0282]: type annotations needed
   --> src\lib.rs:545:19
    |
545 |         .map_err(|e| e.to_string())
    |                   ^  - type must be known at this point
    |
help: consider giving this closure parameter an explicit type
    |
545 |         .map_err(|e: /* Type */| e.to_string())
    |                    ++++++++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, LspClient>>` in the current scope
   --> src\lib.rs:550:43
    |
550 |     let mut lsp = state.lsp_client.lock().unwrap();
    |                                           ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, LspClient>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:572:40
    |
572 |     let mut eh = state.ext_host.lock().unwrap();
    |                                        ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>`

error[E0282]: type annotations needed
   --> src\lib.rs:573:35
    |
573 |     eh.scan_extensions().map_err(|e| e.to_string())?;
    |                                   ^  - type must be known at this point
    |
help: consider giving this closure parameter an explicit type
    |
573 |     eh.scan_extensions().map_err(|e: /* Type */| e.to_string())?;
    |                                    ++++++++++++

error[E0282]: type annotations needed
   --> src\lib.rs:574:28
    |
574 |     eh.start(app).map_err(|e| e.to_string())
    |                            ^  - type must be known at this point
    |
help: consider giving this closure parameter an explicit type
    |
574 |     eh.start(app).map_err(|e: /* Type */| e.to_string())
    |                             ++++++++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:579:40
    |
579 |     let mut eh = state.ext_host.lock().unwrap();
    |                                        ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>`

error[E0282]: type annotations needed
   --> src\lib.rs:580:35
    |
580 |     eh.send_message(msg).map_err(|e| e.to_string())
    |                                   ^  - type must be known at this point
    |
help: consider giving this closure parameter an explicit type
    |
580 |     eh.send_message(msg).map_err(|e: /* Type */| e.to_string())
    |                                    ++++++++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, KeybindingRegistry>>` in the current scope
   --> src\lib.rs:585:39
    |
585 |     let kb = state.keybindings.lock().unwrap();
    |                                       ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, KeybindingRegistry>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:722:36
    |
722 |     let eh = state.ext_host.lock().unwrap();
    |                                    ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:735:36
    |
735 |     let eh = state.ext_host.lock().unwrap();
    |                                    ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\lib.rs:906:50
    |
906 |     let mut current = state.current_model.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\lib.rs:913:50
    |
913 |     let mut current = state.advisor_model.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
913 |     let mut current = state.advisor_model.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\lib.rs:920:50
    |
920 |     let sdk_path = state.android_sdk_path.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
920 |     let sdk_path = state.android_sdk_path.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\lib.rs:958:49
    |
958 |     let mut active = state.active_device.lock().unwrap();
    |                                                 ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
958 |     let mut active = state.active_device.lock().await.unwrap();
    |                                                 ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\lib.rs:971:50
    |
971 |     let sdk_path = state.android_sdk_path.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
971 |     let sdk_path = state.android_sdk_path.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\lib.rs:988:49
    |
988 |     let mut sdk = state.android_sdk_path.lock().unwrap();
    |                                                 ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
988 |     let mut sdk = state.android_sdk_path.lock().await.unwrap();
    |                                                 ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
   --> src\lib.rs:995:50
    |
995 |     let sdk_path = state.android_sdk_path.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
    |
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
995 |     let sdk_path = state.android_sdk_path.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>` in the current scope
    --> src\lib.rs:1018:50
     |
1018 |     let sdk_path = state.android_sdk_path.lock().unwrap();
     |                                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<std::string::String>>>`
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1018 |     let sdk_path = state.android_sdk_path.lock().await.unwrap();
     |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1041:45
     |
1041 |     let mut root = state.active_root.lock().unwrap();
     |                                             ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1041 |     let mut root = state.active_root.lock().await.unwrap();
     |                                             ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1086:41
     |
1086 |     let root = state.active_root.lock().unwrap();
     |                                         ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1086 |     let root = state.active_root.lock().await.unwrap();
     |                                         ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1210:10
     |
1207 |       let root = state
     |  ________________-
1208 | |         .active_root
1209 | |         .lock()
1210 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     | |_________|
     |
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1210 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1303:10
     |
1300 |       let root = state
     |  ________________-
1301 | |         .active_root
1302 | |         .lock()
1303 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     | |_________|
     |
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1303 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1390:34
     |
1390 |         state.active_root.lock().unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
     |                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1390 |         state.active_root.lock().await.unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
     |                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1425:34
     |
1425 |         state.active_root.lock().unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
     |                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1425 |         state.active_root.lock().await.unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
     |                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1591:45
     |
1591 |         let root = state.active_root.lock().unwrap();
     |                                             ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<PathBuf>>>`
     |
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1591 |         let root = state.active_root.lock().await.unwrap();
     |                                             ++++++

error[E0282]: type annotations needed
    --> src\lib.rs:1593:35
     |
1593 |             let r_owned: String = r.display().to_string();
     |                                   ^ cannot infer type

error[E0308]: mismatched types
    --> src\lib.rs:1639:28
     |
1639 |                     if let Ok(mut buffers) = state.terminal_buffers.lock() {
     |                            ^^^^^^^^^^^^^^^   ----------------------------- this expression has type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Vec<std::string::String>>>>`
     |                            |
     |                            expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Vec<std::string::String>>>>`
                       found enum `Result<_, _>`

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn MasterPty + Send>>>>` in the current scope
    --> src\lib.rs:1677:10
     |
1674 | /     state
1675 | |         .terminal_masters
1676 | |         .lock()
1677 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn MasterPty + Send>>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-202085710481428740.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>` in the current scope
    --> src\lib.rs:1682:10
     |
1679 | /     state
1680 | |         .terminal_writers
1681 | |         .lock()
1682 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13950145716504459422.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>` in the current scope
    --> src\lib.rs:1684:37
     |
1684 |     state.terminal_processes.lock().unwrap().insert(id, child);
     |                                     ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-6264432265849339865.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>` in the current scope
    --> src\lib.rs:1691:35
     |
1691 |     state.terminal_writers.lock().unwrap().remove(&id);
     |                                   ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-11889351033094295990.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn MasterPty + Send>>>>` in the current scope
    --> src\lib.rs:1692:35
     |
1692 |     state.terminal_masters.lock().unwrap().remove(&id);
     |                                   ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn MasterPty + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-12642570125930937554.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>` in the current scope
    --> src\lib.rs:1693:62
     |
1693 |     if let Some(mut child) = state.terminal_processes.lock().unwrap().remove(&id) {
     |                                                              ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13208701274851531118.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:1694:17
     |
1694 |         let _ = child.kill();
     |                 ^^^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>` in the current scope
    --> src\lib.rs:1727:53
     |
1727 |     let mut writers = state.terminal_writers.lock().unwrap();
     |                                                     ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-11889351033094295990.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:1729:9
     |
1729 |         writer
     |         ^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:1731:23
     |
1731 |             .map_err(|e| e.to_string())?;
     |                       ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
1731 |             .map_err(|e: /* Type */| e.to_string())?;
     |                        ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:1732:33
     |
1732 |         writer.flush().map_err(|e| e.to_string())?;
     |                                 ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
1732 |         writer.flush().map_err(|e: /* Type */| e.to_string())?;
     |                                  ++++++++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn MasterPty + Send>>>>` in the current scope
    --> src\lib.rs:1746:49
     |
1746 |     let masters = state.terminal_masters.lock().unwrap();
     |                                                 ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn MasterPty + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-11364350947364344953.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:1748:9
     |
1748 |         master
     |         ^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:1755:23
     |
1755 |             .map_err(|e| e.to_string())?;
     |                       ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
1755 |             .map_err(|e: /* Type */| e.to_string())?;
     |                        ++++++++++++

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>` in the current scope
    --> src\lib.rs:2046:38
     |
2046 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                      ^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>`

error[E0282]: type annotations needed
    --> src\lib.rs:2046:47
     |
2046 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                               ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2046 |     let host = state.ext_host.lock().map_err(|e: /* Type */| e.to_string())?;
     |                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2051:47
     |
2051 |             if let Some(contributed_themes) = contributes.get("themes").and_then(|v| v.as_array()) {
     |                                               ^^^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2051:83
     |
2051 |             if let Some(contributed_themes) = contributes.get("themes").and_then(|v| v.as_array()) {
     |                                                                                   ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2051 |             if let Some(contributed_themes) = contributes.get("themes").and_then(|v: /* Type */| v.as_array()) {
     |                                                                                    ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2053:42
     |
2053 |                     if let Some(label) = theme.get("label").and_then(|v| v.as_str()) {
     |                                          ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2053:71
     |
2053 |                     if let Some(label) = theme.get("label").and_then(|v| v.as_str()) {
     |                                                                       ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2053 |                     if let Some(label) = theme.get("label").and_then(|v: /* Type */| v.as_str()) {
     |                                                                        ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2054:73
     |
2054 |                         if let Some(path) = theme.get("path").and_then(|v| v.as_str()) {
     |                                                                         ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2054 |                         if let Some(path) = theme.get("path").and_then(|v: /* Type */| v.as_str()) {
     |                                                                          ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2062:75
     |
2062 | ...                   "uiTheme": theme.get("uiTheme").and_then(|v| v.as_str()).unwrap_or("vs-dark"),
     |                                                                 ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2062 |                                 "uiTheme": theme.get("uiTheme").and_then(|v: /* Type */| v.as_str()).unwrap_or("vs-dark"),
     |                                                                            ++++++++++++

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>` in the current scope
    --> src\lib.rs:2198:38
     |
2198 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                      ^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>`

error[E0282]: type annotations needed
    --> src\lib.rs:2198:47
     |
2198 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                               ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2198 |     let host = state.ext_host.lock().map_err(|e: /* Type */| e.to_string())?;
     |                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2203:40
     |
2203 |             if let Some(icon_themes) = contributes.get("iconThemes").and_then(|v| v.as_array()) {
     |                                        ^^^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2203:80
     |
2203 |             if let Some(icon_themes) = contributes.get("iconThemes").and_then(|v| v.as_array()) {
     |                                                                                ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2203 |             if let Some(icon_themes) = contributes.get("iconThemes").and_then(|v: /* Type */| v.as_array()) {
     |                                                                                 ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2205:41
     |
2205 |                     if let Some(path) = theme.get("path").and_then(|v| v.as_str()) {
     |                                         ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2205:69
     |
2205 |                     if let Some(path) = theme.get("path").and_then(|v| v.as_str()) {
     |                                                                     ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2205 |                     if let Some(path) = theme.get("path").and_then(|v: /* Type */| v.as_str()) {
     |                                                                      ++++++++++++

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>` in the current scope
    --> src\lib.rs:2247:38
     |
2247 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                      ^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ExtensionHostManager>>`

error[E0282]: type annotations needed
    --> src\lib.rs:2247:47
     |
2247 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                               ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2247 |     let host = state.ext_host.lock().map_err(|e: /* Type */| e.to_string())?;
     |                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2260:41
     |
2260 |             if let Some(ext_snippets) = contributes.get("snippets").and_then(|v| v.as_array()) {
     |                                         ^^^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2260:79
     |
2260 |             if let Some(ext_snippets) = contributes.get("snippets").and_then(|v| v.as_array()) {
     |                                                                               ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2260 |             if let Some(ext_snippets) = contributes.get("snippets").and_then(|v: /* Type */| v.as_array()) {
     |                                                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2262:33
     |
2262 |                     let mut s = snippet.clone();
     |                                 ^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2263:66
     |
2263 |                     if let Some(spath) = s.get("path").and_then(|v| v.as_str()) {
     |                                                                  ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2263 |                     if let Some(spath) = s.get("path").and_then(|v: /* Type */| v.as_str()) {
     |                                                                   ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2264:66
     |
2264 |                         let full_spath = ext.extension_path.join(spath.replace("./", ""));
     |                                                                  ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2266:29
     |
2266 | ...                   obj.insert(
     |                       ^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2276:77
     |
2276 |             if let Some(ext_langs) = contributes.get("languages").and_then(|v| v.as_array()) {
     |                                                                             ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2276 |             if let Some(ext_langs) = contributes.get("languages").and_then(|v: /* Type */| v.as_array()) {
     |                                                                              ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2281:31
     |
2281 |                         .push(lang.clone());
     |                               ^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2285:79
     |
2285 |             if let Some(ext_grammars) = contributes.get("grammars").and_then(|v| v.as_array()) {
     |                                                                               ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2285 |             if let Some(ext_grammars) = contributes.get("grammars").and_then(|v: /* Type */| v.as_array()) {
     |                                                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2287:33
     |
2287 |                     let mut g = grammar.clone();
     |                                 ^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2288:66
     |
2288 |                     if let Some(gpath) = g.get("path").and_then(|v| v.as_str()) {
     |                                                                  ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2288 |                     if let Some(gpath) = g.get("path").and_then(|v: /* Type */| v.as_str()) {
     |                                                                   ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2289:66
     |
2289 |                         let full_gpath = ext.extension_path.join(gpath.replace("./", ""));
     |                                                                  ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2291:29
     |
2291 | ...                   obj.insert(
     |                       ^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2302:44
     |
2302 |                 if let Some(activitybar) = containers.get("activitybar").and_then(|v| v.as_array())
     |                                            ^^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2302:84
     |
2302 |                 if let Some(activitybar) = containers.get("activitybar").and_then(|v| v.as_array())
     |                                                                                    ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2302 |                 if let Some(activitybar) = containers.get("activitybar").and_then(|v: /* Type */| v.as_array())
     |                                                                                     ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2305:37
     |
2305 |                         let mut c = container.clone();
     |                                     ^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2307:29
     |
2307 | ...                   obj.insert(
     |                       ^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2314:79
     |
2314 | ...                   if let Some(icon_val) = obj.get("icon").and_then(|v| v.as_str()) {
     |                                                                         ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2314 |                             if let Some(icon_val) = obj.get("icon").and_then(|v: /* Type */| v.as_str()) {
     |                                                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2315:36
     |
2315 | ...                   if icon_val.starts_with("$(") && icon_val.ends_with(")") {
     |                          ^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2346:69
     |
2346 |             if let Some(views) = contributes.get("views").and_then(|v| v.as_object()) {
     |                                                                     ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2346 |             if let Some(views) = contributes.get("views").and_then(|v: /* Type */| v.as_object()) {
     |                                                                      ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2348:40
     |
2348 |                     if let Some(arr) = view_list.as_array() {
     |                                        ^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2352:36
     |
2352 | ...                   .entry(location.clone())
     |                              ^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2357:41
     |
2357 | ...                   let mut v = view.clone();
     |                                   ^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2359:33
     |
2359 | ...                   obj.insert(
     |                       ^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>` in the current scope
    --> src\lib.rs:2551:48
     |
2551 |     let mut debug = state.debug_manager.lock().unwrap();
     |                                                ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>` in the current scope
    --> src\lib.rs:2557:48
     |
2557 |     let mut debug = state.debug_manager.lock().unwrap();
     |                                                ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>` in the current scope
    --> src\lib.rs:2563:48
     |
2563 |     let mut debug = state.debug_manager.lock().unwrap();
     |                                                ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ActivationManager>>` in the current scope
    --> src\lib.rs:2569:50
     |
2569 |     let mut am = state.activation_manager.lock().unwrap();
     |                                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ActivationManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>` in the current scope
    --> src\lib.rs:2588:57
     |
2588 |     let mut processes = state.terminal_processes.lock().unwrap();
     |                                                         ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13404418817589738805.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:2590:17
     |
2590 |         let _ = child.kill();
     |                 ^^^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn MasterPty + Send>>>>` in the current scope
    --> src\lib.rs:2592:35
     |
2592 |     state.terminal_masters.lock().unwrap().remove(&id);
     |                                   ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn MasterPty + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-12642570125930937554.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>` in the current scope
    --> src\lib.rs:2593:35
     |
2593 |     state.terminal_writers.lock().unwrap().remove(&id);
     |                                   ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-11219515314523669978.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>` in the current scope
    --> src\lib.rs:2613:57
     |
2613 |     let mut processes = state.terminal_processes.lock().unwrap();
     |                                                         ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Child + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13404418817589738805.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:2615:15
     |
2615 |         match child.try_wait() {
     |               ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2617:68
     |
2617 |                 Ok(serde_json::json!({ "active": false, "success": status.success() }))
     |                                                                    ^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2620:27
     |
2620 |             Err(e) => Err(e.to_string()),
     |                           ^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>` in the current scope
    --> src\lib.rs:2999:69
     |
2999 |                     let mut writers = state.terminal_writers.lock().unwrap();
     |                                                                     ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, Box<dyn Write + Send>>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-3169830899442916022.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:3010:37
     |
3010 | ...                   let _ = writer.write_all(data.as_bytes());
     |                               ^^^^^^ cannot infer type

Some errors have detailed explanations: E0277, E0282, E0308, E0599.
For more information about an error, try `rustc --explain E0277`.
warning: `vscode-rust-app` (lib) generated 1 warning
error: could not compile `vscode-rust-app` (lib) due to 189 previous errors; 1 warning emitted


*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

*Edited relevant file*

*Grep searched codebase*

*Analyzed [kairos.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/kairos.rs) *

*Analyzed [context_indexer.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/context_indexer.rs) *

*Edited relevant file*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Edited relevant file*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Edited relevant file*

*Grep searched codebase*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Edited relevant file*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Analyzed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Edited relevant file*

*Edited relevant file*

*Edited relevant file*

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Grep searched codebase*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*User accepted the command `grep -n "fn terminal_create" src-tauri\src\ai_tools.rs`*

*Checked command status*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Grep searched codebase*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Grep searched codebase*

*User accepted the command `findstr /n "fn terminal_create" src-tauri\src\ai_tools.rs`*

*Checked command status*

*User accepted the command `findstr /n "fn terminal_send_data" src-tauri\src\ai_tools.rs`*

*Checked command status*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Grep searched codebase*

*Viewed [task.md](file:///C:/Users/HADES/.gemini/antigravity/brain/17dc6d5a-04e0-4580-8f68-2bc579d661d9/task.md) *

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Edited relevant file*

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Grep searched codebase*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Grep searched codebase*

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [task.md](file:///C:/Users/HADES/.gemini/antigravity/brain/17dc6d5a-04e0-4580-8f68-2bc579d661d9/task.md) *

*Edited relevant file*

*Edited relevant file*

*User accepted the command `cargo check`*

*Checked command status*

*Checked command status*

### User Input

C:\Users\HADES>cd C:\Users\HADES\Desktop\vscodium-rust\src-tauri

C:\Users\HADES\Desktop\vscodium-rust\src-tauri>cargo check
    Checking vscode-rust-app v0.1.0 (C:\Users\HADES\Desktop\vscodium-rust\src-tauri)
error: async commands that contain references as inputs must return a `Result`
   --> src\lib.rs:521:26
    |
521 | async fn update_settings(state: State<'_, EditorState>, settings: Settings) {
    |                          ^^^^^

error: async commands that contain references as inputs must return a `Result`
   --> src\lib.rs:549:19
    |
549 | async fn lsp_stop(state: State<'_, EditorState>) {
    |                   ^^^^^

error: async commands that contain references as inputs must return a `Result`
    --> src\lib.rs:1040:26
     |
1040 | async fn set_active_root(state: State<'_, EditorState>, path: Option<String>) {
     |                          ^^^^^

error: cannot find macro `__cmd__update_settings` in this scope
    --> src\lib.rs:3124:13
     |
 515 | #[tauri::command]
     | ----------------- similarly named macro `__cmd__get_settings` defined here
...
3124 |             update_settings,
     |             ^^^^^^^^^^^^^^^
     |
help: a macro with a similar name exists
     |
3124 -             update_settings,
3124 +             __cmd__get_settings,
     |

error: cannot find macro `__cmd__set_active_root` in this scope
    --> src\lib.rs:3128:13
     |
 956 | #[tauri::command]
     | ----------------- similarly named macro `__cmd__set_active_device` defined here
...
3128 |             set_active_root,
     |             ^^^^^^^^^^^^^^^
     |
help: a macro with a similar name exists
     |
3128 -             set_active_root,
3128 +             __cmd__set_active_device,
     |

error: cannot find macro `__cmd__lsp_stop` in this scope
    --> src\lib.rs:3138:13
     |
 526 | #[tauri::command]
     | ----------------- similarly named macro `__cmd__lsp_start` defined here
...
3138 |             lsp_stop,
     |             ^^^^^^^^
     |
help: a macro with a similar name exists
     |
3138 -             lsp_stop,
3138 +             __cmd__lsp_start,
     |

warning: unused import: `Path`
 --> src\lib.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `Mutex`
  --> src\lib.rs:11:22
   |
11 | use std::sync::{Arc, Mutex};
   |                      ^^^^^

warning: unused import: `Mutex`
 --> src\ai_tools.rs:8:22
  |
8 | use std::sync::{Arc, Mutex};
  |                      ^^^^^

warning: unused import: `Mutex`
 --> src\kairos.rs:1:22
  |
1 | use std::sync::{Arc, Mutex};
  |                      ^^^^^

error[E0728]: `await` is only allowed inside `async` functions and blocks
    --> src\ai_engine.rs:2251:54
     |
2251 |         if let Some(handle) = self.app_handle.lock().await.as_ref() {
     |                                                      ^^^^^ only allowed inside `async` functions and blocks

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\ai_engine.rs:457:44
    |
457 |             let u = self.ollama_url.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\ai_engine.rs:473:44
    |
473 |             let u = self.ollama_url.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0308]: `match` arms have incompatible types
    --> src\ai_tools.rs:1085:33
     |
1045 | /         match name {
1046 | |             // Filesystem Operations
1047 | |             "view_file"
1048 | |             | "write_to_file"
...    |
1060 | |             | "editor_get_active_file" => self.handle_fs_tool(name, arguments).await,
     | |                                           ------------------------------------------ this is found to be of type `Result<serde_json::Value, anyhow::Error>`
...    |
1070 | |             | "terminal_list" => self.handle_terminal_tool(name, arguments).await,
     | |                                  ------------------------------------------------ this is found to be of type `Result<serde_json::Value, anyhow::Error>`
...    |
1082 | |             | "browser_read_dom" => self.handle_browser_tool(name, arguments).await,
     | |                                     ----------------------------------------------- this is found to be of type `Result<serde_json::Value, anyhow::Error>`
...    |
1085 | |             "spawn_subagent" => self.spawn_subagent(arguments),
     | |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future
...    |
1132 | |             _ => Err(anyhow!("Unknown tool: {}", name)),
1133 | |         }
     | |_________- `match` arms have incompatible types
     |
note: calling an async function returns a future
    --> src\ai_tools.rs:1085:33
     |
1085 |             "spawn_subagent" => self.spawn_subagent(arguments),
     |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
     |
1085 |             "spawn_subagent" => self.spawn_subagent(arguments).await,
     |                                                               ++++++

error[E0277]: `Result<serde_json::Value, anyhow::Error>` is not a future
    --> src\ai_tools.rs:1103:64
     |
1103 |             "generate_image" => self.generate_image(arguments).await,
     |                                                                ^^^^^ `Result<serde_json::Value, anyhow::Error>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<serde_json::Value, anyhow::Error>`
     = note: Result<serde_json::Value, anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<serde_json::Value, anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
1103 -             "generate_image" => self.generate_image(arguments).await,
1103 +             "generate_image" => self.generate_image(arguments),
     |

error[E0277]: `Result<serde_json::Value, anyhow::Error>` is not a future
    --> src\ai_tools.rs:1104:62
     |
1104 |             "analyze_image" => self.analyze_image(arguments).await,
     |                                                              ^^^^^ `Result<serde_json::Value, anyhow::Error>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<serde_json::Value, anyhow::Error>`
     = note: Result<serde_json::Value, anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<serde_json::Value, anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
1104 -             "analyze_image" => self.analyze_image(arguments).await,
1104 +             "analyze_image" => self.analyze_image(arguments),
     |

error[E0308]: mismatched types
    --> src\ai_tools.rs:1159:28
     |
1159 |             "view_file" => self.read_file(arguments),
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future
     |
note: calling an async function returns a future
    --> src\ai_tools.rs:1159:28
     |
1159 |             "view_file" => self.read_file(arguments),
     |                            ^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
     |
1159 |             "view_file" => self.read_file(arguments).await,
     |                                                     ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:1200:30
     |
1200 |             "run_command" => self.run_command(arguments),
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future
     |
note: calling an async function returns a future
    --> src\ai_tools.rs:1200:30
     |
1200 |             "run_command" => self.run_command(arguments),
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
     |
1200 |             "run_command" => self.run_command(arguments).await,
     |                                                         ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:1214:32
     |
1214 |             "browser_close" => self.browser_close(arguments),
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future
     |
note: calling an async function returns a future
    --> src\ai_tools.rs:1214:32
     |
1214 |             "browser_close" => self.browser_close(arguments),
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
     |
1214 |             "browser_close" => self.browser_close(arguments).await,
     |                                                             ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:1230:29
     |
1230 |             "git_status" => self.git_status(arguments),
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future
     |
note: calling an async function returns a future
    --> src\ai_tools.rs:1230:29
     |
1230 |             "git_status" => self.git_status(arguments),
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
     |
1230 |             "git_status" => self.git_status(arguments).await,
     |                                                       ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:1241:34
     |
1241 |             "get_system_info" => self.get_system_info(arguments),
     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future
     |
note: calling an async function returns a future
    --> src\ai_tools.rs:1241:34
     |
1241 |             "get_system_info" => self.get_system_info(arguments),
     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
     |
1241 |             "get_system_info" => self.get_system_info(arguments).await,
     |                                                                 ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:1782:16
     |
1782 |         if let Ok(h_lock) = self.app_handle.lock() {
     |                ^^^^^^^^^^   ---------------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                |
     |                expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-248191250114457470.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:1784:25
     |
1784 |                 let _ = h.emit("ai-artifact", json!({
     |                         ^ cannot infer type

error[E0308]: mismatched types
  --> src\tool_invoker.rs:24:16
   |
24 |         if let Ok(result) = self.ai_tools.call_tool(name, arguments.clone()) {
   |                ^^^^^^^^^^   ------------------------------------------------ this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
   |                |
   |                expected future, found `Result<_, _>`
   |
   = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                     found enum `Result<_, _>`
help: consider `await`ing on the `Future`
   |
24 |         if let Ok(result) = self.ai_tools.call_tool(name, arguments.clone()).await {
   |                                                                             ++++++

error[E0609]: no field `browser_state` on type `&AiTools`
    --> src\ai_tools.rs:2512:37
     |
2512 |         let mut browser_lock = self.browser_state.browser.lock().await;
     |                                     ^^^^^^^^^^^^^ unknown field
     |
     = note: available fields are: `root_path`, `app_handle`, `git_manager`, `mcp_registry`, `memory_store` ... and 4 others

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2512:13
     |
2512 |         let mut browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^^^^^
2513 |         if browser_lock.is_some() {
     |            ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2512 |         let mut browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                             ++++++++++++

error[E0609]: no field `browser_state` on type `&AiTools`
    --> src\ai_tools.rs:2533:33
     |
2533 |         let browser_lock = self.browser_state.browser.lock().await;
     |                                 ^^^^^^^^^^^^^ unknown field
     |
     = note: available fields are: `root_path`, `app_handle`, `git_manager`, `mcp_registry`, `memory_store` ... and 4 others

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2533:13
     |
2533 |         let browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^
2534 |         let browser = browser_lock
     |                       ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2533 |         let browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                         ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2538:46
     |
2538 |         let tab = browser.new_tab().map_err(|e| anyhow!(e.to_string()))?;
     |                                              ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2538 |         let tab = browser.new_tab().map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                                               ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2539:39
     |
2539 |         tab.navigate_to(url).map_err(|e| anyhow!(e.to_string()))?;
     |                                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2539 |         tab.navigate_to(url).map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                                        ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2541:23
     |
2541 |             .map_err(|e| anyhow!(e.to_string()))?;
     |                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2541 |             .map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                        ++++++++++++

error[E0609]: no field `browser_state` on type `&AiTools`
    --> src\ai_tools.rs:2548:33
     |
2548 |         let browser_lock = self.browser_state.browser.lock().await;
     |                                 ^^^^^^^^^^^^^ unknown field
     |
     = note: available fields are: `root_path`, `app_handle`, `git_manager`, `mcp_registry`, `memory_store` ... and 4 others

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2548:13
     |
2548 |         let browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^
2549 |         let browser = browser_lock
     |                       ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2548 |         let browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                         ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2553:19
     |
2553 |           let tab = browser
     |  ___________________^
2554 | |             .get_tabs()
2555 | |             .lock()
2556 | |             .await
     | |__________________^ cannot infer type

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2567:23
     |
2567 |             .map_err(|e| anyhow!(e.to_string()))?;
     |                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2567 |             .map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                        ++++++++++++

error[E0609]: no field `browser_state` on type `&AiTools`
    --> src\ai_tools.rs:2579:33
     |
2579 |         let browser_lock = self.browser_state.browser.lock().await;
     |                                 ^^^^^^^^^^^^^ unknown field
     |
     = note: available fields are: `root_path`, `app_handle`, `git_manager`, `mcp_registry`, `memory_store` ... and 4 others

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2579:13
     |
2579 |         let browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^
2580 |         let browser = browser_lock
     |                       ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2579 |         let browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                         ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2584:19
     |
2584 |           let tab = browser
     |  ___________________^
2585 | |             .get_tabs()
2586 | |             .lock()
2587 | |             .await
     | |__________________^ cannot infer type

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2593:23
     |
2593 |             .map_err(|e| anyhow!(e.to_string()))?;
     |                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2593 |             .map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                        ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2594:34
     |
2594 |         element.click().map_err(|e| anyhow!(e.to_string()))?;
     |                                  ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2594 |         element.click().map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                                   ++++++++++++

error[E0609]: no field `browser_state` on type `&AiTools`
    --> src\ai_tools.rs:2608:33
     |
2608 |         let browser_lock = self.browser_state.browser.lock().await;
     |                                 ^^^^^^^^^^^^^ unknown field
     |
     = note: available fields are: `root_path`, `app_handle`, `git_manager`, `mcp_registry`, `memory_store` ... and 4 others

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2608:13
     |
2608 |         let browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^
2609 |         let browser = browser_lock
     |                       ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2608 |         let browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                         ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2613:19
     |
2613 |           let tab = browser
     |  ___________________^
2614 | |             .get_tabs()
2615 | |             .lock()
2616 | |             .await
     | |__________________^ cannot infer type

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2622:23
     |
2622 |             .map_err(|e| anyhow!(e.to_string()))?;
     |                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2622 |             .map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                        ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2625:23
     |
2625 |             .map_err(|e| anyhow!(e.to_string()))?;
     |                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2625 |             .map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                        ++++++++++++

error[E0609]: no field `browser_state` on type `&AiTools`
    --> src\ai_tools.rs:2631:33
     |
2631 |         let browser_lock = self.browser_state.browser.lock().await;
     |                                 ^^^^^^^^^^^^^ unknown field
     |
     = note: available fields are: `root_path`, `app_handle`, `git_manager`, `mcp_registry`, `memory_store` ... and 4 others

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2631:13
     |
2631 |         let browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^
2632 |         let browser = browser_lock
     |                       ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2631 |         let browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                         ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2636:19
     |
2636 |           let tab = browser
     |  ___________________^
2637 | |             .get_tabs()
2638 | |             .lock()
2639 | |             .await
     | |__________________^ cannot infer type

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2643:50
     |
2643 |         let content = tab.get_content().map_err(|e| anyhow!(e.to_string()))?;
     |                                                  ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2643 |         let content = tab.get_content().map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                                                   ++++++++++++

error[E0609]: no field `browser_state` on type `&AiTools`
    --> src\ai_tools.rs:2649:37
     |
2649 |         let mut browser_lock = self.browser_state.browser.lock().await;
     |                                     ^^^^^^^^^^^^^ unknown field
     |
     = note: available fields are: `root_path`, `app_handle`, `git_manager`, `mcp_registry`, `memory_store` ... and 4 others

error[E0277]: `Result<(), anyhow::Error>` is not a future
    --> src\ai_tools.rs:3125:19
     |
3125 |                 ).await;
     |                   ^^^^^ `Result<(), anyhow::Error>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<(), anyhow::Error>`
     = note: Result<(), anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<(), anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3125 -                 ).await;
3125 +                 );
     |

error[E0308]: mismatched types
  --> src\mcp_server.rs:46:21
   |
45 |                 match self.ai_tools.call_tool(name, args) {
   |                       ----------------------------------- this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
46 |                     Ok(res) => json!({ "result": res }),
   |                     ^^^^^^^ expected future, found `Result<_, _>`
   |
   = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                     found enum `Result<_, _>`
help: consider `await`ing on the `Future`
   |
45 |                 match self.ai_tools.call_tool(name, args).await {
   |                                                          ++++++

error[E0308]: mismatched types
  --> src\mcp_server.rs:47:21
   |
45 |                 match self.ai_tools.call_tool(name, args) {
   |                       ----------------------------------- this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
46 |                     Ok(res) => json!({ "result": res }),
47 |                     Err(e) => json!({ "error": e.to_string() }),
   |                     ^^^^^^ expected future, found `Result<_, _>`
   |
   = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                     found enum `Result<_, _>`
help: consider `await`ing on the `Future`
   |
45 |                 match self.ai_tools.call_tool(name, args).await {
   |                                                          ++++++

error[E0277]: `()` is not a future
   --> src\lib.rs:497:53
    |
497 |         state.ai_engine.set_root_path(path.clone()).await;
    |                                                     ^^^^^ `()` is not a future
    |
    = help: the trait `futures::Future` is not implemented for `()`
    = note: () must be a future or must implement `IntoFuture` to be awaited
    = note: required for `()` to implement `std::future::IntoFuture`
help: remove the `.await`
    |
497 -         state.ai_engine.set_root_path(path.clone()).await;
497 +         state.ai_engine.set_root_path(path.clone());
    |

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:606:40
    |
606 |         let eh = state.ext_host.lock().unwrap();
    |                                        ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-11400047194141011597.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:636:48
    |
636 |             let mut eh = state.ext_host.lock().unwrap();
    |                                                ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-7450070992799796568.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:672:40
    |
672 |     let mut eh = state.ext_host.lock().unwrap();
    |                                        ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-11400047194141011597.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>` in the current scope
    --> src\lib.rs:1768:10
     |
1764 | /     state
1765 | |         .ai_engine
1766 | |         .get_tools()
1767 | |         .get_system_health(json!({}))
1768 | |         .map_err(|e: anyhow::Error| e.to_string())
     | |         -^^^^^^^ method not found in `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
     | |_________|
     |
     |
     = help: items from traits can only be used if the trait is in scope
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1768 |         .await.map_err(|e: anyhow::Error| e.to_string())
     |          ++++++
help: trait `TryFutureExt` which provides `map_err` is implemented but not in scope; perhaps you want to import it
     |
   1 + use futures::TryFutureExt;
     |

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
    --> src\lib.rs:1861:44
     |
1861 |         let m = state.current_model.lock().unwrap();
     |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
    --> src\lib.rs:1885:41
     |
1885 |         let u = state.ollama_url.lock().unwrap();
     |                                         ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0277]: the `?` operator can only be applied to values that implement `Try`
    --> src\lib.rs:2180:5
     |
2180 |     is_path_valid(&state, &path_buf)?;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `impl futures::Future<Output = Result<(), std::string::String>>`
     |
     = help: the nightly-only, unstable trait `Try` is not implemented for `impl futures::Future<Output = Result<(), std::string::String>>`
help: consider `await`ing on the `Future`
     |
2180 |     is_path_valid(&state, &path_buf).await?;
     |                                     ++++++

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PatchEngine>>` in the current scope
    --> src\lib.rs:2182:48
     |
2182 |     let mut engine = state.patch_engine.lock().map_err(|e| e.to_string())?;
     |                                                ^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PatchEngine>>`

error[E0282]: type annotations needed
    --> src\lib.rs:2182:57
     |
2182 |     let mut engine = state.patch_engine.lock().map_err(|e| e.to_string())?;
     |                                                         ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2182 |     let mut engine = state.patch_engine.lock().map_err(|e: /* Type */| e.to_string())?;
     |                                                          ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2183:46
     |
2183 |     engine.commit_shadow(&path_buf).map_err(|e| e.to_string())
     |                                              ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2183 |     engine.commit_shadow(&path_buf).map_err(|e: /* Type */| e.to_string())
     |                                               ++++++++++++

error[E0277]: the `?` operator can only be applied to values that implement `Try`
    --> src\lib.rs:2189:5
     |
2189 |     is_path_valid(&state, &path_buf)?;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `impl futures::Future<Output = Result<(), std::string::String>>`
     |
     = help: the nightly-only, unstable trait `Try` is not implemented for `impl futures::Future<Output = Result<(), std::string::String>>`
help: consider `await`ing on the `Future`
     |
2189 |     is_path_valid(&state, &path_buf).await?;
     |                                     ++++++

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PatchEngine>>` in the current scope
    --> src\lib.rs:2191:48
     |
2191 |     let mut engine = state.patch_engine.lock().map_err(|e| e.to_string())?;
     |                                                ^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PatchEngine>>`

error[E0282]: type annotations needed
    --> src\lib.rs:2191:57
     |
2191 |     let mut engine = state.patch_engine.lock().map_err(|e| e.to_string())?;
     |                                                         ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2191 |     let mut engine = state.patch_engine.lock().map_err(|e: /* Type */| e.to_string())?;
     |                                                          ++++++++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
    --> src\lib.rs:2691:51
     |
2691 |         let mut current = state.ollama_url.lock().unwrap();
     |                                                   ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2831:10
     |
2828 |       let root = state
     |  ________________-
2829 | |         .active_root
2830 | |         .lock()
2831 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13722326455606168564.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2831 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2842:10
     |
2839 |       let root = state
     |  ________________-
2840 | |         .active_root
2841 | |         .lock()
2842 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13722326455606168564.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2842 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2853:10
     |
2850 |       let root = state
     |  ________________-
2851 | |         .active_root
2852 | |         .lock()
2853 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13722326455606168564.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2853 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2864:10
     |
2861 |       let root = state
     |  ________________-
2862 | |         .active_root
2863 | |         .lock()
2864 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13722326455606168564.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2864 |         .await.unwrap()
     |          ++++++

error[E0308]: mismatched types
   --> src\ai_engine.rs:203:13
    |
196 |         let ai_tools = Arc::new(AiTools::new(
    |                                 ------------ arguments to this function are incorrect
...
203 |             patch_engine.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<PatchEngine>`, found `std::sync::Mutex<PatchEngine>`
    |
    = note: `std::sync::Mutex<PatchEngine>` and `tauri::async_runtime::Mutex<PatchEngine>` have similar names, but are actually distinct types
note: `std::sync::Mutex<PatchEngine>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<PatchEngine>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: associated function defined here
   --> src\ai_tools.rs:33:12
    |
 33 |     pub fn new(
    |            ^^^
...
 40 |         patch_engine: Arc<tokio::sync::Mutex<crate::patch_engine::PatchEngine>>,
    |         -----------------------------------------------------------------------

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\ai_engine.rs:256:44
    |
256 |         let mut u = self.ollama_url.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\ai_engine.rs:261:47
    |
261 |         let mut m = self.advisor_model.lock().unwrap();
    |                                               ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-858563648896843062.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
261 |         let mut m = self.advisor_model.lock().await.unwrap();
    |                                               ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>` in the current scope
   --> src\ai_engine.rs:270:44
    |
270 |         let mut h = self.app_handle.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-8552119640505717818.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
270 |         let mut h = self.app_handle.lock().await.unwrap();
    |                                            ++++++

error[E0282]: type annotations needed
    --> src\ai_engine.rs:2252:21
     |
2252 |             let _ = handle.emit(event, payload);
     |                     ^^^^^^ cannot infer type

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:2708:14
     |
2705 |           let root = self
     |  ____________________-
2706 | |             .root_path
2707 | |             .lock()
2708 | |             .map_err(|e| anyhow!("Lock error: {}", e))?;
     | |             -^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`
     | |_____________|
     |

error[E0609]: no field `browser_state` on type `&AiTools`
    --> src\ai_tools.rs:2977:67
     |
2977 |             crate::browser::capture_vision_context_internal(&self.browser_state)
     |                                                                   ^^^^^^^^^^^^^ unknown field
     |
     = note: available fields are: `root_path`, `app_handle`, `git_manager`, `mcp_registry`, `memory_store` ... and 4 others

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:3158:14
     |
3155 |           let root = self
     |  ____________________-
3156 | |             .root_path
3157 | |             .lock()
3158 | |             .map_err(|_| anyhow!("Failed to lock root_path"))?;
     | |             -^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`
     | |_____________|
     |

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:3193:14
     |
3190 |           let root = self
     |  ____________________-
3191 | |             .root_path
3192 | |             .lock()
3193 | |             .map_err(|_| anyhow!("Failed to lock root_path"))?;
     | |             -^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`
     | |_____________|
     |

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:3230:14
     |
3227 |           let root = self
     |  ____________________-
3228 | |             .root_path
3229 | |             .lock()
3230 | |             .map_err(|_| anyhow!("Failed to lock root_path"))?;
     | |             -^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`
     | |_____________|
     |

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:3266:14
     |
3263 |           let root = self
     |  ____________________-
3264 | |             .root_path
3265 | |             .lock()
3266 | |             .map_err(|e| anyhow!("Lock error: {}", e))?;
     | |             -^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`
     | |_____________|
     |

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:3367:42
     |
3367 |         let root = self.root_path.lock().map_err(|_| anyhow!("Lock error"))?;
     |                                          ^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`

error[E0308]: mismatched types
    --> src\ai_tools.rs:3423:16
     |
3423 |         if let Ok(h_lock) = app_handle.lock() {
     |                ^^^^^^^^^^   ----------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                |
     |                expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-14103550994970572321.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3425:25
     |
3425 |                 let _ = h.emit(
     |                         ^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3451:24
     |
3451 |                 if let Ok(h_lock) = h_loop.lock() {
     |                        ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                        |
     |                        expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-4827434933185009573.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3453:33
     |
3453 |                         let _ = h_val.emit(
     |                                 ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3467:20
     |
3467 |             if let Err(e) = sub_tools.browser_open(json!({})) {
     |                    ^^^^^^   --------------------------------- this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
     |                    |
     |                    expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3467 |             if let Err(e) = sub_tools.browser_open(json!({})).await {
     |                                                              ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3469:28
     |
3469 |                     if let Ok(h_lock) = h_loop.lock() {
     |                            ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                            |
     |                            expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-9170996804894148312.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3471:37
     |
3471 | ...                   let _ = h_val.emit(
     |                               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3487:24
     |
3487 |                 if let Ok(h_lock) = h_loop.lock() {
     |                        ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                        |
     |                        expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-8709021802698544946.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3489:33
     |
3489 |                         let _ = h_val.emit(
     |                                 ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3504:17
     |
3503 |             match sub_tools.browser_search(json!({ "query": t_loop })) {
     |                   ---------------------------------------------------- this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
3504 |                 Ok(_) => {
     |                 ^^^^^ expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3503 |             match sub_tools.browser_search(json!({ "query": t_loop })).await {
     |                                                                       ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3520:17
     |
3503 |             match sub_tools.browser_search(json!({ "query": t_loop })) {
     |                   ---------------------------------------------------- this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
...
3520 |                 Err(e) => {
     |                 ^^^^^^ expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3503 |             match sub_tools.browser_search(json!({ "query": t_loop })).await {
     |                                                                       ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3505:28
     |
3505 |                     if let Ok(h_lock) = h_loop.lock() {
     |                            ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                            |
     |                            expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-9519746072787246002.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3507:37
     |
3507 | ...                   let _ = h_val.emit(
     |                               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3522:32
     |
3522 |                         if let Ok(h_lock) = h_loop.lock() {
     |                                ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                                |
     |                                expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-4680314390408527865.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3524:41
     |
3524 | ...                   let _ = h_val.emit(
     |                               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3541:24
     |
3541 |                 if let Ok(h_lock) = h_loop.lock() {
     |                        ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                        |
     |                        expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2587812069418213099.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3543:33
     |
3543 |                         let _ = h_val.emit(
     |                                 ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3558:17
     |
3557 |             let summary = match sub_tools.browser_get_content_summary(json!({})) {
     |                                 ------------------------------------------------ this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
3558 |                 Ok(s) => s,
     |                 ^^^^^ expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3557 |             let summary = match sub_tools.browser_get_content_summary(json!({})).await {
     |                                                                                 ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3559:17
     |
3557 |             let summary = match sub_tools.browser_get_content_summary(json!({})) {
     |                                 ------------------------------------------------ this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
3558 |                 Ok(s) => s,
3559 |                 Err(e) => {
     |                 ^^^^^^ expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3557 |             let summary = match sub_tools.browser_get_content_summary(json!({})).await {
     |                                                                                 ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3561:32
     |
3561 |                         if let Ok(h_lock) = h_loop.lock() {
     |                                ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                                |
     |                                expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-11087954034738499077.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3563:41
     |
3563 | ...                   let _ = h_val.emit(
     |                               ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3581:38
     |
3581 |                 if let Some(first) = links.first() {
     |                                      ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3584:36
     |
3584 | ...                   if let Ok(h_lock) = h_loop.lock() {
     |                              ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                              |
     |                              expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-14282471841869206307.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3586:45
     |
3586 | ...   let _ = h_val.emit("subagent-progress", json!({ "id": tid_loop, "title": format!("Web Research: {}", t_...
     |               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3593:36
     |
3593 | ...                   if let Ok(h_lock) = h_loop.lock() {
     |                              ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                              |
     |                              expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-9062579882650796463.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3595:45
     |
3595 | ...   let _ = h_val.emit("subagent-progress", json!({ "id": tid_loop, "title": format!("Web Research: {}", t_...
     |               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3599:32
     |
3599 |                         if let Ok(detail_summary) = sub_tools.browser_get_content_summary(json!({}))
     |                                ^^^^^^^^^^^^^^^^^^   ------------------------------------------------ this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
     |                                |
     |                                expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3599 |                         if let Ok(detail_summary) = sub_tools.browser_get_content_summary(json!({})).await
     |                                                                                                     ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3614:24
     |
3614 |                 if let Ok(h_lock) = h_loop.lock() {
     |                        ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                        |
     |                        expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-5970074603278053953.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3616:33
     |
3616 | ...   let _ = h_val.emit("subagent-progress", json!({ "id": tid_loop, "title": format!("Web Research: {}", t_...
     |               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3630:24
     |
3630 |                 if let Ok(h_lock) = h_loop.lock() {
     |                        ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                        |
     |                        expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2172568282242636007.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3632:33
     |
3632 |                         let _ = h_val.emit(
     |                                 ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3981:9
     |
3979 |     pub fn run_command_safe(&self, args: Value) -> Result<Value> {
     |                                                    ------------- expected `Result<serde_json::Value, anyhow::Error>` because of return type
3980 |         let command = args["command"].as_str().ok_or_else(|| anyhow!("Missing command"))?;
3981 |         self.run_command(json!({ "command": command }))
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future

error[E0308]: mismatched types
    --> src\ai_tools.rs:3986:9
     |
3984 |     pub fn verify_implementation(&self, args: Value) -> Result<Value> {
     |                                                         ------------- expected `Result<serde_json::Value, anyhow::Error>` because of return type
3985 |         let command = args["command"].as_str().unwrap_or("cargo check");
3986 |         self.run_command(json!({ "command": command, "shell_hint": "powershell" }))
     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future

error[E0308]: mismatched types
    --> src\ai_tools.rs:3992:16
     |
3992 |         if let Ok(h_lock) = self.app_handle.lock() {
     |                ^^^^^^^^^^   ---------------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                |
     |                expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-12200164493273104965.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3994:25
     |
3994 |                 let _ = h.emit("agent-mission-plan", json!({ "plan": plan }));
     |                         ^ cannot infer type

error[E0308]: mismatched types
  --> src\kairos.rs:39:16
   |
39 |         if let Ok(mut last) = self.last_activity.lock() {
   |                ^^^^^^^^^^^^   ------------------------- this expression has type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::time::Instant>>`
   |                |
   |                expected future, found `Result<_, _>`
   |
   = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::time::Instant>>`
                     found enum `Result<_, _>`

error[E0277]: `Result<MutexGuard<'_, PatchEngine>, PoisonError<...>>` is not a future
   --> src\lib.rs:243:23
    |
243 |             pe.lock().await.set_app_handle(app_clone);
    |                       ^^^^^ `Result<MutexGuard<'_, PatchEngine>, PoisonError<...>>` is not a future
    |
    = help: the trait `futures::Future` is not implemented for `Result<MutexGuard<'_, PatchEngine>, PoisonError<...>>`
    = note: Result<MutexGuard<'_, PatchEngine>, PoisonError<...>> must be a future or must implement `IntoFuture` to be awaited
    = note: required for `Result<MutexGuard<'_, PatchEngine>, PoisonError<...>>` to implement `std::future::IntoFuture`
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2737898902523629281.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
    |
243 -             pe.lock().await.set_app_handle(app_clone);
243 +             pe.lock().set_app_handle(app_clone);
    |

error[E0282]: type annotations needed
   --> src\lib.rs:243:13
    |
243 |             pe.lock().await.set_app_handle(app_clone);
    |             ^^^^^^^^^^^^^^^ cannot infer type

error[E0308]: mismatched types
   --> src\lib.rs:276:22
    |
276 |             Arc::new(std::sync::Mutex::new(Some(root.clone()))),
    |             -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Mutex<Option<PathBuf>>`, found a different `Mutex<Option<PathBuf>>`
    |             |
    |             arguments to this function are incorrect
    |
    = note: `Mutex<Option<PathBuf>>` and `Mutex<Option<PathBuf>>` have similar names, but are actually distinct types
note: `Mutex<Option<PathBuf>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<Option<PathBuf>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: associated function defined here
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\alloc\src\sync.rs:419:11

error[E0308]: mismatched types
   --> src\lib.rs:326:13
    |
326 |             patch_engine,
    |             ^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<PatchEngine>`, found `std::sync::Mutex<PatchEngine>`
    |
    = note: `std::sync::Mutex<PatchEngine>` and `tauri::async_runtime::Mutex<PatchEngine>` have similar names, but are actually distinct types
note: `std::sync::Mutex<PatchEngine>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<PatchEngine>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0277]: async commands that contain references as inputs must return a `Result`
   --> src\lib.rs:516:57
    |
516 | async fn get_settings(state: State<'_, EditorState>) -> Settings {
    |                                                         ^^^^^^^^ unsatisfied trait bound
    |
help: the trait `_::AsyncCommandMustReturnResult` is not implemented for `Settings`
   --> src\lib.rs:103:1
    |
103 | struct Settings {
    | ^^^^^^^^^^^^^^^
help: the trait `_::AsyncCommandMustReturnResult` is implemented for `Result<A, B>`
   --> src\lib.rs:516:57
    |
516 | async fn get_settings(state: State<'_, EditorState>) -> Settings {
    |                                                         ^^^^^^^^
    = note: required for the cast from `&Settings` to `&dyn _::AsyncCommandMustReturnResult`

error[E0277]: async commands that contain references as inputs must return a `Result`
   --> src\lib.rs:584:76
    |
584 | async fn resolve_keybinding(state: State<'_, EditorState>, key: String) -> Option<String> {
    |                                                                            ^^^^^^ the trait `_::AsyncCommandMustReturnResult` is not implemented for `std::option::Option<std::string::String>`
    |
help: the trait `_::AsyncCommandMustReturnResult` is implemented for `Result<A, B>`
   --> src\lib.rs:584:76
    |
584 | async fn resolve_keybinding(state: State<'_, EditorState>, key: String) -> Option<String> {
    |                                                                            ^^^^^^
    = note: required for the cast from `&std::option::Option<std::string::String>` to `&dyn _::AsyncCommandMustReturnResult`

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:722:36
    |
722 |     let eh = state.ext_host.lock().unwrap();
    |                                    ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-1274290757008072307.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:735:36
    |
735 |     let eh = state.ext_host.lock().unwrap();
    |                                    ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-1274290757008072307.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\lib.rs:906:50
    |
906 |     let mut current = state.current_model.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:913:50
    |
913 |     let mut current = state.advisor_model.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-3088504107074835308.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
913 |     let mut current = state.advisor_model.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:920:50
    |
920 |     let sdk_path = state.android_sdk_path.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-3088504107074835308.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
920 |     let sdk_path = state.android_sdk_path.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:958:49
    |
958 |     let mut active = state.active_device.lock().unwrap();
    |                                                 ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-3088504107074835308.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
958 |     let mut active = state.active_device.lock().await.unwrap();
    |                                                 ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:971:50
    |
971 |     let sdk_path = state.android_sdk_path.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-3088504107074835308.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
971 |     let sdk_path = state.android_sdk_path.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:988:49
    |
988 |     let mut sdk = state.android_sdk_path.lock().unwrap();
    |                                                 ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-3088504107074835308.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
988 |     let mut sdk = state.android_sdk_path.lock().await.unwrap();
    |                                                 ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:995:50
    |
995 |     let sdk_path = state.android_sdk_path.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-3088504107074835308.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
995 |     let sdk_path = state.android_sdk_path.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1210:10
     |
1207 |       let root = state
     |  ________________-
1208 | |         .active_root
1209 | |         .lock()
1210 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-5704303777381790984.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1210 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1303:10
     |
1300 |       let root = state
     |  ________________-
1301 | |         .active_root
1302 | |         .lock()
1303 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-5704303777381790984.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1303 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1390:34
     |
1390 |         state.active_root.lock().unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
     |                                  ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-5704303777381790984.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1390 |         state.active_root.lock().await.unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
     |                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1425:34
     |
1425 |         state.active_root.lock().unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
     |                                  ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-5704303777381790984.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1425 |         state.active_root.lock().await.unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
     |                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:1591:45
     |
1591 |         let root = state.active_root.lock().unwrap();
     |                                             ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-15228758765362619664.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1591 |         let root = state.active_root.lock().await.unwrap();
     |                                             ++++++

error[E0282]: type annotations needed
    --> src\lib.rs:1593:35
     |
1593 |             let r_owned: String = r.display().to_string();
     |                                   ^ cannot infer type

error[E0308]: mismatched types
    --> src\lib.rs:1639:28
     |
1639 |                     if let Ok(mut buffers) = state.terminal_buffers.lock() {
     |                            ^^^^^^^^^^^^^^^   ----------------------------- this expression has type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |                            |
     |                            expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Vec<std::string::String>>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-6191199128610625577.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:1677:10
     |
1674 | /     state
1675 | |         .terminal_masters
1676 | |         .lock()
1677 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-3104457465388715430.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:1682:10
     |
1679 | /     state
1680 | |         .terminal_writers
1681 | |         .lock()
1682 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-9189334405916215541.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:1684:37
     |
1684 |     state.terminal_processes.lock().unwrap().insert(id, child);
     |                                     ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-11916798154681625955.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:1691:35
     |
1691 |     state.terminal_writers.lock().unwrap().remove(&id);
     |                                   ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-5632932024041894338.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:1692:35
     |
1692 |     state.terminal_masters.lock().unwrap().remove(&id);
     |                                   ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-9139594934373215249.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:1693:62
     |
1693 |     if let Some(mut child) = state.terminal_processes.lock().unwrap().remove(&id) {
     |                                                              ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2879747016394571833.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:1694:17
     |
1694 |         let _ = child.kill();
     |                 ^^^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:1727:53
     |
1727 |     let mut writers = state.terminal_writers.lock().unwrap();
     |                                                     ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-5632932024041894338.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:1729:9
     |
1729 |         writer
     |         ^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:1731:23
     |
1731 |             .map_err(|e| e.to_string())?;
     |                       ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
1731 |             .map_err(|e: /* Type */| e.to_string())?;
     |                        ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:1732:33
     |
1732 |         writer.flush().map_err(|e| e.to_string())?;
     |                                 ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
1732 |         writer.flush().map_err(|e: /* Type */| e.to_string())?;
     |                                  ++++++++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:1746:49
     |
1746 |     let masters = state.terminal_masters.lock().unwrap();
     |                                                 ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-18163055192123012754.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:1748:9
     |
1748 |         master
     |         ^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:1755:23
     |
1755 |             .map_err(|e| e.to_string())?;
     |                       ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
1755 |             .map_err(|e: /* Type */| e.to_string())?;
     |                        ++++++++++++

error[E0599]: no method named `map_err` found for opaque type `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>` in the current scope
    --> src\lib.rs:2046:38
     |
2046 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                      ^^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-1274290757008072307.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:2046:47
     |
2046 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                               ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2046 |     let host = state.ext_host.lock().map_err(|e: /* Type */| e.to_string())?;
     |                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2051:47
     |
2051 |             if let Some(contributed_themes) = contributes.get("themes").and_then(|v| v.as_array()) {
     |                                               ^^^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2051:83
     |
2051 |             if let Some(contributed_themes) = contributes.get("themes").and_then(|v| v.as_array()) {
     |                                                                                   ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2051 |             if let Some(contributed_themes) = contributes.get("themes").and_then(|v: /* Type */| v.as_array()) {
     |                                                                                    ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2053:42
     |
2053 |                     if let Some(label) = theme.get("label").and_then(|v| v.as_str()) {
     |                                          ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2053:71
     |
2053 |                     if let Some(label) = theme.get("label").and_then(|v| v.as_str()) {
     |                                                                       ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2053 |                     if let Some(label) = theme.get("label").and_then(|v: /* Type */| v.as_str()) {
     |                                                                        ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2054:73
     |
2054 |                         if let Some(path) = theme.get("path").and_then(|v| v.as_str()) {
     |                                                                         ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2054 |                         if let Some(path) = theme.get("path").and_then(|v: /* Type */| v.as_str()) {
     |                                                                          ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2062:75
     |
2062 | ...                   "uiTheme": theme.get("uiTheme").and_then(|v| v.as_str()).unwrap_or("vs-dark"),
     |                                                                 ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2062 |                                 "uiTheme": theme.get("uiTheme").and_then(|v: /* Type */| v.as_str()).unwrap_or("vs-dark"),
     |                                                                            ++++++++++++

error[E0277]: the `?` operator can only be applied to values that implement `Try`
    --> src\lib.rs:2147:5
     |
2147 |     is_path_valid(&state, &path_buf)?;
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the `?` operator cannot be applied to type `impl futures::Future<Output = Result<(), std::string::String>>`
     |
     = help: the nightly-only, unstable trait `Try` is not implemented for `impl futures::Future<Output = Result<(), std::string::String>>`
note: this implements `Future` and its output type supports `?`, but the future cannot be awaited in a synchronous function
    --> src\lib.rs:2147:37
     |
2140 | / fn propose_file_change(
2141 | |     state: tauri::State<'_, EditorState>,
2142 | |     path: String,
2143 | |     content: String,
2144 | |     description: String,
2145 | | ) -> Result<serde_json::Value, String> {
     | |______________________________________- this is not `async`
2146 |       let path_buf = PathBuf::from(&path);
2147 |       is_path_valid(&state, &path_buf)?;
     |                                       ^

error[E0599]: no method named `map_err` found for opaque type `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>` in the current scope
    --> src\lib.rs:2198:38
     |
2198 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                      ^^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-1274290757008072307.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:2198:47
     |
2198 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                               ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2198 |     let host = state.ext_host.lock().map_err(|e: /* Type */| e.to_string())?;
     |                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2203:40
     |
2203 |             if let Some(icon_themes) = contributes.get("iconThemes").and_then(|v| v.as_array()) {
     |                                        ^^^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2203:80
     |
2203 |             if let Some(icon_themes) = contributes.get("iconThemes").and_then(|v| v.as_array()) {
     |                                                                                ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2203 |             if let Some(icon_themes) = contributes.get("iconThemes").and_then(|v: /* Type */| v.as_array()) {
     |                                                                                 ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2205:41
     |
2205 |                     if let Some(path) = theme.get("path").and_then(|v| v.as_str()) {
     |                                         ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2205:69
     |
2205 |                     if let Some(path) = theme.get("path").and_then(|v| v.as_str()) {
     |                                                                     ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2205 |                     if let Some(path) = theme.get("path").and_then(|v: /* Type */| v.as_str()) {
     |                                                                      ++++++++++++

error[E0599]: no method named `map_err` found for opaque type `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>` in the current scope
    --> src\lib.rs:2247:38
     |
2247 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                      ^^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-1274290757008072307.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:2247:47
     |
2247 |     let host = state.ext_host.lock().map_err(|e| e.to_string())?;
     |                                               ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2247 |     let host = state.ext_host.lock().map_err(|e: /* Type */| e.to_string())?;
     |                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2260:41
     |
2260 |             if let Some(ext_snippets) = contributes.get("snippets").and_then(|v| v.as_array()) {
     |                                         ^^^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2260:79
     |
2260 |             if let Some(ext_snippets) = contributes.get("snippets").and_then(|v| v.as_array()) {
     |                                                                               ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2260 |             if let Some(ext_snippets) = contributes.get("snippets").and_then(|v: /* Type */| v.as_array()) {
     |                                                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2262:33
     |
2262 |                     let mut s = snippet.clone();
     |                                 ^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2263:66
     |
2263 |                     if let Some(spath) = s.get("path").and_then(|v| v.as_str()) {
     |                                                                  ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2263 |                     if let Some(spath) = s.get("path").and_then(|v: /* Type */| v.as_str()) {
     |                                                                   ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2264:66
     |
2264 |                         let full_spath = ext.extension_path.join(spath.replace("./", ""));
     |                                                                  ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2266:29
     |
2266 | ...                   obj.insert(
     |                       ^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2276:77
     |
2276 |             if let Some(ext_langs) = contributes.get("languages").and_then(|v| v.as_array()) {
     |                                                                             ^  - type must be known at this point     |
help: consider giving this closure parameter an explicit type
     |
2276 |             if let Some(ext_langs) = contributes.get("languages").and_then(|v: /* Type */| v.as_array()) {
     |                                                                              ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2281:31
     |
2281 |                         .push(lang.clone());
     |                               ^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2285:79
     |
2285 |             if let Some(ext_grammars) = contributes.get("grammars").and_then(|v| v.as_array()) {
     |                                                                               ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2285 |             if let Some(ext_grammars) = contributes.get("grammars").and_then(|v: /* Type */| v.as_array()) {
     |                                                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2287:33
     |
2287 |                     let mut g = grammar.clone();
     |                                 ^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2288:66
     |
2288 |                     if let Some(gpath) = g.get("path").and_then(|v| v.as_str()) {
     |                                                                  ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2288 |                     if let Some(gpath) = g.get("path").and_then(|v: /* Type */| v.as_str()) {
     |                                                                   ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2289:66
     |
2289 |                         let full_gpath = ext.extension_path.join(gpath.replace("./", ""));
     |                                                                  ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2291:29
     |
2291 | ...                   obj.insert(
     |                       ^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2302:44
     |
2302 |                 if let Some(activitybar) = containers.get("activitybar").and_then(|v| v.as_array())
     |                                            ^^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2302:84
     |
2302 |                 if let Some(activitybar) = containers.get("activitybar").and_then(|v| v.as_array())
     |                                                                                    ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2302 |                 if let Some(activitybar) = containers.get("activitybar").and_then(|v: /* Type */| v.as_array())
     |                                                                                     ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2305:37
     |
2305 |                         let mut c = container.clone();
     |                                     ^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2307:29
     |
2307 | ...                   obj.insert(
     |                       ^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2314:79
     |
2314 | ...                   if let Some(icon_val) = obj.get("icon").and_then(|v| v.as_str()) {
     |                                                                         ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2314 |                             if let Some(icon_val) = obj.get("icon").and_then(|v: /* Type */| v.as_str()) {
     |                                                                                ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2315:36
     |
2315 | ...                   if icon_val.starts_with("$(") && icon_val.ends_with(")") {
     |                          ^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2346:69
     |
2346 |             if let Some(views) = contributes.get("views").and_then(|v| v.as_object()) {
     |                                                                     ^  - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2346 |             if let Some(views) = contributes.get("views").and_then(|v: /* Type */| v.as_object()) {
     |                                                                      ++++++++++++

error[E0282]: type annotations needed
    --> src\lib.rs:2348:40
     |
2348 |                     if let Some(arr) = view_list.as_array() {
     |                                        ^^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2352:36
     |
2352 | ...                   .entry(location.clone())
     |                              ^^^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2357:41
     |
2357 | ...                   let mut v = view.clone();
     |                                   ^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2359:33
     |
2359 | ...                   obj.insert(
     |                       ^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>` in the current scope
    --> src\lib.rs:2551:48
     |
2551 |     let mut debug = state.debug_manager.lock().unwrap();
     |                                                ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>` in the current scope
    --> src\lib.rs:2557:48
     |
2557 |     let mut debug = state.debug_manager.lock().unwrap();
     |                                                ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>` in the current scope
    --> src\lib.rs:2563:48
     |
2563 |     let mut debug = state.debug_manager.lock().unwrap();
     |                                                ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ActivationManager>>` in the current scope
    --> src\lib.rs:2569:50
     |
2569 |     let mut am = state.activation_manager.lock().unwrap();
     |                                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ActivationManager>>`

error[E0308]: mismatched types
    --> src\lib.rs:2576:5
     |
2575 | fn terminal_read_output(state: State<'_, EditorState>, id: String) -> Result<String, String> {
     |                                                                       ---------------------- expected `Result<std::string::String, std::string::String>` because of return type
2576 |     state.terminal_read_output(id)
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<String, String>`, found future

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:2588:57
     |
2588 |     let mut processes = state.terminal_processes.lock().unwrap();
     |                                                         ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-17620088172427558434.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:2590:17
     |
2590 |         let _ = child.kill();
     |                 ^^^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:2592:35
     |
2592 |     state.terminal_masters.lock().unwrap().remove(&id);
     |                                   ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-9139594934373215249.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:2593:35
     |
2593 |     state.terminal_writers.lock().unwrap().remove(&id);
     |                                   ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-4408499721280646092.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>` in the current scope
    --> src\lib.rs:2605:10
     |
2603 | /     tools
2604 | |         .editor_get_active_file(serde_json::json!({}))
2605 | |         .map_err(|e: anyhow::Error| e.to_string())
     | |         -^^^^^^^ method not found in `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
     | |_________|
     |
     |
     = help: items from traits can only be used if the trait is in scope
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2605 |         .await.map_err(|e: anyhow::Error| e.to_string())
     |          ++++++
help: trait `TryFutureExt` which provides `map_err` is implemented but not in scope; perhaps you want to import it
     |
   1 + use futures::TryFutureExt;
     |

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:2613:57
     |
2613 |     let mut processes = state.terminal_processes.lock().unwrap();
     |                                                         ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-17620088172427558434.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:2615:15
     |
2615 |         match child.try_wait() {
     |               ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2617:68
     |
2617 |                 Ok(serde_json::json!({ "active": false, "success": status.success() }))
     |                                                                    ^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2620:27
     |
2620 |             Err(e) => Err(e.to_string()),
     |                           ^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:2999:69
     |
2999 |                     let mut writers = state.terminal_writers.lock().unwrap();
     |                                                                     ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-6707466884342220440.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:3010:37
     |
3010 | ...                   let _ = writer.write_all(data.as_bytes());
     |                               ^^^^^^ cannot infer type

error: future cannot be sent between threads safely
    --> src\ai_engine.rs:1095:25
     |
1095 | /                         tauri::async_runtime::spawn(async move {
1096 | |                             let _ = h_arc.execute_agent_step("Final Mission Review").await;
1097 | |                         });
     | |__________________________^ future created by async block is not `Send`
     |
     = note: cannot satisfy `impl Future<Output = Result<String, Error>>: Send`
note: future is not `Send` as it awaits another future which is not `Send`
    --> src\ai_engine.rs:402:22
     |
 402 |         let result = self.autonomous_loop(req, on_chunk).await?;
     |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here on type `impl futures::Future<Output = Result<std::string::String, anyhow::Error>>`, which is not `Send`
note: required by a bound in `tauri::async_runtime::spawn`
    --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tauri-2.10.3\src\async_runtime.rs:270:15
     |
 268 | pub fn spawn<F>(task: F) -> JoinHandle<F::Output>
     |        ----- required by a bound in this function
 269 | where
 270 |   F: Future + Send + 'static,
     |               ^^^^ required by this bound in `spawn`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-4696054822559594837.txt'
     = note: consider using `--verbose` to print the full type name to the console

warning: unused variable: `browser_state`
  --> src\ai_tools.rs:35:9
   |
35 |         browser_state: Arc<crate::browser::BrowserState>,
   |         ^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_browser_state`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

Some errors have detailed explanations: E0277, E0282, E0308, E0599, E0609, E0728.
For more information about an error, try `rustc --explain E0277`.
warning: `vscode-rust-app` (lib) generated 5 warnings
error: could not compile `vscode-rust-app` (lib) due to 210 previous errors; 5 warnings emitted

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [implementation_plan.md](file:///C:/Users/HADES/.gemini/antigravity/brain/17dc6d5a-04e0-4580-8f68-2bc579d661d9/implementation_plan.md) *

*Edited relevant file*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Edited relevant file*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Grep searched codebase*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [task.md](file:///C:/Users/HADES/.gemini/antigravity/brain/17dc6d5a-04e0-4580-8f68-2bc579d661d9/task.md) *

*Edited relevant file*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Grep searched codebase*

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Grep searched codebase*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Edited relevant file*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Grep searched codebase*

*Grep searched codebase*

*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

*Grep searched codebase*

*Edited relevant file*

*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Grep searched codebase*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Grep searched codebase*

### User Input

continue

*Grep searched codebase*

*Analyzed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Grep searched codebase*

*Grep searched codebase*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Grep searched codebase*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Viewed [ai_tools.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs) *

*Edited relevant file*

*Viewed [mcp_server.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/mcp_server.rs) *

*Edited relevant file*

*Viewed [kairos.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/kairos.rs) *

*Grep searched codebase*

*Viewed [lib.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/lib.rs) *

*Grep searched codebase*

*Edited relevant file*

*Viewed [ai_auth.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_auth.rs) *

*Edited relevant file*

*Edited relevant file*

*Viewed [mcp_registry.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/mcp_registry.rs) *

*User accepted the command `cargo check`*

*Checked command status*

*Edited relevant file*

*Checked command status*

### User Input

C:\Users\HADES\Desktop\vscodium-rust\src-tauri>cargo check
    Checking vscode-rust-app v0.1.0 (C:\Users\HADES\Desktop\vscodium-rust\src-tauri)
warning: unused import: `Path`
 --> src\lib.rs:9:17
  |
9 | use std::path::{Path, PathBuf};
  |                 ^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `Mutex`
  --> src\lib.rs:11:22
   |
11 | use std::sync::{Arc, Mutex};
   |                      ^^^^^

warning: unused import: `std::sync::Mutex`
 --> src\ai_auth.rs:3:5
  |
3 | use std::sync::Mutex;
  |     ^^^^^^^^^^^^^^^^

warning: unused import: `Mutex`
 --> src\ai_tools.rs:8:22
  |
8 | use std::sync::{Arc, Mutex};
  |                      ^^^^^

warning: unused import: `Mutex`
 --> src\kairos.rs:1:22
  |
1 | use std::sync::{Arc, Mutex};
  |                      ^^^^^

error[E0728]: `await` is only allowed inside `async` functions and blocks
    --> src\ai_engine.rs:2254:54
     |
2254 |         if let Some(handle) = self.app_handle.lock().await.as_ref() {
     |                                                      ^^^^^ only allowed inside `async` functions and blocks

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\ai_engine.rs:476:44
    |
476 |             let u = self.ollama_url.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0308]: mismatched types
    --> src\ai_engine.rs:1147:24
     |
1147 |                 if let Some(session) = crate::ai_auth::get_session(&self.auth_state, &provider_name)
     |                        ^^^^^^^^^^^^^   ------------------------------------------------------------- this expression has type `impl futures::Future<Output = std::option::Option<AiSession>>`
     |                        |
     |                        expected future, found `Option<_>`
     |
     = note: expected opaque type `impl futures::Future<Output = std::option::Option<AiSession>>`
                       found enum `std::option::Option<_>`
help: consider `await`ing on the `Future`
     |
1147 |                 if let Some(session) = crate::ai_auth::get_session(&self.auth_state, &provider_name).await
     |                                                                                                     ++++++

error[E0277]: `Result<serde_json::Value, anyhow::Error>` is not a future
    --> src\ai_tools.rs:1088:96
     |
1088 |             "browser_subagent" => AiTools::browser_subagent(Arc::new(self.clone()), arguments).await,
     |                                   ------------------------------------------------------------ ^^^^^ `Result<serde_json::Value, anyhow::Error>` is not a future
     |                                   |
     |                                   this call returns `Result<serde_json::Value, anyhow::Error>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<serde_json::Value, anyhow::Error>`
     = note: Result<serde_json::Value, anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<serde_json::Value, anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
1088 -             "browser_subagent" => AiTools::browser_subagent(Arc::new(self.clone()), arguments).await,
1088 +             "browser_subagent" => AiTools::browser_subagent(Arc::new(self.clone()), arguments),
     |

error[E0277]: `Result<serde_json::Value, anyhow::Error>` is not a future
    --> src\ai_tools.rs:1089:94
     |
1089 |             "perplexity_ask" => AiTools::perplexity_proxy(Arc::new(self.clone()), arguments).await,
     |                                 ------------------------------------------------------------ ^^^^^ `Result<serde_json::Value, anyhow::Error>` is not a future
     |                                 |
     |                                 this call returns `Result<serde_json::Value, anyhow::Error>`
     |
     = help: the trait `futures::Future` is not implemented for `Result<serde_json::Value, anyhow::Error>`
     = note: Result<serde_json::Value, anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<serde_json::Value, anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
1089 -             "perplexity_ask" => AiTools::perplexity_proxy(Arc::new(self.clone()), arguments).await,
1089 +             "perplexity_ask" => AiTools::perplexity_proxy(Arc::new(self.clone()), arguments),
     |

error[E0277]: `Result<serde_json::Value, anyhow::Error>` is not a future
    --> src\ai_tools.rs:1105:64
     |
1105 |             "generate_image" => self.generate_image(arguments).await,
     |                                                                ^^^^^ `Result<serde_json::Value, anyhow::Error>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<serde_json::Value, anyhow::Error>`
     = note: Result<serde_json::Value, anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<serde_json::Value, anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
1105 -             "generate_image" => self.generate_image(arguments).await,
1105 +             "generate_image" => self.generate_image(arguments),
     |

error[E0277]: `Result<serde_json::Value, anyhow::Error>` is not a future
    --> src\ai_tools.rs:1106:62
     |
1106 |             "analyze_image" => self.analyze_image(arguments).await,
     |                                                              ^^^^^ `Result<serde_json::Value, anyhow::Error>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<serde_json::Value, anyhow::Error>`
     = note: Result<serde_json::Value, anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<serde_json::Value, anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
1106 -             "analyze_image" => self.analyze_image(arguments).await,
1106 +             "analyze_image" => self.analyze_image(arguments),
     |

error[E0277]: `Result<serde_json::Value, anyhow::Error>` is not a future
    --> src\ai_tools.rs:1168:88
     |
1168 |             "multi_replace_file_content" => self.multi_replace_file_content(arguments).await,
     |                                                                                        ^^^^^ `Result<serde_json::Value, anyhow::Error>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<serde_json::Value, anyhow::Error>`
     = note: Result<serde_json::Value, anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<serde_json::Value, anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
1168 -             "multi_replace_file_content" => self.multi_replace_file_content(arguments).await,
1168 +             "multi_replace_file_content" => self.multi_replace_file_content(arguments),
     |

error[E0277]: `Result<serde_json::Value, anyhow::Error>` is not a future
    --> src\ai_tools.rs:1169:60
     |
1169 |             "find_by_name" => self.find_by_name(arguments).await,
     |                                                            ^^^^^ `Result<serde_json::Value, anyhow::Error>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<serde_json::Value, anyhow::Error>`
     = note: Result<serde_json::Value, anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<serde_json::Value, anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
1169 -             "find_by_name" => self.find_by_name(arguments).await,
1169 +             "find_by_name" => self.find_by_name(arguments),
     |

error[E0277]: `Result<serde_json::Value, anyhow::Error>` is not a future
    --> src\ai_tools.rs:1170:82
     |
1170 |             "get_directory_structure" => self.get_directory_structure(arguments).await,
     |                                                                                  ^^^^^ `Result<serde_json::Value, anyhow::Error>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<serde_json::Value, anyhow::Error>`
     = note: Result<serde_json::Value, anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<serde_json::Value, anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
1170 -             "get_directory_structure" => self.get_directory_structure(arguments).await,
1170 +             "get_directory_structure" => self.get_directory_structure(arguments),
     |

error[E0277]: `Result<serde_json::Value, anyhow::Error>` is not a future
    --> src\ai_tools.rs:1187:76
     |
1187 |             "network_port_scanner" => self.network_port_scanner(arguments).await,
     |                                                                            ^^^^^ `Result<serde_json::Value, anyhow::Error>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<serde_json::Value, anyhow::Error>`
     = note: Result<serde_json::Value, anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<serde_json::Value, anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
1187 -             "network_port_scanner" => self.network_port_scanner(arguments).await,
1187 +             "network_port_scanner" => self.network_port_scanner(arguments),
     |

error[E0308]: mismatched types
    --> src\ai_tools.rs:1202:30
     |
1202 |             "run_command" => self.run_command(arguments),
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future
     |
note: calling an async function returns a future
    --> src\ai_tools.rs:1202:30
     |
1202 |             "run_command" => self.run_command(arguments),
     |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
     |
1202 |             "run_command" => self.run_command(arguments).await,
     |                                                         ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:1216:32
     |
1216 |             "browser_close" => self.browser_close(arguments),
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future
     |
note: calling an async function returns a future
    --> src\ai_tools.rs:1216:32
     |
1216 |             "browser_close" => self.browser_close(arguments),
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
     |
1216 |             "browser_close" => self.browser_close(arguments).await,
     |                                                             ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:1232:29
     |
1232 |             "git_status" => self.git_status(arguments),
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future
     |
note: calling an async function returns a future
    --> src\ai_tools.rs:1232:29
     |
1232 |             "git_status" => self.git_status(arguments),
     |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
     |
1232 |             "git_status" => self.git_status(arguments).await,
     |                                                       ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:1243:34
     |
1243 |             "get_system_info" => self.get_system_info(arguments),
     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<Value, Error>`, found future
     |
note: calling an async function returns a future
    --> src\ai_tools.rs:1243:34
     |
1243 |             "get_system_info" => self.get_system_info(arguments),
     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider `await`ing on the `Future`
     |
1243 |             "get_system_info" => self.get_system_info(arguments).await,
     |                                                                 ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:1784:16
     |
1784 |         if let Ok(h_lock) = self.app_handle.lock() {
     |                ^^^^^^^^^^   ---------------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                |
     |                expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-17173498386581617989.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:1786:25
     |
1786 |                 let _ = h.emit("ai-artifact", json!({
     |                         ^ cannot infer type

error[E0277]: `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
    --> src\ai_tools.rs:2514:66
     |
2514 |         let mut browser_lock = self.browser_state.browser.lock().await;
     |                                                                  ^^^^^ `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>`
     = note: Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-14092764065622039932.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
2514 -         let mut browser_lock = self.browser_state.browser.lock().await;
2514 +         let mut browser_lock = self.browser_state.browser.lock();
     |

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2514:13
     |
2514 |         let mut browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^^^^^
2515 |         if browser_lock.is_some() {
     |            ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2514 |         let mut browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                             ++++++++++++

error[E0277]: `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
    --> src\ai_tools.rs:2535:62
     |
2535 |         let browser_lock = self.browser_state.browser.lock().await;
     |                                                              ^^^^^ `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>`
     = note: Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13561074160363250495.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
2535 -         let browser_lock = self.browser_state.browser.lock().await;
2535 +         let browser_lock = self.browser_state.browser.lock();
     |

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2535:13
     |
2535 |         let browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^
2536 |         let browser = browser_lock
     |                       ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2535 |         let browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                         ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2540:46
     |
2540 |         let tab = browser.new_tab().map_err(|e| anyhow!(e.to_string()))?;
     |                                              ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2540 |         let tab = browser.new_tab().map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                                               ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2541:39
     |
2541 |         tab.navigate_to(url).map_err(|e| anyhow!(e.to_string()))?;
     |                                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2541 |         tab.navigate_to(url).map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                                        ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2543:23
     |
2543 |             .map_err(|e| anyhow!(e.to_string()))?;
     |                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2543 |             .map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                        ++++++++++++

error[E0277]: `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
    --> src\ai_tools.rs:2550:62
     |
2550 |         let browser_lock = self.browser_state.browser.lock().await;
     |                                                              ^^^^^ `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>`
     = note: Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-14092764065622039932.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
2550 -         let browser_lock = self.browser_state.browser.lock().await;
2550 +         let browser_lock = self.browser_state.browser.lock();
     |

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2550:13
     |
2550 |         let browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^
2551 |         let browser = browser_lock
     |                       ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2550 |         let browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                         ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2555:19
     |
2555 |           let tab = browser
     |  ___________________^
2556 | |             .get_tabs()
2557 | |             .lock()
2558 | |             .await
     | |__________________^ cannot infer type

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2569:23
     |
2569 |             .map_err(|e| anyhow!(e.to_string()))?;
     |                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2569 |             .map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                        ++++++++++++

error[E0277]: `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
    --> src\ai_tools.rs:2581:62
     |
2581 |         let browser_lock = self.browser_state.browser.lock().await;
     |                                                              ^^^^^ `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>`
     = note: Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13561074160363250495.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
2581 -         let browser_lock = self.browser_state.browser.lock().await;
2581 +         let browser_lock = self.browser_state.browser.lock();
     |

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2581:13
     |
2581 |         let browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^
2582 |         let browser = browser_lock
     |                       ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2581 |         let browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                         ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2586:19
     |
2586 |           let tab = browser
     |  ___________________^
2587 | |             .get_tabs()
2588 | |             .lock()
2589 | |             .await
     | |__________________^ cannot infer type

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2595:23
     |
2595 |             .map_err(|e| anyhow!(e.to_string()))?;
     |                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2595 |             .map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                        ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2596:34
     |
2596 |         element.click().map_err(|e| anyhow!(e.to_string()))?;
     |                                  ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2596 |         element.click().map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                                   ++++++++++++

error[E0277]: `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
    --> src\ai_tools.rs:2610:62
     |
2610 |         let browser_lock = self.browser_state.browser.lock().await;
     |                                                              ^^^^^ `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>`
     = note: Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-4130338727270629381.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
2610 -         let browser_lock = self.browser_state.browser.lock().await;
2610 +         let browser_lock = self.browser_state.browser.lock();
     |

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2610:13
     |
2610 |         let browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^
2611 |         let browser = browser_lock
     |                       ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2610 |         let browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                         ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2615:19
     |
2615 |           let tab = browser
     |  ___________________^
2616 | |             .get_tabs()
2617 | |             .lock()
2618 | |             .await
     | |__________________^ cannot infer type

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2624:23
     |
2624 |             .map_err(|e| anyhow!(e.to_string()))?;
     |                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2624 |             .map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                        ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2627:23
     |
2627 |             .map_err(|e| anyhow!(e.to_string()))?;
     |                       ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2627 |             .map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                        ++++++++++++

error[E0277]: `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
    --> src\ai_tools.rs:2633:62
     |
2633 |         let browser_lock = self.browser_state.browser.lock().await;
     |                                                              ^^^^^ `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>`
     = note: Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-14092764065622039932.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
2633 -         let browser_lock = self.browser_state.browser.lock().await;
2633 +         let browser_lock = self.browser_state.browser.lock();
     |

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2633:13
     |
2633 |         let browser_lock = self.browser_state.browser.lock().await;
     |             ^^^^^^^^^^^^
2634 |         let browser = browser_lock
     |                       ------------ type must be known at this point
     |
help: consider giving `browser_lock` an explicit type
     |
2633 |         let browser_lock: /* Type */ = self.browser_state.browser.lock().await;
     |                         ++++++++++++

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2638:19
     |
2638 |           let tab = browser
     |  ___________________^
2639 | |             .get_tabs()
2640 | |             .lock()
2641 | |             .await
     | |__________________^ cannot infer type

error[E0282]: type annotations needed
    --> src\ai_tools.rs:2645:50
     |
2645 |         let content = tab.get_content().map_err(|e| anyhow!(e.to_string()))?;
     |                                                  ^          - type must be known at this point
     |
help: consider giving this closure parameter an explicit type
     |
2645 |         let content = tab.get_content().map_err(|e: /* Type */| anyhow!(e.to_string()))?;
     |                                                   ++++++++++++

error[E0277]: `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
    --> src\ai_tools.rs:2651:66
     |
2651 |         let mut browser_lock = self.browser_state.browser.lock().await;
     |                                                                  ^^^^^ `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>`
     = note: Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<MutexGuard<'_, Option<Browser>>, PoisonError<...>>` to implement `std::future::IntoFuture`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-14092764065622039932.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
     |
2651 -         let mut browser_lock = self.browser_state.browser.lock().await;
2651 +         let mut browser_lock = self.browser_state.browser.lock();
     |

error[E0277]: `Result<(), anyhow::Error>` is not a future
    --> src\ai_tools.rs:3127:19
     |
3127 |                 ).await;
     |                   ^^^^^ `Result<(), anyhow::Error>` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `Result<(), anyhow::Error>`
     = note: Result<(), anyhow::Error> must be a future or must implement `IntoFuture` to be awaited
     = note: required for `Result<(), anyhow::Error>` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
3127 -                 ).await;
3127 +                 );
     |

error[E0308]: mismatched types
  --> src\tool_invoker.rs:24:16
   |
24 |         if let Ok(result) = self.ai_tools.call_tool(name, arguments.clone()) {
   |                ^^^^^^^^^^   ------------------------------------------------ this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
   |                |
   |                expected future, found `Result<_, _>`
   |
   = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                     found enum `Result<_, _>`
help: consider `await`ing on the `Future`
   |
24 |         if let Ok(result) = self.ai_tools.call_tool(name, arguments.clone()).await {
   |                                                                             ++++++

error[E0277]: `()` is not a future
   --> src\lib.rs:497:53
    |
497 |         state.ai_engine.set_root_path(path.clone()).await;
    |                                                     ^^^^^ `()` is not a future
    |
    = help: the trait `futures::Future` is not implemented for `()`
    = note: () must be a future or must implement `IntoFuture` to be awaited
    = note: required for `()` to implement `std::future::IntoFuture`
help: remove the `.await`
    |
497 -         state.ai_engine.set_root_path(path.clone()).await;
497 +         state.ai_engine.set_root_path(path.clone());
    |

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:638:48
    |
638 |             let mut eh = state.ext_host.lock().unwrap();
    |                                                ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-9193726330556630504.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:674:40
    |
674 |     let mut eh = state.ext_host.lock().unwrap();
    |                                        ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-13309627982533627944.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0277]: `()` is not a future
    --> src\lib.rs:1047:49
     |
1047 |         state.ai_engine.set_root_path(path_buf).await;
     |                                                 ^^^^^ `()` is not a future
     |
     = help: the trait `futures::Future` is not implemented for `()`
     = note: () must be a future or must implement `IntoFuture` to be awaited
     = note: required for `()` to implement `std::future::IntoFuture`
help: remove the `.await`
     |
1047 -         state.ai_engine.set_root_path(path_buf).await;
1047 +         state.ai_engine.set_root_path(path_buf);
     |

error[E0308]: mismatched types
    --> src\lib.rs:1642:28
     |
1642 |                     if let Ok(mut buffers) = state.terminal_buffers.lock() {
     |                            ^^^^^^^^^^^^^^^   ----------------------------- this expression has type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |                            |
     |                            expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, HashMap<std::string::String, Vec<std::string::String>>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2844086213789737516.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>` in the current scope
    --> src\lib.rs:1771:10
     |
1767 | /     state
1768 | |         .ai_engine
1769 | |         .get_tools()
1770 | |         .get_system_health(json!({}))
1771 | |         .map_err(|e: anyhow::Error| e.to_string())
     | |         -^^^^^^^ method not found in `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
     | |_________|
     |
     |
     = help: items from traits can only be used if the trait is in scope
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
1771 |         .await.map_err(|e: anyhow::Error| e.to_string())
     |          ++++++
help: trait `TryFutureExt` which provides `map_err` is implemented but not in scope; perhaps you want to import it
     |
   1 + use futures::TryFutureExt;
     |

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
    --> src\lib.rs:2694:51
     |
2694 |         let mut current = state.ollama_url.lock().unwrap();
     |                                                   ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2834:10
     |
2831 |       let root = state
     |  ________________-
2832 | |         .active_root
2833 | |         .lock()
2834 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2733061978787153627.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2834 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2845:10
     |
2842 |       let root = state
     |  ________________-
2843 | |         .active_root
2844 | |         .lock()
2845 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2733061978787153627.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2845 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2856:10
     |
2853 |       let root = state
     |  ________________-
2854 | |         .active_root
2855 | |         .lock()
2856 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2733061978787153627.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2856 |         .await.unwrap()
     |          ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>` in the current scope
    --> src\lib.rs:2867:10
     |
2864 |       let root = state
     |  ________________-
2865 | |         .active_root
2866 | |         .lock()
2867 | |         .unwrap()
     | |         -^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<PathBuf>>>`
     | |_________|
     |
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2733061978787153627.txt'
     = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2867 |         .await.unwrap()
     |          ++++++

error[E0308]: mismatched types
   --> src\ai_engine.rs:203:13
    |
196 |         let ai_tools = Arc::new(AiTools::new(
    |                                 ------------ arguments to this function are incorrect
...
203 |             patch_engine.clone(),
    |             ^^^^^^^^^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<PatchEngine>`, found `std::sync::Mutex<PatchEngine>`
    |
    = note: `std::sync::Mutex<PatchEngine>` and `tauri::async_runtime::Mutex<PatchEngine>` have similar names, but are actually distinct types
note: `std::sync::Mutex<PatchEngine>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<PatchEngine>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: associated function defined here
   --> src\ai_tools.rs:34:12
    |
 34 |     pub fn new(
    |            ^^^
...
 41 |         patch_engine: Arc<tokio::sync::Mutex<crate::patch_engine::PatchEngine>>,
    |         -----------------------------------------------------------------------

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\ai_engine.rs:256:44
    |
256 |         let mut u = self.ollama_url.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\ai_engine.rs:261:47
    |
261 |         let mut m = self.advisor_model.lock().unwrap();
    |                                               ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-7355542753844816498.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
261 |         let mut m = self.advisor_model.lock().await.unwrap();
    |                                               ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>` in the current scope
   --> src\ai_engine.rs:270:44
    |
270 |         let mut h = self.app_handle.lock().unwrap();
    |                                            ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-6781658907941320465.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
270 |         let mut h = self.app_handle.lock().await.unwrap();
    |                                            ++++++

error[E0282]: type annotations needed
    --> src\ai_engine.rs:2255:21
     |
2255 |             let _ = handle.emit(event, payload);
     |                     ^^^^^^ cannot infer type

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:2710:14
     |
2707 |           let root = self
     |  ____________________-
2708 | |             .root_path
2709 | |             .lock()
2710 | |             .map_err(|e| anyhow!("Lock error: {}", e))?;
     | |             -^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`
     | |_____________|
     |

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:3160:14
     |
3157 |           let root = self
     |  ____________________-
3158 | |             .root_path
3159 | |             .lock()
3160 | |             .map_err(|_| anyhow!("Failed to lock root_path"))?;
     | |             -^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`
     | |_____________|
     |

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:3195:14
     |
3192 |           let root = self
     |  ____________________-
3193 | |             .root_path
3194 | |             .lock()
3195 | |             .map_err(|_| anyhow!("Failed to lock root_path"))?;
     | |             -^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`
     | |_____________|
     |

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:3232:14
     |
3229 |           let root = self
     |  ____________________-
3230 | |             .root_path
3231 | |             .lock()
3232 | |             .map_err(|_| anyhow!("Failed to lock root_path"))?;
     | |             -^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`
     | |_____________|
     |

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:3268:14
     |
3265 |           let root = self
     |  ____________________-
3266 | |             .root_path
3267 | |             .lock()
3268 | |             .map_err(|e| anyhow!("Lock error: {}", e))?;
     | |             -^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`
     | |_____________|
     |

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>` in the current scope
    --> src\ai_tools.rs:3369:42
     |
3369 |         let root = self.root_path.lock().map_err(|_| anyhow!("Lock error"))?;
     |                                          ^^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, PathBuf>>`

error[E0308]: mismatched types
    --> src\ai_tools.rs:3425:16
     |
3425 |         if let Ok(h_lock) = app_handle.lock() {
     |                ^^^^^^^^^^   ----------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                |
     |                expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-11795740903691493915.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3427:25
     |
3427 |                 let _ = h.emit(
     |                         ^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3453:24
     |
3453 |                 if let Ok(h_lock) = h_loop.lock() {
     |                        ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                        |
     |                        expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-6095550772450595186.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3455:33
     |
3455 |                         let _ = h_val.emit(
     |                                 ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3469:20
     |
3469 |             if let Err(e) = sub_tools.browser_open(json!({})) {
     |                    ^^^^^^   --------------------------------- this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
     |                    |
     |                    expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3469 |             if let Err(e) = sub_tools.browser_open(json!({})).await {
     |                                                              ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3471:28
     |
3471 |                     if let Ok(h_lock) = h_loop.lock() {
     |                            ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                            |
     |                            expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-1012055281857126590.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3473:37
     |
3473 | ...                   let _ = h_val.emit(
     |                               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3489:24
     |
3489 |                 if let Ok(h_lock) = h_loop.lock() {
     |                        ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                        |
     |                        expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-16446511774730108051.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3491:33
     |
3491 |                         let _ = h_val.emit(
     |                                 ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3506:17
     |
3505 |             match sub_tools.browser_search(json!({ "query": t_loop })) {
     |                   ---------------------------------------------------- this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
3506 |                 Ok(_) => {
     |                 ^^^^^ expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3505 |             match sub_tools.browser_search(json!({ "query": t_loop })).await {
     |                                                                       ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3522:17
     |
3505 |             match sub_tools.browser_search(json!({ "query": t_loop })) {
     |                   ---------------------------------------------------- this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
...
3522 |                 Err(e) => {
     |                 ^^^^^^ expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3505 |             match sub_tools.browser_search(json!({ "query": t_loop })).await {
     |                                                                       ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3507:28
     |
3507 |                     if let Ok(h_lock) = h_loop.lock() {
     |                            ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                            |
     |                            expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-15892577564253491935.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3509:37
     |
3509 | ...                   let _ = h_val.emit(
     |                               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3524:32
     |
3524 |                         if let Ok(h_lock) = h_loop.lock() {
     |                                ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                                |
     |                                expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-14787929888369333169.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3526:41
     |
3526 | ...                   let _ = h_val.emit(
     |                               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3543:24
     |
3543 |                 if let Ok(h_lock) = h_loop.lock() {
     |                        ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                        |
     |                        expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-14194673885238425699.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3545:33
     |
3545 |                         let _ = h_val.emit(
     |                                 ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3560:17
     |
3559 |             let summary = match sub_tools.browser_get_content_summary(json!({})) {
     |                                 ------------------------------------------------ this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
3560 |                 Ok(s) => s,
     |                 ^^^^^ expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3559 |             let summary = match sub_tools.browser_get_content_summary(json!({})).await {
     |                                                                                 ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3561:17
     |
3559 |             let summary = match sub_tools.browser_get_content_summary(json!({})) {
     |                                 ------------------------------------------------ this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
3560 |                 Ok(s) => s,
3561 |                 Err(e) => {
     |                 ^^^^^^ expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3559 |             let summary = match sub_tools.browser_get_content_summary(json!({})).await {
     |                                                                                 ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3563:32
     |
3563 |                         if let Ok(h_lock) = h_loop.lock() {
     |                                ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                                |
     |                                expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-4671356486214455413.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3565:41
     |
3565 | ...                   let _ = h_val.emit(
     |                               ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3583:38
     |
3583 |                 if let Some(first) = links.first() {
     |                                      ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3586:36
     |
3586 | ...                   if let Ok(h_lock) = h_loop.lock() {
     |                              ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                              |
     |                              expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-334393291239053002.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3588:45
     |
3588 | ...   let _ = h_val.emit("subagent-progress", json!({ "id": tid_loop, "title": format!("Web Research: {}", t_...
     |               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3595:36
     |
3595 | ...                   if let Ok(h_lock) = h_loop.lock() {
     |                              ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                              |
     |                              expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-5262038441638015675.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3597:45
     |
3597 | ...   let _ = h_val.emit("subagent-progress", json!({ "id": tid_loop, "title": format!("Web Research: {}", t_...
     |               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3601:32
     |
3601 |                         if let Ok(detail_summary) = sub_tools.browser_get_content_summary(json!({}))
     |                                ^^^^^^^^^^^^^^^^^^   ------------------------------------------------ this expression has type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
     |                                |
     |                                expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
                       found enum `Result<_, _>`
help: consider `await`ing on the `Future`
     |
3601 |                         if let Ok(detail_summary) = sub_tools.browser_get_content_summary(json!({})).await
     |                                                                                                     ++++++

error[E0308]: mismatched types
    --> src\ai_tools.rs:3616:24
     |
3616 |                 if let Ok(h_lock) = h_loop.lock() {
     |                        ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                        |
     |                        expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-2646595386357108919.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3618:33
     |
3618 | ...   let _ = h_val.emit("subagent-progress", json!({ "id": tid_loop, "title": format!("Web Research: {}", t_...
     |               ^^^^^ cannot infer type

error[E0308]: mismatched types
    --> src\ai_tools.rs:3632:24
     |
3632 |                 if let Ok(h_lock) = h_loop.lock() {
     |                        ^^^^^^^^^^   ------------- this expression has type `impl Future<Output = MutexGuard<'_, Option<AppHandle>>>`
     |                        |
     |                        expected future, found `Result<_, _>`
     |
     = note: expected opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::option::Option<AppHandle>>>`
                       found enum `Result<_, _>`
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-17962288340899697826.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\ai_tools.rs:3634:33
     |
3634 |                         let _ = h_val.emit(
     |                                 ^^^^^ cannot infer type

error[E0277]: `Result<MutexGuard<'_, PatchEngine>, PoisonError<...>>` is not a future
   --> src\lib.rs:243:23
    |
243 |             pe.lock().await.set_app_handle(app_clone);
    |                       ^^^^^ `Result<MutexGuard<'_, PatchEngine>, PoisonError<...>>` is not a future
    |
    = help: the trait `futures::Future` is not implemented for `Result<MutexGuard<'_, PatchEngine>, PoisonError<...>>`
    = note: Result<MutexGuard<'_, PatchEngine>, PoisonError<...>> must be a future or must implement `IntoFuture` to be awaited
    = note: required for `Result<MutexGuard<'_, PatchEngine>, PoisonError<...>>` to implement `std::future::IntoFuture`
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-15108862721487832223.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: remove the `.await`
    |
243 -             pe.lock().await.set_app_handle(app_clone);
243 +             pe.lock().set_app_handle(app_clone);
    |

error[E0282]: type annotations needed
   --> src\lib.rs:243:13
    |
243 |             pe.lock().await.set_app_handle(app_clone);
    |             ^^^^^^^^^^^^^^^ cannot infer type

error[E0308]: mismatched types
   --> src\lib.rs:276:22
    |
276 |             Arc::new(std::sync::Mutex::new(Some(root.clone()))),
    |             -------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Mutex<Option<PathBuf>>`, found a different `Mutex<Option<PathBuf>>`
    |             |
    |             arguments to this function are incorrect
    |
    = note: `Mutex<Option<PathBuf>>` and `Mutex<Option<PathBuf>>` have similar names, but are actually distinct types
note: `Mutex<Option<PathBuf>>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `Mutex<Option<PathBuf>>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: associated function defined here
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\alloc\src\sync.rs:419:11

error[E0308]: mismatched types
   --> src\lib.rs:326:13
    |
326 |             patch_engine,
    |             ^^^^^^^^^^^^ expected `tauri::async_runtime::Mutex<PatchEngine>`, found `std::sync::Mutex<PatchEngine>`
    |
    = note: `std::sync::Mutex<PatchEngine>` and `tauri::async_runtime::Mutex<PatchEngine>` have similar names, but are actually distinct types
note: `std::sync::Mutex<PatchEngine>` is defined in crate `std`
   --> /rustc/e408947bfd200af42db322daf0fadfe7e26d3bd1/library\std\src\sync\poison\mutex.rs:227:0
note: `tauri::async_runtime::Mutex<PatchEngine>` is defined in crate `tokio`
   --> C:\Users\HADES\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\tokio-1.51.0\src\sync\mutex.rs:133:1
    |
133 | pub struct Mutex<T: ?Sized> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>` in the current scope
   --> src\lib.rs:724:36
    |
724 |     let eh = state.ext_host.lock().unwrap();
    |                                    ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, ExtensionHostManager>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-15450969683243031739.txt'
    = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>` in the current scope
   --> src\lib.rs:908:50
    |
908 |     let mut current = state.current_model.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, std::string::String>>`

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:915:50
    |
915 |     let mut current = state.advisor_model.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-6959074832518526328.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
915 |     let mut current = state.advisor_model.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:922:50
    |
922 |     let sdk_path = state.android_sdk_path.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-6959074832518526328.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
922 |     let sdk_path = state.android_sdk_path.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:960:49
    |
960 |     let mut active = state.active_device.lock().unwrap();
    |                                                 ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-6959074832518526328.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
960 |     let mut active = state.active_device.lock().await.unwrap();
    |                                                 ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:973:50
    |
973 |     let sdk_path = state.android_sdk_path.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-6959074832518526328.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
973 |     let sdk_path = state.android_sdk_path.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:990:49
    |
990 |     let mut sdk = state.android_sdk_path.lock().unwrap();
    |                                                 ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-6959074832518526328.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
990 |     let mut sdk = state.android_sdk_path.lock().await.unwrap();
    |                                                 ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, Option<String>>>` in the current scope
   --> src\lib.rs:997:50
    |
997 |     let sdk_path = state.android_sdk_path.lock().unwrap();
    |                                                  ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, Option<String>>>`
    |
    = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-6959074832518526328.txt'
    = note: consider using `--verbose` to print the full type name to the console
help: consider `await`ing on the `Future` and calling the method on its `Output`
    |
997 |     let sdk_path = state.android_sdk_path.lock().await.unwrap();
    |                                                  ++++++

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>` in the current scope
    --> src\lib.rs:2554:48
     |
2554 |     let mut debug = state.debug_manager.lock().unwrap();
     |                                                ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>` in the current scope
    --> src\lib.rs:2560:48
     |
2560 |     let mut debug = state.debug_manager.lock().unwrap();
     |                                                ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>` in the current scope
    --> src\lib.rs:2566:48
     |
2566 |     let mut debug = state.debug_manager.lock().unwrap();
     |                                                ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, DebugManager>>`

error[E0599]: no method named `unwrap` found for opaque type `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ActivationManager>>` in the current scope
    --> src\lib.rs:2572:50
     |
2572 |     let mut am = state.activation_manager.lock().unwrap();
     |                                                  ^^^^^^ method not found in `impl futures::Future<Output = tokio::sync::MutexGuard<'_, ActivationManager>>`

error[E0308]: mismatched types
    --> src\lib.rs:2579:5
     |
2578 | fn terminal_read_output(state: State<'_, EditorState>, id: String) -> Result<String, String> {
     |                                                                       ---------------------- expected `Result<std::string::String, std::string::String>` because of return type
2579 |     state.terminal_read_output(id)
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<String, String>`, found future

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:2591:57
     |
2591 |     let mut processes = state.terminal_processes.lock().unwrap();
     |                                                         ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-16654275474049790360.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:2593:17
     |
2593 |         let _ = child.kill();
     |                 ^^^^^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:2595:35
     |
2595 |     state.terminal_masters.lock().unwrap().remove(&id);
     |                                   ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-14173566796195036158.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:2596:35
     |
2596 |     state.terminal_writers.lock().unwrap().remove(&id);
     |                                   ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-17169552732892069906.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0599]: no method named `map_err` found for opaque type `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>` in the current scope
    --> src\lib.rs:2608:10
     |
2606 | /     tools
2607 | |         .editor_get_active_file(serde_json::json!({}))
2608 | |         .map_err(|e: anyhow::Error| e.to_string())
     | |         -^^^^^^^ method not found in `impl futures::Future<Output = Result<serde_json::Value, anyhow::Error>>`
     | |_________|
     |
     |
     = help: items from traits can only be used if the trait is in scope
help: consider `await`ing on the `Future` and calling the method on its `Output`
     |
2608 |         .await.map_err(|e: anyhow::Error| e.to_string())
     |          ++++++
help: trait `TryFutureExt` which provides `map_err` is implemented but not in scope; perhaps you want to import it
     |
   1 + use futures::TryFutureExt;
     |

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:2616:57
     |
2616 |     let mut processes = state.terminal_processes.lock().unwrap();
     |                                                         ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-16654275474049790360.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:2618:15
     |
2618 |         match child.try_wait() {
     |               ^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2620:68
     |
2620 |                 Ok(serde_json::json!({ "active": false, "success": status.success() }))
     |                                                                    ^^^^^^ cannot infer type

error[E0282]: type annotations needed
    --> src\lib.rs:2623:27
     |
2623 |             Err(e) => Err(e.to_string()),
     |                           ^ cannot infer type

error[E0599]: no method named `unwrap` found for opaque type `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>` in the current scope
    --> src\lib.rs:3002:69
     |
3002 |                     let mut writers = state.terminal_writers.lock().unwrap();
     |                                                                     ^^^^^^ method not found in `impl Future<Output = MutexGuard<'_, HashMap<String, ...>>>`
     |
     = note: the full name for the type has been written to 'C:\Users\HADES\Desktop\vscodium-rust\src-tauri\target\debug\deps\vscode_rust_app_lib-f2fbd94d955789fa.long-type-3990969016969100465.txt'
     = note: consider using `--verbose` to print the full type name to the console

error[E0282]: type annotations needed
    --> src\lib.rs:3013:37
     |
3013 | ...                   let _ = writer.write_all(data.as_bytes());
     |                               ^^^^^^ cannot infer type

Some errors have detailed explanations: E0277, E0282, E0308, E0599, E0728.
For more information about an error, try `rustc --explain E0277`.
warning: `vscode-rust-app` (lib) generated 5 warnings
error: could not compile `vscode-rust-app` (lib) due to 126 previous errors; 5 warnings emitted

C:\Users\HADES\Desktop\vscodium-rust\src-tauri>

*Viewed [tool_invoker.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/tool_invoker.rs) *

*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

*Viewed [ai_engine.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs) *

### Planner Response

Now I have a clear picture. Let me fix all the files systematically. Starting with the quick wins:

*Edited relevant file*

### User Input

Continue

*Grep searched codebase*

*Grep searched codebase*

### User Input

continue

*Grep searched codebase*

*Viewed [browser.rs](file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/browser.rs) *

### Planner Response

Good - [BrowserState](cci:2://file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/browser.rs:5:0-7:1) uses `std::sync::Mutex`, so all browser lock calls in [ai_tools.rs](cci:7://file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_tools.rs:0:0-0:0) must use `.lock().unwrap()` (NOT `.await`). Now let me batch-fix everything systematically.

**[ai_engine.rs](cci:7://file:///c:/Users/HADES/Desktop/vscodium-rust/src-tauri/src/ai_engine.rs:0:0-0:0) first** - setter methods and the remaining `.lock().unwrap()` patterns:

*Edited relevant file*