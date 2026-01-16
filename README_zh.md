<div>

# Zed-Thrift-Next

</div>

> [!TIP]
> `zed-thrift-next` 与 `zed-thrift` 没有任何依赖关系，是一个独立的 `zed` 扩展

> [!caution]
> 在 `zed` 中使用时，可能会出现 `Error: resolving completion` 的错误，这并非扩展的问题，功能也没有任何影响，本人也在积极的去与 `thriftls` 作者沟通，了解此类问题

> [!caution]
> 你不能同时使用 `zed-thrift` 和 `zed-thrift-next` ，安装 `zed-thrift-next` 前请先卸载 `zed-thrift` 扩展

## 功能

- **自动补全**
- **代码高亮**
- **语法检查**
- **定义跳转**
- **代码格式化**


## 为什么选择 Zed-Thrift-Next？

原始的 `zed-thrift` 扩展存在以下问题：

1. 不会自动下载 `thriftls` 服务，并且没有文档说明如何找到正确的 `lsp` 服务器
2. 扩展使用了错误的参数，导致即使尝试两种不同的 `thrift` 服务，`lsp` 服务器也无法启动
3. 只能进行高亮显示，但错误的数据导致频繁出现错误

`zed-thrift-next` 通过执行以下操作解决了这些问题：

1. 检查用户是否已经配置了现有的 `thrift` 服务器；如果是，则使用用户配置，不进行检查、下载或更新
2. 检查用户是否已在 `$PATH` 中安装并配置了 `thrift` 服务器；如果是，则跳过检查、下载和更新
3. 如果用户未安装 `thrift` 服务器，则从 `github` 自动下载
4. 如果存在旧版本，则自动更新并删除旧版本以保持最新状态

## 安装

### 从 `zed` 扩展中安装

打开 `zed` 编辑器，在扩展中找到 `Thrift Next` 进行安装

### 从源代码安装

> 使用源代码安装需要安装好 `rust` ，并且 `rust` 是通过 `rustup` 进行安装或者更新的，这是 `zed` 的要求

- 使用 `git clone` 下载当前项目
- 打开 `zed` 编辑器，运行命令 `zed: install dev extension`
- 在打开的窗口中选择当前克隆的项目文件夹


## 注意事项

1. `zed-thrift-next` 扩展会从 `github` 下载二进制文件，您需要确保网络连接正常
2. 安装扩展会自动下载 `thriftls` 二进制文件，如果失败，请尝试卸载后重新安装
3. 如果由于网络或其他问题无法继续安装，您可以手动下载二进制文件并将其放置在 `$PATH` 中，您可以 [在此处](https://github.com/joyme123/thrift-ls/releases/tag/v0.2.9) 找到并下载该二进制文件
4. 你无须担心二进制名称问题，你可以使用 `thriftls` 或者直接将下载的二进制放入 `$PATH` ，扩展会进行兼容处理
