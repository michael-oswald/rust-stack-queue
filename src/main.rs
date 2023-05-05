use std::collections::VecDeque;
use std::{thread, time::Duration};


fn main() {
    let mut deq = VecDeque::from([-1, 0, 1]);
    deq.push_front(-2);
    //println!("{:?}", deq);
    stack_example();
}

fn stack_example () {
    println!("Stack example with VecDeque:");
    println!("What is VecDeque? It's a double ended queue (so works as a stack or queue) \
    You can push and pop from front or back of the data structure.\n\n");

    println!("Declare Empty mutable stack: let mut stack = VecDeque::new();");
    let mut stack = VecDeque::new();
    println!("Empty Stack: {:?} \n", stack);
    sleep();

    println!("push some elements onto the stack:");

    println!("stack.push_front(1);");
    stack.push_front(1);

    sleep();
    println!("current stack: {:?} \n", stack);

    println!("stack.push_front(2);");
    stack.push_front(2);

    sleep();
    println!("current stack: {:?} \n", stack);

    println!("stack.push_front(3);");
    stack.push_front(3);

    sleep();
    println!("current stack: {:?} \n", stack);

    println!("stack.push_front(4);");
    stack.push_front(4);

    sleep();
    println!("current stack: {:?} \n", stack);

    println!("pop an elements off the stack stack.pop_front() returns -> {:?}", stack.pop_front());
    sleep();
    println!("current stack: {:?}\n", stack);

    println!("pop an elements off the stack stack.pop_front() returns -> {:?}", stack.pop_front());
    sleep();
    println!("current stack: {:?}\n", stack);

    println!("peek at the top of the stack stack.get(0) returns -> {:?}", stack.get(0));
    sleep();
    println!("current stack (has no changes): {:?}\n", stack);

    println!("clear the stack with: deque.clear();");
    stack.clear();
    sleep();
    println!("current stack: {:?}\n", stack);

    println!("check if stack is empty with stack.is_empty(); -> {:?}", stack.is_empty());

}

fn sleep() { // pause so console output can be followed more easily.
    thread::sleep(Duration::from_millis(3000));
}