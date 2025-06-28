import * as vscode from 'vscode';
import { InkDebugSession } from './inkDebugSession';

export function activate(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.debug.registerDebugAdapterDescriptorFactory(
            'ink-trace',
            new InkDebugAdapterDescriptorFactory()
        ),
        vscode.debug.registerDebugConfigurationProvider(
            'ink-trace',
            new InkDebugConfigurationProvider()
        ),
        vscode.commands.registerCommand(
            'ink-trace.runRustTestWithPythonInit',
            async (uri: vscode.Uri, range: vscode.Range) => {
                vscode.window.showInformationMessage('Waiting for Python initialization...');

                // Тут можна викликати Python або інший процес
                await new Promise(resolve => setTimeout(resolve, 2000)); // емулюємо затримку

                vscode.window.showInformationMessage('Python initialized! Running test...');

                // Запускаємо стандартну rust-analyzer команду для запуску тесту
                await vscode.commands.executeCommand('rust-analyzer.runSingle', uri, range);
            }
        ),
        vscode.languages.registerCodeLensProvider(
            { language: 'rust' },
            new RustTestCodeLensProvider()
        )
    );
}

export function deactivate() {}

class InkDebugConfigurationProvider implements vscode.DebugConfigurationProvider {
    resolveDebugConfiguration(
        _folder: vscode.WorkspaceFolder | undefined,
        config: vscode.DebugConfiguration
    ): vscode.ProviderResult<vscode.DebugConfiguration> {
        if (!config.program) {
            config.program = '${file}';
        }
        return config;
    }
}

class InkDebugAdapterDescriptorFactory implements vscode.DebugAdapterDescriptorFactory {
    createDebugAdapterDescriptor(session: vscode.DebugSession): vscode.ProviderResult<vscode.DebugAdapterDescriptor> {
        return new vscode.DebugAdapterInlineImplementation(new InkDebugSession());
    }
}

class RustTestCodeLensProvider implements vscode.CodeLensProvider {
    provideCodeLenses(document: vscode.TextDocument): vscode.CodeLens[] {
        const codeLenses: vscode.CodeLens[] = [];
        const regex = /#\[test\]/g;
        const text = document.getText();
        let match: RegExpExecArray | null;

        while ((match = regex.exec(text))) {
            const matchPos = document.positionAt(match.index);
            const nextLine = new vscode.Position(matchPos.line + 1, 0);
            const range = new vscode.Range(nextLine, nextLine); // під функцією
            codeLenses.push(new vscode.CodeLens(range, {
                title: 'Run with Python Init',
                command: 'ink-trace.runRustTestWithPythonInit',
                arguments: [document.uri, range]
            }));
        }

        return codeLenses;
    }
}
