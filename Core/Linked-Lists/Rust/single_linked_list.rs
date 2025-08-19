/* Simple Linked List Implementation in Rust */


/* Structure for each node */
struct listNode{
    val: String,
    next: Option<Box<ListNode>>,
}

/* Linked List "Object" */
pub struct linkedList{
    head: Option<Box<ListNode>>,
    tail: Option<Box<ListNode>>,
}

/* Adding functions to the linkedList "object" */
impl linkedList{

    /* Constructor for a new linked list - initialize */
    pub fn new() -> Self {
        linkedList{None, None};
    }
    
    /* Printing the contents of the current list */
    pub fn print_list(&self){
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

    /* Insert a node at the tail */
    pub fn insert_node(&self, val: String){
        let mut cur: ListNode = self.head;
        
    }

    /* Determine if the provided value is in the list */
    pub fn find(&self, target: &str) -> bool {

    }
}