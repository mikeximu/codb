# ARCHITECTURE.md（大纲）

> 本文档描述本项目的整体架构设计、核心抽象及演进原则。
> 目标是在保证 API 稳定性的前提下，支持数据库能力的长期演进。

---

## 1. 项目目标（Project Goals）

### 1.1 总体目标

* 提供一个 **商业级、可嵌入的 Key-Value 数据库内核**
* 核心模块可复用，用于：

    * 关系数据库（SQL）
    * 图数据库
    * 向量 / AI 数据库
* 支持长期演进而不破坏用户代码

### 1.2 非目标（Non-Goals）

* v0.x 阶段不追求：

    * 完整 SQL 支持
    * 分布式一致性
    * 强实时分析能力

---

## 2. 架构总览（High-Level Architecture）

```
+---------------------------+
|        User API           |
|        (DB Trait)         |
+-------------+-------------+
              |
+-------------v-------------+
|        DB Core            |
|  (Lifecycle / Stats)     |
+-------------+-------------+
              |
+-------------v-------------+
|        Engine Layer       |
|   (Pluggable Storage)    |
+-------------+-------------+
              |
+-------------v-------------+
|    Physical Storage       |
| (Memory / Disk / Remote) |
+---------------------------+
```

---

## 3. 核心设计原则（Core Design Principles）

### 3.1 API 稳定性优先

* 一旦发布的公共 API 视为长期承诺
* 新功能只能：

    * 通过新增接口
    * 通过 Options 扩展
* 禁止破坏性修改已发布 trait

### 3.2 组合优于继承

* DB 通过组合 Engine 实现功能
* 不使用深层继承结构

### 3.3 明确抽象边界

* DB Core **不关心** 数据存储细节
* Engine **不关心** 生命周期 / 管理逻辑

---

## 4. 模块划分（Module Layout）

```
src/
├── core/        # 稳定的数据库核心 API
├── engine/      # 存储引擎抽象及实现
├── runtime/     # 生命周期 / 状态管理
├── db_impl.rs   # DB 默认实现
```

---

## 5. Core 模块（Database Core）

### 5.1 DB Trait

* 定义对外可见的数据库操作接口
* 作为所有数据库形态（KV / SQL / Graph）的基础能力

### 5.2 Options（ReadOptions / WriteOptions）

* v0.1 中语义为空
* 用于承载未来：

    * 事务
    * 快照
    * 一致性级别
* 所有字段必须有默认值

### 5.3 Stats

* 提供数据库运行时统计信息
* 允许近似值
* 不保证强一致

---

## 6. Engine 层（Storage Engine）

### 6.1 Engine Trait

* 定义最小存储能力集：

    * Get / Put / Delete
    * Key Count / Size
* 不暴露事务、日志等高级能力

### 6.2 Engine 实现分类

```
engine/
├── mem/    # 内存引擎（v0.1）
├── lsm/    # LSM Tree（未来）
├── wal/    # WAL 装饰器
├── remote/ # 分布式 / 远程引擎
```

### 6.3 装饰器模式（Decorator）

* WAL、统计、限流等通过 Engine Decorator 实现
* 不侵入 Core 或原始 Engine

---

## 7. 生命周期与状态管理（Runtime）

### 7.1 数据库状态

* Open
* Closing
* Closed

### 7.2 行为约束

* Closed 状态下禁止数据操作
* Ping 用于健康检查

---

## 8. 错误模型（Error Model）

* 使用统一错误类型
* 错误语义稳定，不随实现变化
* 错误信息可扩展，但错误种类不可随意变更

---

## 9. 并发与线程安全（Concurrency）

* 所有公开 DB 对象必须是 `Send + Sync`
* Engine 自行负责内部并发控制
* DB Core 不假设单线程或多线程使用场景

---

## 10. 版本演进策略（Versioning Strategy）

### 10.1 语义化版本规则

| 版本    | 破坏性变更 |
| ----- | ----- |
| 0.1.x | 不允许   |
| 0.2.x | 不允许   |
| 1.0   | 最后一次  |
| 1.x   | 不允许   |

### 10.2 API 冻结策略

* DB Trait 在 1.0 前冻结
* Options 永远允许扩展
* Engine Trait 尽量保持最小

---

## 11. 向未来演进（Future Evolution）

### 11.1 事务与 MVCC

* 通过新增 Trait 扩展
* 不修改现有 DB 接口

### 11.2 SQL / 关系数据库

* SQL 层作为独立模块
* DB Core 作为执行引擎

### 11.3 AI 自优化

* 基于 Stats 收集运行数据
* 引入策略模块自动调整配置

---

## 12. 贡献与约定（Contribution Guidelines）

* 新功能必须先更新 ARCHITECTURE.md
* 禁止直接侵入 Core API
* 所有公共接口必须有文档说明

---

## 13. 架构稳定性承诺（Stability Guarantees）

* 所有 `core` 模块对用户稳定
* 内部模块可重构但不得影响公共 API
* 示例代码长期可用

