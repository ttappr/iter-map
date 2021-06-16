# iter-map
`iter-map` adds a new method to the Rust `Iterator` classes, `.map_iter()` that gives the ultimate flexibility in how the data from the iterator is transformed. The method takes a callback that receives a reference to the iterator on each invocation. `.map_iter()` produces a new iterator that calls the callback on each infocation of `.next()`.
