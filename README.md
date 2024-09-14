
<img src="public/ico/white_64x64.ico" alt="e-macros"/>

### 📄 [中文](docs/README.zh.md)  | 📄  [English](README.md)

# ⚡ what this ?
**A Rust macros**

### Support app
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

# ✨ Features
```toml
```

# 📖 Example
```toml
[dependencies]
e-macros = "0.2"
```


## `💡!important：`
<!-- ####There are 测试 requirements for building on the windows system environment:

You must use the rust version using the MSVC toolchain

You must install [WinPcap]（ https://www.winpcap.org/ ）Or [npcap]（ https://nmap.org/npcap/ ）(using [WinPcap]（ https://www.winpcap.org/ ）Version 4.1.3) (if using [npcap]（ https://nmap.org/npcap/ ）, please make sure to use "in [WinPcap]（ https://www.winpcap.org/ ）Install [npcap] in API compatibility mode（ https://nmap.org/npcap/ )”）

You must put it in your bag. [WinPcap]( https://www.winpcap.org/ ）The Lib in the developer package is located in the directory named Lib in the root directory of the repository. Alternatively, you can use any location listed in the% lib% / $env: lib environment variable. For the 64 bit toolchain, it is located in wpdpack / lib / x64 / packet. For the 32-bit toolchain, it is located in wpdpack / lib / packet.lib.
```
# 1.install npcap server https://npcap.com/dist/npcap-1.70.exe
setx LIB E:\libs\LIB
# download and decompression https://npcap.com/dist/npcap-sdk-1.13.zip
# npcap-sdk-1.13\Lib\x64\Packet.lib put to E:\libs\LIB
``` -->

# 🚀 fast running
<!-- ```sh
# host/port scan
cargo run --example host_scan
cargo run --example port_scan
# async scan
cargo run --example async_host_scan --features="async"
cargo run --example async_port_scan --features="async"
# Fingerprint scan
cargo run --example os --features="os"
# Service Scan
cargo run --example service_detection --features="service"
# DNS Parse
cargo run --example dns
# Trace Route
cargo run --example tracert
``` -->

# 🦊 Applied Projects
<!-- [E-NetScan](https://github.com/EternalNight996/e-netscan.git): The network scanning project (which supports both command line and cross platform graphical interface) is under development.. -->

# 🔭 why need to e-utils?
<!-- At first, I wanted to complete a cross network scanning project to help me complete some work. I referred to many open source projects, but these projects have some defects that do not meet my needs, so I have e-libscanner.
(process host and port scanning, and support domain name resolution, route tracking, fingerprint scanning, service scanning, asynchronous scanning, scalability and more)
The bottom layer is by calling [npcap]（ https://nmap.org/npcap/ ）And [WinPcap]（ https://www.winpcap.org/ ）Packet capture service;
The service API is [libpnet]（ https://github.com/libpnet/libpnet ); -->

# 🙋 Reference items and materials
<!-- ✨ [RustScan] https://github.com/RustScan/RustScan : Rustlike nmapscan
✨ [netscan] https://github.com/shellrow/netscan : Rust Network Scanner
✨ [libpnet](libpnet) https://github.com/libpnet/libpnet ● The background base of the interplatform network - mainly using captivity services ([npcap]) https://nmap.org/npcap/ with [WinPcap]( https://www.winpcap.org/ ) -->