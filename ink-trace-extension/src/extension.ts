import * as vscode from "vscode";
import * as path from "path";

export function activate(context: vscode.ExtensionContext) {
   const factory = new InkTraceDebugAdapterDescriptorFactory(context);
   context.subscriptions.push(
      vscode.debug.registerDebugAdapterDescriptorFactory("ink-trace", factory),
   );

   const provider = new InkTraceConfigurationProvider();
   context.subscriptions.push(
      vscode.debug.registerDebugConfigurationProvider("ink-trace", provider),
   );
}

class InkTraceDebugAdapterDescriptorFactory
   implements vscode.DebugAdapterDescriptorFactory {
   constructor(private context: vscode.ExtensionContext) {}

   createDebugAdapterDescriptor(
      session: vscode.DebugSession,
      executable: vscode.DebugAdapterExecutable | undefined,
   ): vscode.ProviderResult<vscode.DebugAdapterDescriptor> {
      const adapterPath = path.join(
         this.context.extensionPath,
         "..",
         "ink-dap-server",
         "target",
         "release",
         "ink-dap-server",
      );

      return new vscode.DebugAdapterExecutable(adapterPath, []);
   }
}

class InkTraceConfigurationProvider
   implements vscode.DebugConfigurationProvider {
   resolveDebugConfiguration(
      folder: vscode.WorkspaceFolder | undefined,
      config: vscode.DebugConfiguration,
      token?: vscode.CancellationToken,
   ): vscode.ProviderResult<vscode.DebugConfiguration> {
      if (!config.type && !config.request && !config.name) {
         const editor = vscode.window.activeTextEditor;
         if (editor && editor.document.languageId === "rust") {
            config.type = "ink-trace";
            config.name = "Launch";
            config.request = "launch";
            config.program = "${workspaceFolder}";
            config.stopOnEntry = false;
         }
      }

      if (config.program && config.program.includes("/target/debug/")) {
         config.program = "${workspaceFolder}";
      }

      if (!config.program) {
         return vscode.window.showInformationMessage(
            "Cannot find a program to debug",
         ).then((_) => {
            return undefined;
         });
      }

      const outputChannel = vscode.window.createOutputChannel(
         "Ink Trace Debug",
      );
      outputChannel.show();
      outputChannel.appendLine(
         `Starting debug session with config: ${
            JSON.stringify(config, null, 2)
         }`,
      );

      return config;
   }
}

export function deactivate() {}
