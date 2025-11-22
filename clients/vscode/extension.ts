// Universal Language Connector - VS Code Client
// ~70 LOC - All logic delegated to LSP server

import * as vscode from 'vscode';
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  // Server executable configuration
  const serverOptions: ServerOptions = {
    command: 'universal-connector-server',
    transport: TransportKind.stdio,
    args: []
  };

  // Client configuration - support all file types
  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: 'file', language: 'markdown' },
      { scheme: 'file', language: 'html' },
      { scheme: 'file', language: 'json' },
      { scheme: 'file', pattern: '**/*.{md,html,json}' }
    ],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher('**/*.{md,html,json}')
    }
  };

  // Create and start LSP client
  client = new LanguageClient(
    'universalConnector',
    'Universal Language Connector',
    serverOptions,
    clientOptions
  );

  // Register commands
  context.subscriptions.push(
    vscode.commands.registerCommand('universalConnector.convertToHtml', async () => {
      const editor = vscode.window.activeTextEditor;
      if (editor) {
        await client.sendRequest('workspace/executeCommand', {
          command: 'convert.toHtml',
          arguments: [editor.document.uri.toString()]
        });
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand('universalConnector.convertToMarkdown', async () => {
      const editor = vscode.window.activeTextEditor;
      if (editor) {
        await client.sendRequest('workspace/executeCommand', {
          command: 'convert.toMarkdown',
          arguments: [editor.document.uri.toString()]
        });
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand('universalConnector.convertToJson', async () => {
      const editor = vscode.window.activeTextEditor;
      if (editor) {
        await client.sendRequest('workspace/executeCommand', {
          command: 'convert.toJson',
          arguments: [editor.document.uri.toString()]
        });
      }
    })
  );

  // Start the client
  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
