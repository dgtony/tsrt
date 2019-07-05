# tsrt

Simple library for topological order sorting of arbitrary directed graphs.

### Algorithms
Implemented sorting algorithms:

* Kahn's algorithm
* Simple recursive DFS

### Utility
Directory `bin` contains simple CLI-application for topological sorting, similar
to standard Unix utility `tsort`. Utility reads from `stdin` a number of coma-separated
sequences, where first element represents parent vertex and following are its descendants.
With given relations it constructs graph and outputs its topological order,
if latter could be obtained: i.e. graph is consistent and does not contain cycles.

```bash
$ tsrt a,b,c b,d c,e,f e,g,h

Topological order found:
a -> b -> c -> d -> e -> f -> h -> g
```
