# Region 15 Scholarship Application (Rust Rewrite)

## Requirements
This repo requires that you have `cargo`, `clang`, and `node` installed.

- `cargo`: the project is written in Rust.
- `clang`: `ring`, a dependency of the `leptos_oidc` crate, requires this for compilation.
- `node`: required for `tailwindcss` compilation.

## How to run
Run the command `cargo leptos watch` to compile and run an instance of the server on your local machine. To access info
from the AWS resources, you will need to be approved by me or someone else with AWS administrator access to obtain a 
usable auth key from AWS IAM.

## Current Tasks
There are several tasks left to complete. These are a few of the larger ones:

- Build the application's custom elements
- Build out the database requests
- Create proper authentication
- PDF generation using data (server-side)