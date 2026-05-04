import re
import os

src_dir = r'c:\Users\HADES\Desktop\vscodium-rust\src-tauri\src'
commands_by_file = {}

for root, dirs, files in os.walk(src_dir):
    for file in files:
        if file.endswith('.rs'):
            path = os.path.join(root, file)
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()
                # Updated regex to handle 'pub async fn' and 'pub fn'
                cmds = re.findall(r'#\[tauri::command\].*?(?:pub\s+)?(?:async\s+)?fn\s+(\w+)', content, re.DOTALL)
                if cmds:
                    commands_by_file[file] = cmds

lib_path = r'c:\Users\HADES\Desktop\vscodium-rust\src-tauri\src\lib.rs'

with open(lib_path, 'r', encoding='utf-8') as f:
    lib_content = f.read()

# Build invoke_handler with correct paths
imported_cmds = set()
import_matches = re.findall(r'use\s+[\w:]+::\{([^}]+)\}', lib_content, re.DOTALL)
for match in import_matches:
    for cmd in match.split(','):
        cmd = cmd.strip()
        if cmd:
            cmd = re.sub(r'//.*', '', cmd).strip()
            if cmd:
                imported_cmds.add(cmd)

import_matches_single = re.findall(r'use\s+[\w:]+::(\w+);', lib_content)
for match in import_matches_single:
    imported_cmds.add(match)

import_matches_crate = re.findall(r'use\s+crate::[\w:]+::\{([^}]+)\}', lib_content, re.DOTALL)
for match in import_matches_crate:
    for cmd in match.split(','):
        cmd = cmd.strip()
        if cmd:
            cmd = re.sub(r'//.*', '', cmd).strip()
            if cmd:
                imported_cmds.add(cmd)

invoke_list = []
for file, cmds in commands_by_file.items():
    mod_name = file.replace('.rs', '')
    for cmd in cmds:
        if file == 'lib.rs' or cmd in imported_cmds:
            invoke_list.append(cmd)
        else:
            invoke_list.append(f"{mod_name}::{cmd}")

invoke_list = sorted(list(set(invoke_list)))

invoke_handler = "            .invoke_handler(tauri::generate_handler![\n"
for cmd in invoke_list:
    invoke_handler += f"                {cmd},\n"
invoke_handler += "            ])"

run_function = """
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let filter = EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into());

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();

    std::panic::set_hook(Box::new(|info| {
        let payload = info.payload();
        let message = if let Some(s) = payload.downcast_ref::<&str>() {
            *s
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic"
        };
        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());
        let panic_msg = format!("[CRITICAL PANIC] {} at {}\\n", message, location);
        eprintln!("{}", panic_msg);
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            println!("[DEBUG] Tauri setup starting...");
            let state = EditorState::new(app.handle());
            println!("[DEBUG] EditorState created successfully");
            app.manage(state);
            Ok(())
        })
""" + invoke_handler + """
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
"""

pattern = re.compile(r'#\[cfg_attr\(mobile, tauri::mobile_entry_point\)\]\s*pub fn run\(\).*?\.expect\("[^"]+"\);\s*\}', re.DOTALL)
cleaned_content = pattern.sub("", lib_content)
pattern2 = re.compile(r'pub fn run\(\).*?\.expect\("[^"]+"\);\s*\}', re.DOTALL)
cleaned_content = pattern2.sub("", cleaned_content)

final_content = cleaned_content.strip() + "\n\n" + run_function

with open(lib_path, 'w', encoding='utf-8') as f:
    f.write(final_content)

print(f"Reconstructed lib.rs and removed duplicates. Total commands: {len(invoke_list)}")
