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
