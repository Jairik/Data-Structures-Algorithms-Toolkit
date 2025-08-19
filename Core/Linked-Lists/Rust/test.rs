/* Example implementation of linked list */
use std::io;
mod single_linked_list;
use single_linked_list::linkedList;

fn main() {
    let mut linkedList = linkedList::new();
    // let s: String = "Hello World";

    // Get a string from the user
    println!("Enter a word to convert to a linked list: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap().expect("Failed to read the line");
    
    // Add each character to a linked list and print it
    for c in input.trim().chars() {
        linkedList.insert_node(c.to_string());
    }
    linkedList.print_list();

    // Prompt the user to find an item in the list
    print!("Search for: ");
    let mut search = String::new();
    io::stdin().read_line(&mut search).expect("Failed to read the line");
    
    // Search the list, and display the status
    let found: bool = linkedList.find(search);
    if(found){
        println!("{} was found in the Linked List!", search);
    }
    else{
        println!("{} was not found in the Linked List...", search);
    }
}

