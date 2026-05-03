# Chapter 3 实验报告

## 实验概述

本实验在 rCore OS Chapter 3（多道程序与分时多任务）的基础上，实现了 `sys_trace` 系统调用（ID 410），用于追踪当前任务的系统调用历史信息。

## 实验目标

实现一个新的系统调用 `sys_trace`，支持三种功能：
1. **读取用户内存**（trace_request=0）：从指定地址读取 1 字节
2. **写入用户内存**（trace_request=1）：向指定地址写入 1 字节
3. **查询系统调用计数**（trace_request=2）：返回指定系统调用的调用次数

## 实现方案

### 1. 扩展 TaskControlBlock 结构

在 `src/task.rs` 中扩展 `TaskControlBlock` 结构，添加系统调用计数器：

```rust
pub struct TaskControlBlock {
    ctx: LocalContext,
    pub finish: bool,
    stack: [usize; 1024],
    // 新增：系统调用计数器数组
    syscall_counts: [usize; 512],
}
```

**设计决策**：
- 使用 `usize` 类型而非 `u16`，提供足够大的计数空间（2^64-1）
- 数组大小为 512，足以覆盖所有可能的系统调用 ID
- 每个任务独立维护自己的系统调用计数

### 2. 实现系统调用计数功能

修改 `TaskControlBlock::handle_syscall()` 方法，在处理系统调用前记录调用次数：

```rust
pub fn handle_syscall(&mut self) -> SchedulingEvent {
    let id: Id = self.ctx.a(7).into();
    
    // 记录系统调用计数（在处理之前，确保 trace 调用本身也被计入）
    let id_num = id.0;
    if id_num < self.syscall_counts.len() {
        self.syscall_counts[id_num] += 1;
    }
    
    // ... 处理系统调用 ...
}
```

添加查询方法：

```rust
pub fn get_syscall_count(&self, syscall_id: usize) -> usize {
    if syscall_id < self.syscall_counts.len() {
        self.syscall_counts[syscall_id]
    } else {
        0
    }
}
```

### 3. 解决栈溢出问题

**问题发现**：
- 原始 TCB 大小约 8.5 KB
- 添加 `syscall_counts: [usize; 512]` 后，TCB 大小增加到约 12.5 KB
- 32 个 TCB 的数组总大小为 392.5 KB，超过内核栈大小（272 KB）

**解决方案**：
将 TCB 数组从栈上移到 `.bss` 段（静态存储区）：

```rust
// 在 src/main.rs 中声明全局静态变量
static mut TCBS: [TaskControlBlock; APP_CAPACITY] = [TaskControlBlock::ZERO; APP_CAPACITY];
```

修改 `rust_main()` 函数使用全局 TCB 数组：

```rust
// 初始化 TCB 数组
for (i, app) in tg_linker::AppMeta::locate().iter().enumerate() {
    let entry = app.as_ptr() as usize;
    log::info!("load app{i} to {entry:#x}");
    unsafe { TCBS[i].init(entry) };
    index_mod += 1;
}

// 主循环中使用全局数组
while remain > 0 {
    let tcb = unsafe { &mut TCBS[i] };
    // ...
}
```

### 4. 实现 Trace::trace() 方法

在 `src/main.rs` 的 `impls` 模块中实现 `Trace::trace()` 方法：

```rust
// 添加全局变量用于访问当前 TCB
static mut CURRENT_TCB: Option<*mut TaskControlBlock> = None;

impl Trace for SyscallContext {
    fn trace(
        &self,
        _caller: Caller,
        trace_request: usize,
        id: usize,
        data: usize,
    ) -> isize {
        unsafe {
            let tcb = match CURRENT_TCB {
                Some(ptr) => &*ptr,
                None => return -1,
            };

            match trace_request {
                // 读取用户内存
                0 => {
                    let byte = *(id as *const u8);
                    byte as isize
                }
                // 写入用户内存
                1 => {
                    *(id as *mut u8) = data as u8;
                    0
                }
                // 查询系统调用计数
                2 => tcb.get_syscall_count(id) as isize,
                // 无效的 trace_request
                _ => -1,
            }
        }
    }
}
```

在主循环中设置当前 TCB 指针：

```rust
while remain > 0 {
    let tcb = unsafe { &mut TCBS[i] };
    if !tcb.finish {
        // 设置当前 TCB 指针（用于 sys_trace 访问）
        unsafe { CURRENT_TCB = Some(tcb as *mut _) };
        // ...
    }
}
```

## 技术难点与解决方案

### 难点 1：栈溢出问题

**问题**：TCB 数组太大，导致栈溢出，内核无法启动。

