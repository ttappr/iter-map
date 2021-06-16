# iter-map
`iter-map` adds a new method to the Rust `Iterator` classes, `.map_iter()` that gives the ultimate flexibility in how the data from the iterator is transformed. The method takes a callback that receives a reference to the iterator on each invocation. `.map_iter()` produces a new iterator that calls the callback on each infocation of `.next()`.

Sometimes to accomlish very specific things with the standard iterators becomes very complex. The code could be more simply written with a `for` loop or using `.for_each()` to populate an external object. This provides similar capability, but keeps more in the spirit of functional programming with iterators.

## Example Usage

The standard iterator intersperse methods don't allow for interspersing at arbitrary points in the stream of items. The code below shows a simple transform to inject an extra character before each `o`.

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
