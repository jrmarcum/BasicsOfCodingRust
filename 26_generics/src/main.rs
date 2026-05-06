// Rust generics use trait bounds instead of Go's type constraints.
// Go's `comparable` constraint maps to `PartialEq` in Rust.
// Go's `[S ~[]E, E comparable]` maps to a generic function with a slice
// reference and `PartialEq` bound on the element type.

// Generic function: find the index of the first occurrence of v in s,
// or -1 if not present. Equivalent to Go's SlicesIndex.
fn slices_index<E: PartialEq>(s: &[E], v: &E) -> i32 {
    for (i, item) in s.iter().enumerate() {
        if item == v {
            return i as i32;
        }
    }
    -1
}

// Generic singly-linked list, equivalent to Go's List[T].
struct List<T> {
    head: Option<Box<Element<T>>>,
}

struct Element<T> {
    next: Option<Box<Element<T>>>,
    val: T,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    // Push appends a value to the end of the list.
    fn push(&mut self, v: T) {
        // Find the last None slot and place the new element there.
        let mut current = &mut self.head;
        while current.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }
        *current = Some(Box::new(Element { next: None, val: v }));
    }

    // AllElements returns all values as a Vec.
    fn all_elements(&self) -> Vec<&T> {
        let mut elems = Vec::new();
        let mut current = &self.head;
        while let Some(ref node) = *current {
            elems.push(&node.val);
            current = &node.next;
        }
        elems
    }
}

fn main() {
    let s = vec!["foo", "bar", "zoo"];

    // Type inference works here; no need to specify the type parameter.
    println!("index of zoo: {}", slices_index(&s, &"zoo"));

    let mut lst: List<i32> = List::new();
    lst.push(10);
    lst.push(13);
    lst.push(23);

    // Collect to owned values for display.
    let elems: Vec<i32> = lst.all_elements().into_iter().copied().collect();
    println!("list: {:?}", elems);
}
