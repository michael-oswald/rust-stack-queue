# rust-stack-queue
Tutorial on how to use stack and queue in rust

# VecDeque Data Structure
For stack and queue in this example we'll use the VecDeque data structure provided by Rust by using: `std::collections::VecDeque;`

The docs say that it is a `A double-ended queue`, which means it can insert and remove elements from both ends
of the structure. Thus, it can be used as a stack or a queue. Here are the [official docs](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html)

# TLDR stack and queue in rust
**Queue**
```
// Queue - FIFO structure 
//import
use std::collections::VecDeque;

// First define a mutable VecDeque:
let mut queue = VecDeque::new();

// add elements to the queue:
queue.push_back(1);
queue.push_back(2);
queue.push_back(3);
queue.push_back(4);

// remove 2 elements:
queue.pop_back();
queue.pop_back();

// peek:
queue.get(queue.len() - 1);

// clear:
queue.clear();

```

**Stack**

```
// Stack - LIFO structure 
//import
use std::collections::VecDeque;

// First define a mutable VecDeque:
let mut stack = VecDeque::new();

// add elements to the stack:
stack.push_front(1);
stack.push_front(2);
stack.push_front(3);
stack.push_front(4);

// remove 2 elements:
stack.pop_front();
stack.pop_front();

// peek:
stack.get(0);

// clear:
stack.clear();

```

# Run this example:
Prereq: You need Rust installed with cargo cli available
```
#from the root of this project execute:
cargo run 
```

The example in this program in [main.rs](src/main.rs), is very simple with two functions. It will run a queue example 
and a stack example and print out the common methods to the console. 

