/* Simple Linked List Implementation in Rust */

struct listNode(){
    val: String,
    next: Option<Box<ListNode>>,
}

fn printList(head: Option<Box<ListNode>>){
    let mut temp_node = head;
    while let Some(node) = temp_node {
        print!("{}", node.val);
        temp_node = &node.next;
        if temp_node.is_some() {
            print(" => ");
        }
    }
    println!("");  // Clear the line
    return
}

fn insertNode(tail: Option<Box<ListNode>>){
    
}

fn main() {
    // !TODO
}