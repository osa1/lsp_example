# An actually minimal VSCode LSP extension demo, with a server in Rust

This repo contains an actually minimal language server extension for VSCode,
and a Rust language server implementation.

Most other "samples" and "minimal examples" online require 20 JS command line
tools and aim at hardcore 10x JS web-devs with 25 years of experience.

This repo contains only 3 files for the VSCode extension, and it's actually
"minimal": if you remove any line of code it won't work.

(If you find a line that can be removed from the VSCode extension, please let
me know in the issues or even better, open a PR!)

The server, implemented in Rust, is not minimal: it handles all LSP events that
the LSP library (tower-lsp) allows handling, to print all requests to a file.
The server does not need to be minimal as it's a simple Rust program: trivial
to build and run.

## The setup

There are two hard-coded things in the project that you may want to modify
before running it:

- `vscode_extension/extension.js` has hard-coded path to the server executable.
  Update it based on where you clone this repo.

- The server writes all logs to `$HOME/lsp_logs`. You may want to modify the
  path in `server/src/main.rs`.

## Building

- Build the server with `(cd server; cargo build)`.

  This step generates the server executable at
  `server/target/debug/lsp_server_example`. Copy the absolute path to this to
  `extension.js`, as mentioned above.

- Download JS dependencies with `(cd vscode_extension; npm install)`.

  Note: This command does not install anything to your system, just downloads
  dependencies in `node_modules` directory.

## Running

### From within VSCode (for development)

Run VSCode in the extension directory with `code .`. Navigate to "Run and
Debug" tab on the left, then run the "Run extension" configuration.

A new VSCode window will appear, this new VSCode will have the extension
installed. Open a plain text file. In the "Debug console" of the original
VSCode you should see a line `Activating lsp-example...`. As you edit the file
the log file at `$HOME/lsp_logs` will be updated by the server with the
incoming messages and requests.

### Installing the extension

Note: if you're installing VSCode from the tarball from the [official site][1],
you need to install the CLI as well (right below the tarball link). The `code`
executable in instructions below should run the CLI, not the `code` executable
from the tarball.

In the `vscode_extension` directory: run

1. `npm install`
2. `npx vsce package`

Step (2) will generate a file named `lsp-example-0.0.1.vsix`.

After this, you have two options:

1. `code --install-extension lsp-example-0.0.1.vsix`.

   After this step you should see the extension listed in `code
   --list-extensions`.

2. Run `code`, switch to the "extensions" tab on the left, click on "..." on
   the top right, select "Install from VSIX" and point it to
   `lsp-example-0.0.1.vsix`.

   After this step the extension should be listed in "Installed" section in
   the extensions tab.

[1]: https://code.visualstudio.com/download
