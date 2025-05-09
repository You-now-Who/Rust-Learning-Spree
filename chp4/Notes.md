## Stack vs Heap

- Stack follows a last in, first out
- Data on stack should be of a known, fixed size
- When requesting memory in heap, it 'allocates' space in the heap, gets the pointer and stores that on the stack
- Stack is faster than heap
- Ownership helps explain why this works the way it does

## Ownership rules

- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

(What *is* a value?)

- Data types like uint8 etc are one with known sizes. That means they can be stored on the stack

- String Literals are inherently different from String type 