# Chapter 3 任务清单

本文档记录了 Chapter 3 练习任务的完成情况。

## 任务概述

实现 `sys_trace` 系统调用（ID 410），用于追踪当前任务的系统调用历史信息。

## 任务列表

### 1. 理解任务需求
**状态**: ✅ Completed

**描述**: 
- 阅读 README.md 了解项目结构和运行方式
- 阅读 exercise.md 了解 sys_trace 系统调用的三种功能：
  - trace_request=0: 读取用户内存
  - trace_request=1: 写入用户内存
  - trace_request=2: 查询系统调用计数

### 2. 扩展 TaskControlBlock 结构
**状态**: ✅ Completed

**描述**:
- 在 `src/task.rs` 中扩展 `TaskControlBlock` 结构
- 添加 `syscall_counts: [usize; 512]` 数组用于记录系统调用计数
- 更新 `ZERO` 常量和 `init()` 方法以初始化新字段

**关键修改**:
- 添加系统调用计数器数组
- 使用 `usize` 类型提供足够大的计数空间（2^64-1）

### 3. 实现系统调用计数功能
**状态**: ✅ Completed

**描述**:
- 修改 `TaskControlBlock::handle_syscall()` 方法
- 在处理系统调用前记录调用次数
- 添加 `get_syscall_count()` 方法用于查询计数

**关键修改**:
- 在 `handle_syscall()` 开始处增加计数器
- 确保 trace 调用本身也被计入统计

### 4. 解决栈溢出问题
**状态**: ✅ Completed

**描述**:
- 发现 TCB 数组太大（392.5 KB）超过内核栈大小（272 KB）
- 将 TCB 数组从栈上移到 .bss 段（静态存储区）
- 在 `src/main.rs` 中声明全局静态变量 `TCBS`

**关键修改**:
- 添加 `static mut TCBS: [TaskControlBlock; APP_CAPACITY]`
- 修改 `rust_main()` 使用全局 TCB 数组

### 5. 实现 Trace::trace() 方法
**状态**: ✅ Completed

**描述**:
- 在 `src/main.rs` 的 `impls` 模块中实现 `Trace::trace()` 方法
- 添加全局变量 `CURRENT_TCB` 用于在系统调用处理中访问当前 TCB
- 实现三种 trace 操作

**关键修改**:
- trace_request=0: 使用 `unsafe` 读取用户内存
- trace_request=1: 使用 `unsafe` 写入用户内存
- trace_request=2: 调用 `get_syscall_count()` 返回计数

### 6. 运行测试并验证
**状态**: ✅ Completed

**描述**:
- 运行基础测试：`./test.sh base`
- 运行练习测试：`./test.sh exercise`
- 运行完整测试：`./test.sh`

**测试结果**:
- ✅ 基础测试: 4/4 通过
- ✅ 练习测试: 7/7 通过
- ✅ 完整测试: 11/11 通过

## 完成情况总结

所有任务已完成，所有测试通过。

**完成时间**: 2026/05/02

**主要成就**:
- 成功实现了 sys_trace 系统调用的三种功能
- 解决了 TCB 数组栈溢出问题
- 通过将 TCB 数组移到 .bss 段优化了内存布局
- 所有测试 100% 通过
