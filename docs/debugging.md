# Debugging

## VS Code

Install the following vscode extensions depending on the platform:

- C/C++ (Windows)
- CodeLLDB (MacOS / Linux)
- Rust

### Configure VS Code

Goto the `Run & Debug` tab and add a new `launch.json` configuration to something like this.

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "(Windows) Launch",
      "type": "cppvsdbg",
      "request": "launch",
      "program": "${workspaceRoot}/target/debug/atlantis.exe",
      "args": [],
      "stopAtEntry": false,
      "cwd": "${workspaceRoot}",
      "environment": [],
      "console": "externalTerminal"
    },
    {
      "name": "(MacOS) Launch",
      "type": "lldb",
      "request": "launch",
      "program": "${workspaceRoot}/target/debug/atlantis",
      "args": [],
      "stopAtEntry": false,
      "cwd": "${workspaceRoot}"
    }
  ]
}
```

Then goto File > Preferences > Settings and search for the setting under Debug: `Debug: Allow Breakpoints Everywhere` and toggle the `Allow setting breakpoints in any file` setting.

Add a breakpoint, press F5 to launch.

### References

<https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/>
