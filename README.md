## kv-server

>  键值数据库是一种非关系数据库，它使用简单的键值方法来存储数据。键值数据库将数据存储为键值对集合，其中键作为唯一标识符。键和值都可以是从简单对象到复杂复合对象的任何内容。键值数据库是高度可分区的，并且允许以其他类型的数据库无法实现的规模进行水平扩展。

核心需求：

1. 根据不同的命令进行数据存储、读取、监听等操作；
2. 客户端要能通过网络访问 KV Server，包括发送包含命令的请求，得到结果；
3. 数据要能根据需要，存储在内存中或者持久化到磁盘上。

### 架构设计

梳理 kv server 的业务主流程：

![Main Process](https://static001.geekbang.org/resource/image/67/dc/67894066ecedd65897d5644f949b8cdc.jpg?wh=2617x1584)

关键问题：

* 客户端（Client）与服务端（Server）之前通信协议的确定，是否支持多种协议（高性能场景，优先考虑 TCP，但网络层需要灵活）；
* 对应用层传输数据序列化 / 反序列化的考量；Protobuf？（protobuf 解析快、省带宽，需要额外的编译工具）
* 存储引擎的功能支持考虑；
* 对服务端支持的命令逻辑的细化和定义；（参考 Redis 命令集，作 Trait 抽象）
* 事件通知的 hook 处理；
* 系统配置中心的逻辑考虑；

核心的接口设计：客户端与服务器端的接口（协议）、服务端与命令处理流程的接口、存储引擎的接口。

### 开发任务

系统架构如图：

![](https://static001.geekbang.org/resource/image/82/2c/82da823b4eb16935fdeyy727e3b3262c.jpg?wh=1920x1145)

- [x] 使用 protobuf 定义支持的客户端命令，并使用 [prost]() 编译成 Rust 代码;
- [x] 封装 proto 模块，增加一些基本的类型转换，**src/pb/mode.rs**;
- [x] 设计如何处理请求命令，返回响应的接口抽象：CommandService trait;
- [x] 设计 Storage trait 的抽象设计, 实现 Memory Table;
- [x] CommandService 的具体实现；
- [x] 完成所有命令的工作量；
- [x] Iter trait 的高级抽象和 Service Execute 事件通知处理；
- [x] 增加网络处理层，独立处理封包和解包的逻辑;
- [x] 网络层的安全问题处理；
- [x] 网络层的异步处理优化重构 Stream;
- [ ] 架构重构，利用 yamux 支持 PUB/SUB 模式;


