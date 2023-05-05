use std::collections::VecDeque;

fn main() {
    let mut deq = VecDeque::from([-1, 0, 1]);
    deq.push_front(-2);
    //println!("{:?}", deq);
    stack_example();
}

fn stack_example () {
    //TODO: can we have it have pauses so people can see what it looks like?
    println!("Stack example with VecDeque:");

    let mut stack = VecDeque::new();
    println!("Empty Stack: {:?}", stack);

    println!("push some elements onto the stack:");
    stack.push_front(1);
    stack.push_front(2);
    stack.push_front(3);
    stack.push_front(4);

    println!("current stack: {:?}", stack);

    println!("pop an elements off the stack stack.pop_front()");
    stack.pop_front();
    println!("current stack: {:?}", stack);

    println!("pop an elements off the stack stack.pop_front()");
    stack.pop_front();
    println!("current stack: {:?}", stack);

    println!("peek the top of the stack stack.get(0)");
    println!("{:?}", stack.get(0));
    println!("current stack: {:?}", stack);

    println!("clear the stack with: deque.drain(..);");
    stack.drain(..);
    println!("current stack: {:?}", stack);
}
