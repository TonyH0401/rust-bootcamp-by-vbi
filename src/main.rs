fn main() {
    println!("Hello, Rust Bootcamp by VBI Academy, class-2-exercises-answers!");
}

// this is called "dangling", I've detail notes in Notion about this,
// I will keep things short and simple here.
// "dangling" is when a pointer points to a memory zone,
// that is no longer exists or in technical term be deallocated,
// how can a memory zone be deallocated? There are a few ways,
// either by deleting the variable that houses the memory zone,
// or the variable is out-of-scope, when the variable is out-of-scope,
// it is dropped and cease to exist.
// Why return "&String" is "dangling"? Because it's returning a reference,
// a reference that points to a variable which is out-of-scope,
// meaning it points to variable that has been deallocated.
// that is why we must not return reference type.
// fn dangling() -> &String {
//     let s = String::from("hello");
//     &s
// }
