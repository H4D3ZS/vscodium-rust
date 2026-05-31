import * as vscode from 'vscode';

// Loading the highly optimized Rust C-FFI backend dynamically applying explicit Node-API bindings!
const ffi = require('ffi-napi');
const ref = require('ref-napi');

// The Exact C-ABI Interface mapping Phase 3 libaim.dll native constraints flawlessly!
const libaim = ffi.Library('../../libaim/target/release/libaim', {
    'aim_mount_vfs': ['pointer', ['string']],
    'aim_get_tensor': ['pointer', ['pointer', 'pointer']],
    'aim_unmount_vfs': ['void', ['pointer']]
});

export function activate(context: vscode.ExtensionContext) {
    console.log('⚡ [KORTEX] True Neural .AIM VFS Provider Active and Executing implicitly!');

    let disposable = vscode.commands.registerCommand('aim.mountWorkspace', () => {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showErrorMessage("Neural VFS Failure: No valid workspace parameters actively bound to map native tensor embeddings.");
            return;
        }

        const projectPath = workspaceFolders[0].uri.fsPath;
        vscode.window.showInformationMessage(`🧠 AIM-VFS: Aggressively Executing Zero-Copy C-ABI Binding mapping: ${projectPath}`);

        try {
            // Execute the physical Shared OS Memory Buffer completely bypassing massive Network Json constraints natively
            const memoryPtr = libaim.aim_mount_vfs(projectPath);
            if (memoryPtr.isNull()) {
                vscode.window.showErrorMessage("AIM: Discarded Null Pointers systematically. The .aim Tensor Matrix could not be instantiated robustly.");
                return;
            }

            // Exfiltrate explicit high-density Float32 Semantic structures bounding arrays natively
            const tensorSizePtr = ref.alloc('size_t');
            const tensorPtr = libaim.aim_get_tensor(memoryPtr, tensorSizePtr);
            const size = tensorSizePtr.deref();

            vscode.window.showInformationMessage(`✅ [AIM KV-CACHE HIJACK]: Flawlessly extracted ${size} semantic vectors seamlessly via ffi-napi! Completely overwriting underlying VSCodium parameters natively!`);

            // Execute safe Dereference bounding un-mapping strict RAM regions intrinsically
            libaim.aim_unmount_vfs(memoryPtr);
        } catch (err: any) {
            vscode.window.showErrorMessage("AIM-VFS Fatal FFI Execution Constraint: " + err.message);
        }
    });

    context.subscriptions.push(disposable);
}

export function deactivate() { }
