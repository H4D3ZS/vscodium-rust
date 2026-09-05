//! Standalone Mermaid diagram viewer export (Antigravity-style).
//!
//! Writes a self-contained `index.html` + `style.css` + `app.js` into a folder:
//! a glassmorphic, tabbed viewer with zoom/pan, copy-to-clipboard and toasts,
//! rendering the supplied Mermaid diagrams via the Mermaid CDN. No build step —
//! `index.html` opens directly in any browser.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagramInput {
    pub title: String,
    pub code: String,
    #[serde(default)]
    pub description: Option<String>,
}

const STYLE_CSS: &str = r#":root{--bg:#0b1020;--panel:rgba(255,255,255,0.05);--border:rgba(255,255,255,0.12);--fg:#e5e9f0;--muted:#9aa4b2;--accent:#6366f1;--accent2:#38bdf8}
*{box-sizing:border-box}
html,body{margin:0;height:100%;font-family:Inter,system-ui,-apple-system,Segoe UI,sans-serif;background:radial-gradient(1200px 600px at 20% -10%,#1e1b4b 0%,transparent 60%),radial-gradient(1000px 500px at 100% 0%,#082f49 0%,transparent 55%),var(--bg);color:var(--fg)}
.app{display:flex;flex-direction:column;height:100%}
header{display:flex;align-items:center;gap:12px;padding:16px 22px;border-bottom:1px solid var(--border);backdrop-filter:blur(12px)}
header h1{font-size:16px;margin:0;font-weight:600;letter-spacing:.2px}
.tabs{display:flex;gap:6px;flex-wrap:wrap;padding:12px 18px;border-bottom:1px solid var(--border)}
.tab{padding:7px 14px;border-radius:10px;border:1px solid var(--border);background:var(--panel);color:var(--muted);cursor:pointer;font-size:13px;transition:.15s}
.tab:hover{color:var(--fg)}
.tab.active{color:#fff;border-color:transparent;background:linear-gradient(135deg,var(--accent),var(--accent2))}
.stage{flex:1;position:relative;overflow:hidden;background-image:radial-gradient(rgba(255,255,255,.05) 1px,transparent 1px);background-size:24px 24px}
.canvas{position:absolute;inset:0;display:flex;align-items:center;justify-content:center;cursor:grab}
.canvas svg{max-width:none}
.toolbar{position:absolute;top:14px;right:14px;display:flex;gap:6px}
.btn{display:inline-flex;align-items:center;gap:6px;padding:8px 12px;border-radius:10px;border:1px solid var(--border);background:var(--panel);color:var(--fg);cursor:pointer;font-size:12px;backdrop-filter:blur(10px);transition:.15s}
.btn:hover{border-color:var(--accent2)}
.toast{position:fixed;bottom:24px;left:50%;transform:translateX(-50%) translateY(20px);opacity:0;background:rgba(17,24,39,.95);border:1px solid var(--border);color:#fff;padding:10px 16px;border-radius:12px;font-size:13px;transition:.25s;pointer-events:none}
.toast.show{opacity:1;transform:translateX(-50%) translateY(0)}
foreignObject div{white-space:nowrap!important}
"#;

const APP_JS_TMPL: &str = r#"const DIAGRAMS = /*__DIAGRAMS__*/;
let active = 0, zoom = 1, pan = {x:0,y:0}, drag = null;
const stage = document.getElementById('canvas');
const tabsEl = document.getElementById('tabs');
function toast(m){const t=document.getElementById('toast');t.textContent=m;t.classList.add('show');setTimeout(()=>t.classList.remove('show'),1600);}
function applyTransform(){stage.firstChild && (stage.firstChild.style.transform=`translate(${pan.x}px,${pan.y}px) scale(${zoom})`);}
async function render(){
  const d = DIAGRAMS[active]; if(!d) return;
  zoom=1; pan={x:0,y:0};
  try{
    const {svg} = await mermaid.render('g'+active+'_'+Date.now(), d.code);
    stage.innerHTML='<div class="diagram" style="transform-origin:center center">'+svg+'</div>';
  }catch(e){ stage.innerHTML='<pre style="color:#f87171;padding:20px;white-space:pre-wrap">'+String(e&&e.message||e)+'</pre>'; }
}
function buildTabs(){
  tabsEl.innerHTML='';
  DIAGRAMS.forEach((d,i)=>{const b=document.createElement('button');b.className='tab'+(i===active?' active':'');b.textContent=d.title;b.onclick=()=>{active=i;buildTabs();render();};tabsEl.appendChild(b);});
}
stage.addEventListener('wheel',e=>{if(!e.ctrlKey&&!e.metaKey)return;e.preventDefault();zoom=Math.min(5,Math.max(.2,zoom-e.deltaY*.0015));applyTransform();},{passive:false});
stage.addEventListener('mousedown',e=>{drag={x:e.clientX,y:e.clientY,px:pan.x,py:pan.y};});
window.addEventListener('mousemove',e=>{if(!drag)return;pan={x:drag.px+(e.clientX-drag.x),y:drag.py+(e.clientY-drag.y)};applyTransform();});
window.addEventListener('mouseup',()=>drag=null);
document.getElementById('zin').onclick=()=>{zoom=Math.min(5,zoom+.2);applyTransform();};
document.getElementById('zout').onclick=()=>{zoom=Math.max(.2,zoom-.2);applyTransform();};
document.getElementById('reset').onclick=()=>{zoom=1;pan={x:0,y:0};applyTransform();};
document.getElementById('copy').onclick=()=>{navigator.clipboard.writeText(DIAGRAMS[active].code);toast('Source copied');};
mermaid.initialize({startOnLoad:false,theme:'base',securityLevel:'strict',fontFamily:'Inter,system-ui,sans-serif',flowchart:{htmlLabels:true,curve:'basis',padding:18,nodeSpacing:55,rankSpacing:70},themeVariables:{background:'transparent',primaryColor:'#1e293b',primaryBorderColor:'#6366f1',primaryTextColor:'#e5e9f0',lineColor:'#64748b',textColor:'#cbd5e1',clusterBkg:'rgba(255,255,255,0.03)',clusterBorder:'rgba(255,255,255,0.12)',edgeLabelBackground:'#0b1020',fontSize:'14px'}});
buildTabs();render();
"#;

fn index_html(title: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="en"><head>
<meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>{title}</title><link rel="stylesheet" href="style.css">
<script src="https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.min.js"></script>
</head><body>
<div class="app">
  <header><h1>{title}</h1></header>
  <div class="tabs" id="tabs"></div>
  <div class="stage">
    <div class="toolbar">
      <button class="btn" id="zout">-</button>
      <button class="btn" id="zin">+</button>
      <button class="btn" id="reset">Reset</button>
      <button class="btn" id="copy">Copy source</button>
    </div>
    <div class="canvas" id="canvas"></div>
  </div>
</div>
<div class="toast" id="toast"></div>
<script src="app.js"></script>
</body></html>
"#)
}

/// Resolve the OS app-data location for the generated viewer so it never lands
/// in the user's project tree (the way Gemini/Claude tools keep their state out
/// of the workspace). Per-project subfolder keeps diagrams from different repos
/// separate.
///   Windows: %LOCALAPPDATA%\vscodium-rust\diagram_viewer
///   Linux:   ~/.local/share/vscodium-rust/diagram_viewer
///   macOS:   ~/Library/Application Support/vscodium-rust/diagram_viewer
fn resolve_viewer_dir(project: Option<&str>) -> PathBuf {
    let mut base = dirs::data_local_dir()
        .or_else(dirs::config_dir)
        .unwrap_or_else(std::env::temp_dir)
        .join("vscodium-rust")
        .join("diagram_viewer");
    if let Some(proj) = project.map(str::trim).filter(|s| !s.is_empty()) {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        proj.hash(&mut h);
        base = base.join(format!("{:016x}", h.finish()));
    }
    base
}

/// Write a standalone HTML/CSS/JS Mermaid viewer to the app-data dir (NOT the
/// project). `project` namespaces the output per-repo; `dir_override` forces an
/// explicit path if ever needed. Returns the index.html path.
#[tauri::command]
pub fn export_diagram_viewer(
    diagrams: Vec<DiagramInput>,
    project: Option<String>,
    dir_override: Option<String>,
) -> Result<String, String> {
    if diagrams.is_empty() {
        return Err("No diagrams to export".to_string());
    }
    let base = match dir_override.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()) {
        Some(d) => PathBuf::from(d),
        None => resolve_viewer_dir(project.as_deref()),
    };
    std::fs::create_dir_all(&base).map_err(|e| format!("create dir: {}", e))?;

    let diagrams_json = serde_json::to_string(&diagrams).map_err(|e| e.to_string())?;
    let app_js = APP_JS_TMPL.replace("/*__DIAGRAMS__*/", &diagrams_json);
    let title = "Diagram Viewer";

    std::fs::write(base.join("style.css"), STYLE_CSS).map_err(|e| format!("write css: {}", e))?;
    std::fs::write(base.join("app.js"), app_js).map_err(|e| format!("write js: {}", e))?;
    let index = base.join("index.html");
    std::fs::write(&index, index_html(title)).map_err(|e| format!("write html: {}", e))?;

    Ok(index.to_string_lossy().to_string())
}
