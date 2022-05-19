# irma4

`irma4` is a 2D simulator of virtual particles, which was created to produce `Artificial Life` ([ALife](https://en.wikipedia.org/wiki/Artificial_life)) forms. It's a field of study wherein researchers examine systems related to natural life, its processes, and its evolution, through the use of simulations with computer models, robotics, and biochemistry. Due to the fact, that our universe is very complex and requires an enormous amount of computational resources, the creation of a more or less realistic virtual world (and later, virtual life) is an almost impossible task today. So we as scientists and developers are trying to find the most similar and also simple model, which is computable for modern PCs. irma4 is a mix of [Artificial Chemistry](https://en.m.wikipedia.org/wiki/Artificial_chemistry), [two-dimensional](https://esolangs.org/wiki/Category:Two-dimensional_languages) programming language and simple [particle physics](https://en.m.wikipedia.org/wiki/Particle_physics). It's an experiment with *unpredictable* results. Also, there is an idea of [Open-Ended Evolution](https://royalsocietypublishing.org/doi/10.1098/rsif.2018.0395#:~:text=10.1098%2Frsif.2018.0395-,Abstract,characterize%20evolution%20on%20multiple%20scales), which refers to the unbounded increase in complexity that seems to characterize evolution on multiple scales. The system starts from very simple elements and their interactions and increases its complexity almost infinitely. This is actually how life variety appears. We hope, that such a process will partially appear in our system as well.

This is actually fourth ([first](https://github.com/tmptrash/jevo), [second](https://github.com/tmptrash/construct), [third](https://github.com/tmptrash/irma)) attempt to create such simulator, so we have some experience in the area :) The general idea is to create simple artificial world model with some restrictions: 2D instead of 3D world, simplified physics, restricted world size, limited computation power and so on. There are only atoms in the world. They may join together to create [molecules](#Molecules) and later - simple organisms. There are few atom types and interactions between them, which give all the variety of the forms. To run such interactions (we also call it "run atoms") we use special [Virtual Machines](https://github.com/tmptrash/irma4/blob/feature/1-25.09.21-skeleton/src/vm/mod.rs).

From the technical perspective, irma4 is a stand alone [Rust](https://www.rust-lang.org/) console application (it's also possible to use other languages (like JavaScript) for tools like organisms editor). It uses [microkernel](https://en.wikipedia.org/wiki/Microkernel) architecture with a possibility to extend functionality with plugins ([dynamic libraries](https://en.wikipedia.org/wiki/Dynamic-link_library)). Planned plugins are: visualization, user terminal, distributed system and statistics. We also have a plan to implement distributed calculations by adding separate processes (separate nodes of the network) to compute more world areas.

There are few goals we are looking for:

- Create minimal physics for virtual life
- Obtain life like behavior of digital creatures
- Assemble LUCA - Last Universal Common Ancestor (first organism or cell)
- Obtain increasing complexity of organisms
- Just run and check what will be

For more details look at [this](https://github.com/tmptrash/irma4/wiki/Project-overview) description and [this](https://docs.google.com/document/d/1I4wnmFnxpCtH4gmAgwguqrFmK4xMWq0tq7pvmjfqguU/edit#) developer document.

### How to run

```
cargo run
```

### How to test

```
cargo test
```

### How to generate documentation

```
cargo doc
```
