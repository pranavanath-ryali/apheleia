🚧 **Work in Progress** 🚧

In Greek Mythology, __Apheleia__ was the spirit and personification of simplicity, "the good old days".

# What is Apheleia
Apheleia is an experimental, retained-mode, ECS framework to build Terminal User Interface (TUI) built in rust. It uses unique, isolated buffers for each node that scales with changes in content rather than a dense 2D grid of cells.

# Examples

A counter program to test the underlying ECS, resource mutations, dynamic math expressions, and event based dirty rendering:
```bash
cargo run --bin counter
```

# Highlights

* **Isolated Per-Node Buffers: **

# Roadmap
- [ ] Refactor the entire APP crate
- [ ] Provide access to creation and deletion dynamically during runtime
- [ ] Find an alternative method to achieve runtime_expressions
- [ ] MORE Widgets
- [ ] Layout Crate
- [ ] Full Unicode Support
- [ ] Make shit finally good
