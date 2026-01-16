<div align="center">

# Zed-Thrift-Next


</div>

> [!TIP]
> `zed-thrift-next` is an independent extension project and has no dependency relationship with `zed-thrift`

> [!caution]
> When using in `zed`, you may encounter an `Error: resolving completion` error. This is not an issue with the extension itself, and functionality remains unaffected. I am actively communicating with the `thriftls` author to understand this issue.


> [!caution]
> You cannot use both `zed-thrift` and `zed-thrift-next` simultaneously. Please uninstall the `zed-thrift` extension before installing `zed-thrift-next`


## Features

- **Auto Completion**
- **Code Highlighting**
- **Syntax Checking**
- **Definition Jumping**
- **Code Formatting**

## Why Zed-Thrift-Next?

The original `zed-thrift` extension has the following issues:

1. It does not automatically download the `thriftls` service, and there is no documentation indicating how to find the correct `lsp` server
2. The extension uses incorrect parameters, causing the `lsp` server to fail to start even when trying two different `thrift` services
3. It only works for highlighting, but incorrect data leads to frequent errors

`zed-thrift-next` solves these problems by performing the following operations:

1. Check if the user has already configured an existing `thrift` server; if so, use the user configuration without checking, downloading, or updating
2. Check if the user has installed the `thrift` server and configured it in `$PATH`; if so, skip checking, downloading, and updating
3. If the user has not installed the `thrift` server, automatically download it from `github`
4. If an old version exists, automatically update and delete the old version to keep it up-to-date


## Installation

### Install from `zed` extensions

Search for `Thrift Next` in `zed` to install

### Install from source code

> Make sure you have installed `rust`, and it was installed and updated using `rustup`, this is required by `zed`

- Download current project using `git clone`
- Open `zed` editor, enter command: `zed: install dev extension`
- In the opened window, select the current project directory

## Notes

1. `zed-thrift-next` extension downloads binary files from `github`, you need to ensure network connectivity
2. Installing the extension will automatically download the `thriftls` binary file, if it fails, try uninstalling and reinstalling
3. If installation cannot proceed due to network or other issues, you can manually download the binary file and place it in `$PATH`, you can find the binary and download it [here](https://github.com/joyme123/thrift-ls/releases/tag/v0.2.9)
4. You don't need to worry about the binary name issue, you can use `thriftls` or directly put the downloaded binary into `$PATH`, the extension will handle compatibility
