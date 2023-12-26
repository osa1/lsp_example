const vscode = require('vscode');
const node = require("vscode-languageclient/node");

let client;

function activate(context) {
    console.log('Activating lsp-example...');
    const command = '/home/omer/rust/lsp_example/server/target/debug/lsp_server_example';
    const run = {
        command,
    };
    const serverOptions = {
        run,
        debug: run,
    };
    const clientOptions = {
        documentSelector: [{ scheme: 'file', language: 'plaintext' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/.clientrc')
        }
    };
    client = new node.LanguageClient('languageServerExample', 'Language Server Example', serverOptions, clientOptions);
    client.start();
}

function deactivate() {
    console.log('Deactivating lsp-example...');
    if (!client) {
        return undefined;
    }
    return client.stop();
}

module.exports = {
    activate,
    deactivate
}
