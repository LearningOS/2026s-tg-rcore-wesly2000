## Project Overview

本项目是在rCore OS (用于教育目的、运行于RISC-V架构上OS内核) 上进行开发/拓展，以完成目标。rCore运行在QEMU上，宿主机使用Ubuntu 24.04系统。

项目依赖于：
+ ../tg-rcore-tutorial-sbi：SBI 调用封装库，支持 nobios 模式，内建 M-mode 启动代码

## Workflow: Course Assignment Solver

### Phase 1: 项目信息蒸馏（仅执行一次）
在阅读其它文件之前，首先阅读README.md：

1. 运行`pwd`查看自己所在路径位置；
2. 阅读[一、环境准备]之前的章节获得本项目的Overview，并根据文档生成TASK.md用于检查任务完成情况；
3. 阅读[一、环境准备]检查环境是否已经安装好（如Rust, QEMU）；
4. 阅读[二、编译与运行]试运行系统，观察输出是否与README.md一致;
5. 后续章节不需要再阅读；

### Phase 2: Implementation Loop (Per-Task)
对于`TASK.md`中的每个Task:
1. **状态更新**: 在`TASK.md`中将该任务标记为"进行中"；
2. **实现**: 根据`TASK.md`中关于任务的描述完成任务
3. **测试-修复循环**: 
   - 运行`cargo run`，查看输出
   - 如果失败，从stderr读出失败日志，修复代码，重新运行直到100%pass
4. **文档记录**: 当所有测试都能够pass后记录自己的完成历程, 直观地解释完成的逻辑并且将"Lessons Learned"写到`WRITEUP.md`中（如果没有`WRITEUP.md`就新建一个）.
5. **Mark Done**: 将该任务在`TASK.md`标记为"Complete"