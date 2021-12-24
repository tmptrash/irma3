# irma4

`irma4` is a 2D simulator of virtual particles, which was created to produce `Artificial Life` ([ALife](https://en.wikipedia.org/wiki/Artificial_life)) forms. It's a field of study wherein researchers examine systems related to natural life, its processes, and its evolution, through the use of simulations with computer models, robotics, and biochemistry. Due to the fact, that our universe is very complex and requires an enormous amount of computational resources, the creation of a more or less realistic virtual world (and later, virtual life) is an almost impossible task today. So we as scientists and developers are trying to find the most similar and also simple model, which is computable for modern PCs. irma4 is a mix of [Artificial Chemistry](https://en.m.wikipedia.org/wiki/Artificial_chemistry), [two-dimensional](https://esolangs.org/wiki/Category:Two-dimensional_languages) programming language and simple [particle physics](https://en.m.wikipedia.org/wiki/Particle_physics). It's an experiment with *unpredictable* results. Also, there is an idea of [Open-Ended Evolution](https://royalsocietypublishing.org/doi/10.1098/rsif.2018.0395#:~:text=10.1098%2Frsif.2018.0395-,Abstract,characterize%20evolution%20on%20multiple%20scales), which refers to the unbounded increase in complexity that seems to characterize evolution on multiple scales. The system starts from very simple elements and their interactions and increases its complexity almost infinitely. This is actually how life variety appears. We hope, that such a process will partially appear in our system as well.

This is actually fourth ([first](https://github.com/tmptrash/jevo), [second](https://github.com/tmptrash/construct), [third](https://github.com/tmptrash/irma)) attempt to create such simulator, so we have some experience in the area. The general idea is very similar to our real-world, but with many restrictions: there is a 2D world of [atoms](#Atoms) (colored pixels). They may join together to create [molecules](#Molecules) and later - simple organisms. There are few interactions and atoms types, which give all the variety of forms in the world. To run such interactions (we also call them "run atoms") we use special [Virtual Machines](#Atomic-Virtual-Machines).

There are few goals we are looking for:

- Create minimal physics for virtual life
- Obtain life like behavior of digital creatures
- Assemble LUCA - Last Universal Common Ancestor
- Obtain increasing complexity of organisms
- Just run and check what will be

For more details look at [this](https://github.com/tmptrash/irma4/wiki/Project-overview) description.

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
