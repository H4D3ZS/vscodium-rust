import re
import os

src_dir = r'c:\Users\HADES\Desktop\vscodium-rust\src-tauri\src'
commands = []

for root, dirs, files in os.walk(src_dir):
    for file in files:
        if file.endswith('.rs'):
            path = os.path.join(root, file)
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()
                cmds = re.findall(r'#\[tauri::command\]\s*async fn\s+(\w+)', content)
                for cmd in cmds:
                    # For commands in modules, we need to know how they are imported
                    commands.append((file, cmd))

for file, cmd in commands:
    print(f"{file}: {cmd}")
