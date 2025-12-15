🚧 **Work in Progress** 🚧

In Greek Mythology, __Apheleia__ was the spirit and personification of simplicity, "the good old days".

# What is Apheleia
Apheleia is a rust toolkit that aims to make the process of building reactive and beautiful looking TUIs as easy as possible.

# Roadmap
- [X] Basic terminal rendering using crossterm
- [X] Support for styling foreground, background and support for various terminal styles (like bold, italic, underlined, etc)
- [X] A simple node based management system
    - [X] Root Node
    - [X] Node
- [X] Parent child relations
    - [X] Link parent child nodes
    - [X] Relative positioning of children with respect to parent
- [ ] Event Loop
    - [ ] Nodes should register the events to listen to
- [X] Update Loop
    - [X] Nodes should register to update if necessary
- [ ] Basic Widgets
    - [ ] Labels
        - [X] Scrolling Text if text exceeds the width of node
        - [X] Ellipses at end...
        - [ ] Multi Line rendering
    - [ ] Blocks
        - [X] Basic implementation
        - [ ] Border styling
