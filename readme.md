## 用来连接我校移动校园网的命令行工具

- release , ouput in `./target/release/link_cmcc5g.exe`
```shell
cargo build --release
```
- install, then use `link_cmcc5g` in cmd
```shell
cargo install --path .
```

- use, will prompt for username and password
```shell
link_cmcc5g
```

- clear config, enter y/n to clear config
```shell
link_cmcc5g clear
```

- config file is in `C:\Users\Administrator\.cargo\bin\config.json`


- set start with windows
get exe path in cmd
```shell
where link_cmcc5g
```
get exe path in powershell
```shell
Get-Command link_cmcc5g
```
- set start with windows in registry
```shell
reg add HKCU\Software\Microsoft\Windows\CurrentVersion\Run /v link_cmcc5g /t REG_SZ /d "C:\Users\Administrator\.cargo\bin\link_cmcc5g.exe"
```

