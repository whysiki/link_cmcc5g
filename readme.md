# 连接我校校园移动网命令行工具

## 构建

使用以下命令进行发布模式构建：

```shell
cargo build --release
```

可执行文件位于：`./target/release/link_cmcc5g.exe`。

## 安装

使用以下命令安装工具：

```shell
cargo install --path .
```

## 使用

运行以下命令连接校园网络，系统会提示输入用户名和密码：

```shell
link_cmcc5g
```

## 清除配置

使用以下命令清除现有配置，系统会提示确认：

```shell
link_cmcc5g clear
```

配置文件位置：  
`C:\Users\Administrator\.cargo\bin\config.json`

## 设置开机启动 (Windows)

**先执行一次`link_cmcc5g`配置好用户名和密码，然后执行以下步骤**:

1. 获取可执行文件路径（CMD）：

```shell
where link_cmcc5g
```

1. 获取可执行文件路径（PowerShell）：

```shell
Get-Command link_cmcc5g
```

1. 在注册表中设置开机启动：

```shell
reg add HKCU\Software\Microsoft\Windows\CurrentVersion\Run /v link_cmcc5g /t REG_SZ /d "C:\Users\Administrator\.cargo\bin\link_cmcc5g.exe"
```
