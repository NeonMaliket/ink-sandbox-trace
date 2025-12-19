# Step-Debugger for Ink! v6 - Testing Guide  

This guide describes how to **manually test** the `ink-trace-extension` end-to-end inside VSCode.
The project is built on Ink! v6 — make sure to use cargo darkly for all builds and runs to ensure compatibility.

## 1. Launch the Python Backend
1. Open a terminal and **navigate to**: `ink-trace-extension/ink-dap-server`
2. Run the backend:
   ```bash
   python3 main.py
    ```
3.	You should see logs confirming that the Python DAP server has started.
This is our backend for the debugger — it will handle and process debug commands from the VSCode Extension.
The server will stay running in the background and respond to incoming requests.


## 2. Launch the Extension
1. Open VSCode and **open the `ink-trace-extension/` folder**.
2. Run command ```npm install```
3. Run command ```npm run compile```
4. Press `F5` or run **“Launch Extension”** from the Run menu. This starts the extension and opens a **new VSCode window** (Extension Host).


## 3. Open the Sample Workspace
1. In the **Extension Host window**, open:      `ink-trace-extension/sampleWorkspace`
2. Wait until the workspace finishes building. 
You should see the **“Run Test | Debug”** buttons above each DRink test in `lib.rs`.


## 4. Run a DRink Test
1.	Press F5 or run **“Launch”** from the Run menu. This starts the debug session — you’ll see confirmation in the Python backend logs. Since breakpoints and step functionality are not yet implemented, you can stop the session at this point.
2. Click **“Run Test”** on any test (or press Run button from the Run menu).  
Wait for the build in the terminal to complete.
3. This triggers the **DRink test pipeline**:
- **Macro execution** - starts the DRink flow.  
- **`cargo-contract build`** - compiles all contract blobs.  
- **Contract execution** - runs inside our custom environment built on top of **DRink**.


## 5. Observe Logs
1. Open the **console output** in VSCode (Terminal).
2. You should see logs with [ink_debug_rpc::sandbox_rpc] prefix
3. These logs come from our **custom sandbox RPC** - each log line corresponds to a **program counter (step)** in contract execution.
4. You should also see logs confirming that the **Python DAP server** is running and responding — this verifies the extension is properly connected to the backend.

**Success Criteria**
- Python DAP server launches successfully and stays active.
- Extension launches correctly.
- Sample workspace builds.
- DRink test runs end-to-end.
- Console shows `[ink_debug_rpc::sandbox_rpc]` logs with step info.
- Python backend logs confirm it’s active and ready to handle debug commands.