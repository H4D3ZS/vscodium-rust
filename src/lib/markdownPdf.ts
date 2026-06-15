/** Zero-dependency PDF export via the OS print dialog (Save as PDF). No bundle bloat. */

const PRINT_STYLES = `
  @page { margin: 18mm 16mm; size: auto; }
  * { box-sizing: border-box; }
  body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    font-size: 11pt;
    line-height: 1.55;
    color: #111;
    max-width: 720px;
    margin: 0 auto;
    padding: 0 4mm;
  }
  h1 { font-size: 1.6em; border-bottom: 1px solid #ddd; padding-bottom: 0.25em; }
  h2 { font-size: 1.25em; border-bottom: 1px solid #eee; padding-bottom: 0.2em; }
  h3 { font-size: 1.1em; }
  a { color: #2563eb; text-decoration: none; }
  code, pre { font-family: 'Cascadia Code', Consolas, monospace; font-size: 0.9em; }
  pre {
    background: #f4f4f5;
    border: 1px solid #e4e4e7;
    border-radius: 6px;
    padding: 10px 12px;
    overflow-x: auto;
    white-space: pre-wrap;
    word-break: break-word;
  }
  code { background: #f4f4f5; padding: 0.1em 0.35em; border-radius: 4px; }
  pre code { background: none; padding: 0; }
  blockquote {
    margin: 0 0 1em;
    padding: 0.4em 1em;
    border-left: 3px solid #d4d4d8;
    color: #52525b;
  }
  table { border-collapse: collapse; width: 100%; margin: 1em 0; }
  th, td { border: 1px solid #e4e4e7; padding: 6px 10px; text-align: left; }
  th { background: #f4f4f5; }
  img { max-width: 100%; height: auto; }
  hr { border: none; border-top: 1px solid #e4e4e7; margin: 1.5em 0; }
`;

function buildPrintDocument(title: string, bodyHtml: string): string {
    const safeTitle = title.replace(/[<>&"]/g, (c) => ({ '<': '&lt;', '>': '&gt;', '&': '&amp;', '"': '&quot;' }[c] || c));
    return `<!DOCTYPE html>
<html><head>
<meta charset="utf-8"/>
<title>${safeTitle}</title>
<style>${PRINT_STYLES}</style>
</head>
<body>${bodyHtml}</body></html>`;
}

/** Opens a print window — user picks "Save as PDF" or a printer. Adds ~0 KB to the app bundle. */
export function exportMarkdownToPdf(title: string, bodyHtml: string): boolean {
    const html = buildPrintDocument(title, bodyHtml);
    const frame = document.createElement('iframe');
    frame.style.position = 'fixed';
    frame.style.right = '0';
    frame.style.bottom = '0';
    frame.style.width = '0';
    frame.style.height = '0';
    frame.style.border = 'none';
    document.body.appendChild(frame);

    const doc = frame.contentDocument || frame.contentWindow?.document;
    if (!doc) {
        document.body.removeChild(frame);
        return false;
    }
    doc.open();
    doc.write(html);
    doc.close();

    const print = () => {
        try {
            frame.contentWindow?.focus();
            frame.contentWindow?.print();
        } finally {
            setTimeout(() => document.body.removeChild(frame), 1000);
        }
    };

    if (frame.contentWindow?.document.readyState === 'complete') {
        print();
    } else {
        frame.onload = print;
    }
    return true;
}
