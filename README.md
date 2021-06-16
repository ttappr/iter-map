# iter-map
`iter-map` adds a new method to the Rust `Iterator` classes, `.map_iter()`, that gives the ultimate flexibility in how the data from the iterator is transformed. The method takes a callback as a parameter and produces a new iterator that invokes the callback on each invocation of its `.next()` method and passes it a reference to the original iterator. 

Sometimes to accomlish very specific things with the standard iterator methods becomes very complex. The code could be more simply written with a `for` loop or using `.for_each()` to populate an external object. This provides similar capability, but allows a more functional approach if that's preferred.

## Example Usage

Below is a simple example of interspersing an extra character before each occurence of `o`.

```rust
 use iter_map::IntoIterMap;

 let mut b = true;

 let s = "hello world!".chars().peekable().iter_map(|iter| {
     if let Some(&ch) = iter.peek() {
         if ch == 'o' && b {
             b = false;
             Some('0')
         } else {
             b = true;
             iter.next()
         }
     } else { 
         None 
     }
 }).collect::<String>();

 assert_eq!(&s, "hell0o w0orld!");
```

This is just one possible application of `.iter_map()`. 
