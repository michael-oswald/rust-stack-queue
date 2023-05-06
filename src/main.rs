use std::collections::VecDeque;

fn main() {
    queue_example();
    println!("\n\n\n\n\n");
    stack_example();
}

//FIFO Queue example
fn queue_example() {
    println!("Queue example with VecDeque:");
    let mut queue = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    queue.push_back(4);

    println!("current queue: {:?} \n", queue);
    println!("Now pop_back() twice:\n");

    queue.pop_back();
    queue.pop_back();

    println!("current queue: {:?} \n", queue);

    println!("queue peek -> queue.get(queue.len()-1): {:?} \n", queue.get(queue.len() - 1));

    println!("queue.clear() to remove all elements from the queue");
    queue.clear();

    println!("queue.isEmpty(): {:?} \n", queue.is_empty());

}

//LIFO Stack example
fn stack_example() {
    println!("Stack example with VecDeque:");

    println!("Declare Empty mutable stack: let mut stack = VecDeque::new();");
    let mut stack = VecDeque::new();
    println!("Empty Stack: {:?} \n", stack);

    println!("push some elements onto the stack:");

    println!("stack.push_front(1);");
    stack.push_front(1);
    
    println!("current stack: {:?} \n", stack);

    println!("stack.push_front(2);");
    stack.push_front(2);

    
    println!("current stack: {:?} \n", stack);

    println!("stack.push_front(3);");
    stack.push_front(3);

    
    println!("current stack: {:?} \n", stack);

    println!("stack.push_front(4);");
    stack.push_front(4);

    
    println!("current stack: {:?} \n", stack);

    println!("pop an elements off the stack stack.pop_front() returns -> {:?}", stack.pop_front());
    
    println!("current stack: {:?}\n", stack);

    println!("pop an elements off the stack stack.pop_front() returns -> {:?}", stack.pop_front());
    
    println!("current stack: {:?}\n", stack);

    println!("peek at the top of the stack stack.get(0) returns -> {:?}", stack.get(0));
    
    println!("current stack (has no changes): {:?}\n", stack);

    println!("clear the stack with: deque.clear();");
    stack.clear();
    
    println!("current stack: {:?}\n", stack);

    println!("check if stack is empty with stack.is_empty(); -> {:?}", stack.is_empty());

}