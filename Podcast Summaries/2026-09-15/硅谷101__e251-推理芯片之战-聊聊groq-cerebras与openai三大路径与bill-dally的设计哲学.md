---
podcast: "硅谷101"
episode: "E251｜推理芯片之战：聊聊Groq、Cerebras与OpenAI三大路径与Bill Dally的设计哲学"
published: 2026-09-15
duration: "1:31:32"
audio_url: "https://aphid.fireside.fm/d/1437767933/f0f20376-8faf-4940-b920-84af6c734e2d/786d63c4-d46c-4ff3-80f8-fc895b2a57f0.mp3"
episode_url: "https://sv101.fireside.fm/264"
transcript_source: show_notes
generated_at: 2026-09-17T22:15:00Z
model: "deepseek-v4-flash"
guid: "786d63c4-d46c-4ff3-80f8-fc895b2a57f0"
---

# E251｜推理芯片之战：聊聊Groq、Cerebras与OpenAI三大路径与Bill Dally的设计哲学 — 硅谷101

## TL;DR
本期邀请两位AI芯片创业者（其中一位Mark是英伟达首席科学家Bill Dally的学生），拆解推理芯片三条技术路径：Groq的确定性SRAM+静态编译调度、Cerebras的整片晶圆路线，以及OpenAI与博通合作的首款自研推理芯片Jalapeño（选择HBM4，9个月流片）。核心结论：训练看算力，推理看带宽；进入推理时代，芯片的最重要指标是每千瓦时能产出多少token，而CUDA在推理时代“一定会被绕过”。

## Key points
- 推理分Prefill（算力密集）与Decode（带宽密集）两阶段：Decode每生成一个token都要把整个模型权重从存储读到计算单元，带宽成为瓶颈。
- SRAM/DRAM/HBM对比：SRAM延迟0.3-3ns但容量极小；DRAM延迟50-100ns；HBM是3D堆叠DRAM，带宽高但成本高且极度缺货。
- Groq走确定性静态编译调度，缺点是动态性不足；Cerebras用整片晶圆缩短物理距离，最大挑战在良率与成本。
- Cerebras在技术选型时点（Transformer尚未出现）缺乏足够信息去针对未来workload优化。
- 英伟达以约200亿美元acqui-hire Groq，体现推理场景对nvidia的战略价值；Cerebras今年6月IPO市值670亿美元。
- OpenAI Jalapeño与博通合作，9个月从设计到流片；选择HBM4的原因是美国“不缺钱，但缺电”；将异构收进单颗芯片内部。
- 多家美国推理芯片初创公司（d-Matrix、MatX）与TPU、Trainium都在向SRAM收敛，每代容量逐步提升。
- Bill Dally的设计哲学：一切围绕局部性（locality）展开；跳出local minimum必须想清楚牺牲什么。
- 对模型迭代要抓住“不变量”；芯片架构需在算力利用率跑不满的现实下做权衡。

## Implications
推理专用芯片的竞争已从单点性能转向“速度、性价比、耗电量”的三元平衡，并且电力可能比钱更稀缺（OpenAI选HBM4 而非纯SRAM 就是先看电网边界）。对云厂商和模型厂商而言，采购决策应从“单卡推理快不快”转向“整体系统异构与每千瓦时token产出”，并在GPU、SRAM推理芯片与专用加速器之间做系统级组合。

## Notable quotes
- “推理时代，Cuda一定会被绕过”（嘉宾论断）
- “速度越快，AI越聪明”（嘉宾关于推理延迟与智能味的关系）
- “一切围绕局部性展开”（Bill Dally 芯片设计哲学）

## People mentioned
- 泓君 — 矽谷101创始人与主持人
- Mark — AI芯片创业者，Bill Dally 学生
- 徐子扬 — AI芯片创业者
- Bill Dally — 英伟达首席科学家
- 黄仁勋 — 英伟达创始人

## Topics
AI芯片, 推理芯片, Groq, Cerebras, OpenAI, Jalapeño, SRAM, HBM, Bill Dally, 算力经济
