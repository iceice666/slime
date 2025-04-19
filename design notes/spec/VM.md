# Virtual Machine Design Notes

## Garbage Collection

For data on the stack, use a reference counting scheme with weak references to avoid circular dependencies.
For data on the heap, use the 3-color mark-and-sweep garbage collector (reference: golang)

### Reference for 3-color mark-and-sweep GC
- [Basics of Golang GC Explained: Tri-color Mark and Sweep and Stop the World](https://blog.stackademic.com/basics-of-golang-gc-explained-tri-color-mark-and-sweep-and-stop-the-world-cc832f99164c)
- [Tricolor Abstraction to build concurrent Garbage Collectors](https://www.youtube.com/watch?v=lhrRwjVPXPo)
- [GC Theory – mark&sweep algorithm](https://www.baremetaldev.com/2021/11/17/gc-theory-marksweep-algorithm/)

## Runtime

- Lightweight runtime with a focus on performance and low overhead (reference: LuaVM && LuaJIT)
- Standard library should focus on immutable data structures and functional operations
- FFI (Foreign Function Interface) should provide seamless interoperability with existing languages/libraries
- Concurrency model should consider lightweight threads or actors with message passing

