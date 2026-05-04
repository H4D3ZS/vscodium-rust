exports.activate = function (context) {
    console.log("Hello World extension activated!");

    vscode.commands.registerCommand("extension.sayHello", () => {
        vscode.window.showInformationMessage("Hello from our Rust-based VSCodium rewrite!");
    });

    vscode.window.showInformationMessage("Hello World extension is now active!");

    vscode.workspace.onDidChangeTextDocument((event) => {
        console.error(`Document changed: ${event.document.uri}`);
        // In a real editor, this would trigger diagnostics or autocomplete updates
    });
};
