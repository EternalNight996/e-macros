<img src="../public/ico/white_64x64.ico" alt="e-macros"/>

### 📄 [中文](README.zh.md)  | 📄  [English](../README.md)

# ⚡ 这是什么?
**A Rust macros**

### 支持 功能
<table style="background:#000">
  <tr>
    <th><h3 style="color:#fff">APP</h3></th>
    <th><h3 style="color:#fff">Windows 10</h3></th>
    <th><h3 style="color:#fff">Unix</h3></th>
    <th><h3 style="color:#fff">Macos</h3></th>
  </tr>
  <tr>
    <td>Json</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
  </tr>
  <tr>
    <td>C</td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
    <td><h4 style="color:green">√</h4></td>
  </tr>
  <tr>
    <td>_</td>
    <td><h4 style="color:red">×</h4></td>
    <td><h4 style="color:red">×</h4></td>
    <td><h4 style="color:red">×</h4></td>
  </tr>
</table>

# ✨ 分支
```toml
```

# 📖 示例
```toml
[dependencies]
e-macros = "0.1"
```

```rust
#[derive(e_macros::C)]
struct B {
  d: i32,
  f: String,
}
fn test() -> Result<()> {
  // 假设我们有一个T类型的实例
  let value: B = B {
    d: 1,
    f: "test".to_string(),
  };
  let ptr = value.to_c_ptr();
  // 还原*c_void指针为<Box<T>>实例
  if let Some(restored_boxed_value) = unsafe { B::from_c_ptr(ptr) } {
    // 成功还原Box<T>实例
    println!("Restored value: {:?}", *restored_boxed_value);
  } else {
    // 还原过程中出现错误
    println!("Failed to restore value");
  }
  Ok(())
}
```
# 智能写入Json
# Example
```rust
#[derive(serde::Deserialize, Debug, serde::Serialize, Default, e_macros::Json)]
struct B {
  d: i32,
  f: String,
}
fn test() {
  let mut b: B = B::default();
  b.f = "test".to_string();
  b.auto_write_json(Path::new("."), "test.json").unwrap();
  let b = B::auto_read_json(Path::new("test.json")).unwrap();
  println!("B {:?}", b);
}
```
# 智能读取Json
# Example
```rust
#[derive(serde::Deserialize, Debug, serde::Serialize, Default, e_utils::Json)]
struct B {
  d: i32,
  f: String,
}
fn test() {
  let mut b: B = B::default();
  b.f = "test".to_string();
  b.auto_write_json(Path::new("."), "test.json").unwrap();
  let b = B::auto_read_json(Path::new("test.json")).unwrap();
  println!("B {:?}", b);
}
```
# 安全地还原*const c_void指针为Box<Self>
# Example
```rust
#[derive(e_utils::C)]
struct B {
  d: i32,
  f: String,
}
fn test() -> Result<()> {
  // 假设我们有一个T类型的实例
  let value: B = B {
    d: 1,
    f: "test".to_string(),
  };
  let ptr = value.to_c_ptr();
  // 还原*c_void指针为<Box<T>>实例
  if let Some(restored_boxed_value) = unsafe { B::from_c_ptr(ptr) } {
    // 成功还原Box<T>实例
    println!("Restored value: {:?}", *restored_boxed_value);
  } else {
    // 还原过程中出现错误
    println!("Failed to restore value");
  }
  Ok(())
}
```
## `💡!重要：`
#### xxx
<!-- 您必须使用使用MSVC工具链的Rust版本
您必须安装[WinPcap](https://www.winpcap.org/)或[npcap](https://nmap.org/npcap/)（使用[WinPcap](https://www.winpcap.org/) 4.1.3版进行测试）（如果使用[npcap](https://nmap.org/npcap/)，请确保使用“在[WinPcap](https://www.winpcap.org/) API兼容模式下安装[npcap](https://nmap.org/npcap/)”）
你必须把它放在包里。[WinPcap](https://www.winpcap.org/)开发者包中的lib位于该存储库根目录中名为lib的目录中。或者，您可以使用%LIB%/$Env:LIB环境变量中列出的任何位置。对于64位工具链，它位于WpdPack/Lib/x64/Packet中。对于32位工具链，它位于WpdPack/lib/Packet.lib中。
```
# 1.安装npcap服务 https://npcap.com/dist/npcap-1.70.exe
setx LIB E:\libs\LIB
# 下载并解压 https://npcap.com/dist/npcap-sdk-1.13.zip
# 将npcap-sdk-1.13\Lib\x64\Packet.lib放到E:\libs\LIB
``` -->

# 🚀 快速运行
<!-- ```sh
# 主机/端口扫描
cargo run --example host_scan
cargo run --example port_scan
``` -->


# 🦊 已运用项目
<!-- [E-NetScan](https://github.com/EternalNight996/e-netscan.git): 网络扫描项目（同时支持命令行与跨平台图形化界面）正在开发中。。 -->

# 🔭 为什么需要e-utils?
<!-- 起初是想完成一个跨网络扫描项目，帮助自己完成一些工作，参考许多开源项目,但这些项目多少有些缺陷并不满足自己需求，所以有了e-libscanner。
(处理主机和端口扫描，同时支持域名解析、路由跟踪、指纹扫描、服务扫描、异步扫描、可扩展更多)
底层是通过调用[npcap](https://nmap.org/npcap/)与[WinPcap](https://www.winpcap.org/)抓包服务；
服务api为[libpnet](https://github.com/libpnet/libpnet); -->

# 🙋 参考项目与资料
<!-- ✨[RustScan](https://github.com/RustScan/RustScan) :Rust仿nmap扫描库
✨[netscan](https://github.com/shellrow/netscan) :Rust 网络扫描库
✨[libpnet](https://github.com/libpnet/libpnet) 跨平台网络底层库--主要是调用抓包服务([npcap](https://nmap.org/npcap/)与[WinPcap](https://www.winpcap.org/)) -->