# Email Newsletter
Zero2prod [book](https://www.zero2prod.com/index.html)

## Prerequisites
1. Rust, of course
2. Docker
3. sqlx-cli (install by using cargo install sqlx-cli --no-default-features --features rustls,postgres)

## Debugging

1. Install CodeLLDB extension [here](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
2. Create a `.vscode/launch.json` file
3. Write this to the `launch.json` file:
```json
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'zero2prod'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=zero2prod",
                    "--package=zero2prod"
                ],
                "filter": {
                    "name": "zero2prod",
                    "kind": "bin"
                }
            },
            "envFile": ".env",
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests in executable 'zero2prod'",
            "cargo": {
                "args": [
                    "test",
                    "--no-run",
                    "--bin=zero2prod",
                    "--package=zero2prod"
                ],
                "filter": {
                    "name": "zero2prod",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```
4. Press F5 or in the debugging menu, run the `Debug executable "zero2prod"`

Note:
if using symlink from windows to linux, add these lines to each of the configs on the `launch.json` file
```
    "sourceMap": {
        "<mountpoint>": "${workspaceFolder}"
    }
```
for example:
```
    "sourceMap": {
        "/mnt/d/Project/zero2prod": "${workspaceFolder}"
    }
```
