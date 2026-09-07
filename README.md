🚧 **Work in Progress** 🚧

In Greek Mythology, __Apheleia__ was the spirit and personification of simplicity, "the good old days".

# What is Apheleia
Apheleia is an experimental, retained-mode, ECS framework to build Terminal User Interface (TUI) built in rust. It uses unique, isolated buffers for each node that scales with changes in content rather than a dense 2D grid of cells.

# Examples

A counter program to test the underlying ECS, resource mutations, dynamic math expressions, and event based dirty rendering:
p.s. doesn't work anymore
```bash
cargo run --bin counter
```

# Roadmap
- [ ] Support for multi-width/single endpoint characters
- [ ] Move alpha straight into style
- [ ] Bring RichString support to new-core
- [ ] Update the entire stack to support the new-core
- [ ] Smarter canvas object for painting.
- [ ] Make shit good