**解决过程**：
1. 最初尝试使用 `u16` 类型减小数组大小（512 * 2 = 1024 字节）
2. 发现即使使用 `u16`，TCB 数组仍然太大（296.5 KB > 272 KB）
3. 最终方案：将 TCB 数组移到 `.bss` 段，完全避免栈空间问题

**经验教训**：
- 大型数据结构应该放在静态存储区而非栈上
- 内核栈空间有限，需要谨慎使用

### 难点 2：整数溢出问题

**问题**：使用 `u16` 类型时，某些测试中系统调用次数超过 65535，导致溢出。

**解决方案对比**：

| 方案 | 优点 | 缺点 | 结果 |
|------|------|------|------|
| u16 + 饱和加法 | 节省内存（296.5 KB） | 需要 `saturating_add()`，计数上限 65535 | ✅ 测试通过 |
| usize | 代码简洁，不会溢出，计数上限 2^64-1 | 占用更多内存（392.5 KB） | ✅ 测试通过 |

**最终选择**：使用 `usize` 类型
- 代码更简洁（直接使用 `+=`）
- 不会溢出
- 内存开销可接受（TCB 数组在 `.bss` 段）
- 性能更好（直接加法比饱和加法快）

### 难点 3：在系统调用处理中访问 TCB

**问题**：`Trace::trace()` 方法在 `SyscallContext` 中实现，无法直接访问当前的 TCB。

**解决方案**：
- 添加全局变量 `CURRENT_TCB` 存储当前 TCB 的指针
- 在主循环中，执行任务前设置 `CURRENT_TCB`
- 在 `Trace::trace()` 中通过 `CURRENT_TCB` 访问当前 TCB

**安全性考虑**：
- 使用 `unsafe` 代码访问全局可变变量
- 在单核、单线程环境下是安全的
- 在多核环境下需要额外的同步机制

## 测试结果

### 基础测试（./test.sh base）
```
Test PASSED: 4/4
✓ ch3 基础测试通过
```

测试内容：
- 多任务并发执行
- 时间片轮转调度
- 协作式调度（yield）
- 系统调用（write、exit、clock_gettime）

### 练习测试（./test.sh exercise）
```
Test PASSED: 7/7
✓ ch3 练习测试通过
```

测试内容：
- sys_trace 读取用户内存功能
- sys_trace 写入用户内存功能
- sys_trace 查询系统调用计数功能
- 系统调用计数的准确性
- 边界条件测试

### 完整测试（./test.sh）
```
基础测试: 4/4 通过
练习测试: 7/7 通过
总计: 11/11 通过
```

## 实现亮点

1. **内存优化**：将 TCB 数组移到 `.bss` 段，避免栈溢出
2. **类型选择**：使用 `usize` 提供足够大的计数空间，避免溢出
3. **计数准确性**：在处理系统调用前记录计数，确保 trace 调用本身也被计入
4. **代码简洁**：使用直接加法而非饱和加法，代码更清晰

## Lessons Learned

### 1. 内存管理
- **栈空间有限**：内核栈通常只有几百 KB，大型数据结构应该放在静态存储区
- **静态存储区**：使用 `static mut` 声明全局变量，数据存储在 `.bss` 段
- **内存布局**：理解不同内存区域的特点和限制

### 2. 类型选择
- **权衡取舍**：在内存占用和功能完整性之间做出选择
- **溢出处理**：考虑整数溢出的可能性，选择合适的类型或使用饱和运算
- **性能考虑**：直接运算通常比饱和运算更快

### 3. 系统调用实现
- **计数时机**：在处理系统调用前记录计数，确保统计准确
- **上下文访问**：使用全局变量在不同模块间传递上下文信息
- **安全性**：在单核环境下使用 `unsafe` 代码是可接受的

### 4. 调试技巧
- **日志级别**：使用 `LOG=TRACE` 获取详细的调试信息
- **测试驱动**：先运行测试发现问题，再针对性地修复
- **逐步验证**：每次修改后立即测试，快速定位问题

### 5. 与 AI 协作
- **清晰沟通**：明确描述问题和期望的解决方案
- **验证建议**：对 AI 的建议进行验证和测试
- **迭代改进**：根据测试结果不断调整实现方案
- **记录过程**：保存交互历史，便于回顾和总结

## 总结

本实验成功实现了 `sys_trace` 系统调用，支持读写用户内存和查询系统调用计数三种功能。通过将 TCB 数组移到 `.bss` 段解决了栈溢出问题，通过使用 `usize` 类型避免了整数溢出问题。所有测试 100% 通过，实现了预期的功能目标。

实验过程中深入理解了：
- 操作系统内核的内存管理
- 系统调用的实现机制
- 任务控制块的设计与扩展
- 内核编程中的安全性考虑

这些知识和经验为后续章节的学习打下了坚实的基础。
