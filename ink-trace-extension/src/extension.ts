import * as vscode from 'vscode';
import { spawn } from 'child_process';
import * as path from 'path';

export function activate(context: vscode.ExtensionContext) {
    const outputChannel = vscode.window.createOutputChannel('Python Server Logs');
    const terminalName = 'Rust Tests';
    let rustTestTerminal: vscode.Terminal | undefined;

    context.subscriptions.push(
        vscode.commands.registerCommand('ink-trace.runRustTestWithPythonInit', async (args: { fileUri: vscode.Uri, testName: string }) => {
            outputChannel.show(true);
            outputChannel.appendLine('Starting Python server initialization...');

            try {
                await initializePythonProcess(outputChannel);
                outputChannel.appendLine(`Python is ready. Running test "${args.testName}"...\n`);

                if (!rustTestTerminal) {
                    rustTestTerminal = vscode.window.createTerminal(terminalName);
                }

                rustTestTerminal.show();
                rustTestTerminal.sendText(`cd "${path.dirname(args.fileUri.fsPath)}"`);
                rustTestTerminal.sendText(`cargo test -- --nocapture ${args.testName}`);
            } catch (err: any) {
                vscode.window.showErrorMessage(`Failed to initialize Python: ${err.message}`);
                outputChannel.appendLine(`Python error: ${err.message}`);
            }
        }),

        vscode.languages.registerCodeLensProvider(
            { language: 'rust' },
            new RustTestCodeLensProvider()
        )
    );
}

async function initializePythonProcess(outputChannel: vscode.OutputChannel): Promise<void> {
    return new Promise((resolve, reject) => {
        outputChannel.appendLine('Connecting to Python (mock)...');

        setTimeout(() => {
            outputChannel.appendLine('SERVER_READY (mock)');
            resolve();
        }, 3000);

        /*
        // Uncomment this block to run a real Python script and wait for "SERVER_READY"
        const pythonPath = 'python'; // or 'python3'
        const scriptPath = path.join(__dirname, 'scripts/server.py');

        const process = spawn(pythonPath, [scriptPath]);

        process.stdout.on('data', (data) => {
            const output = data.toString();
            outputChannel.append(output);
            if (output.includes('SERVER_READY')) {
                resolve();
            }
        });

        process.stderr.on('data', (data) => {
            outputChannel.append(data.toString());
        });

        process.on('error', (err) => {
            reject(err);
        });

        process.on('exit', (code) => {
            if (code !== 0) {
                reject(new Error(`Python exited with code ${code}`));
            }
        });
        */
    });
}

class RustTestCodeLensProvider implements vscode.CodeLensProvider {
    provideCodeLenses(document: vscode.TextDocument): vscode.CodeLens[] {
        const codeLenses: vscode.CodeLens[] = [];
        const regex = /#\[test\]\s*?\n\s*?fn\s+([a-zA-Z0-9_]+)\s*?\(/g;
        const text = document.getText();
        let match: RegExpExecArray | null;

        while ((match = regex.exec(text))) {
            const testName = match[1];
            const position = document.positionAt(match.index);
            const range = new vscode.Range(position, position);

            codeLenses.push(new vscode.CodeLens(range, {
                title: '▶ Run test',
                command: 'ink-trace.runRustTestWithPythonInit',
                arguments: [{ fileUri: document.uri, testName }]
            }));
        }

        return codeLenses;
    }
}
