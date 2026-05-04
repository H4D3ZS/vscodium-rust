const vscode = require('vscode');

function activate(context) {
    vscode.window.showInformationMessage('Lazy Rust extension ACTIVATED! 🦀');
    console.error('Lazy Rust extension activated by onLanguage:rust');
}

function deactivate() { }

module.exports = {
    activate,
    deactivate
}
