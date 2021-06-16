
//! A generic implementation for an iterator generating function implemented on
//! `IntoIterator`. `.iter_map()` takes a callback as its parameter. This 
//! callback gets invoked on each `.next()` invocation and receives a mutable 
//! ref to an internal iterator.
//!
//! ```
//! use iter_map::*;
//!
//! fn main() 
//! {
//!     let mut i = 0;
//!
//!     let v = [1, 2, 3, 4, 5, 6].iter_map(move |iter| {
//!         i += 1;
//!         if i % 3 == 0 {
//!             Some(0)
//!         } else {
//!             iter.next().copied()
//!         }
//!     }).collect::<Vec<_>>();
//!  
//!     assert_eq!(v, vec![1, 2, 0, 3, 4, 0, 5, 6, 0]);
//! }
//! ```


/// With ParamFromFnIter you can create iterators simply by calling 
/// `ParamFromFnIter::new()` and passing it a callback. The callback will be 
/// invoked when `.next()` is invoked on the iterator returned by `.new()`. 
///
pub struct ParamFromFnIter<F, D>
{
    callback: F,
    data: D,
}

impl<F, D, R> ParamFromFnIter<F, D>
//
where F: FnMut(&mut D) -> Option<R>,
{
    /// Creates a new `ParamFromFnIter` iterator instance. This
    /// does pretty much the same thing as `std::iter::from_fn()` except the 
    /// callback signature of this class takes a data argument.
    ///
    /// This provides a flexible and simple way to create new iterators by 
    /// defining a callback. 
    ///
    /// One application for this could be to implement functions that operate
    /// similar to generators. The example below shows how one could implement a
    /// chunking "generator" that breaks up the items from the iterator passed
    /// in to `chunker()` into chunks. This could also be achieved with the 
    /// standard function, `std::iter::from_fn()`.
    ///
    /// ```
    /// use iter_map::ParamFromFnIter;
    /// 
    /// use std::cell::RefCell;
    /// use std::rc::Rc;
    ///
    /// fn chunker<I>(chunk_size    : usize, 
    ///               data          : I
    ///              ) -> impl Iterator<Item = impl Iterator<Item = I::Item>>
    /// where I: Iterator
    /// {
    ///     let p = Rc::new(RefCell::new(data.peekable()));
    ///     ParamFromFnIter::new(
    ///         (),
    ///         move |_| {
    ///             if p.borrow_mut().peek().is_some() {
    ///                 Some(chunk(chunk_size, p.clone()))
    ///             } else {
    ///                 None
    ///             }
    ///         })
    /// }
    ///
    /// fn chunk<I, T>(chunk_size   : usize, 
    ///                data         : Rc<RefCell<I>>
    ///               ) -> impl Iterator<Item = I::Item>
    /// where I: Iterator<Item = T>
    /// {
    ///     let mut i = 0;
    ///     ParamFromFnIter::new(
    ///         (),
    ///         move |_| {
    ///             if i < chunk_size {
    ///                 i += 1;
    ///                 data.borrow_mut().next()
    ///             } else {
    ///                 None
    ///             }
    ///         })
    /// }
    ///
    /// let mut result = String::new();
    ///
    /// for s in chunker(2, [1, 2, 3, 4, 5, 6].iter()) {
    ///     for n in s {
    ///         result.push_str(&format!("{}, ", n));
    ///     }
    ///     result.push('\n');
    /// }
    ///
    /// assert_eq!(&result, "1, 2, \n3, 4, \n5, 6, \n");
    /// ```
    ///
    /// # Arguments
    /// * `data`      - Data that will be passed to the callback on each 
    ///                 invocation.
    /// * `callback`  - The callback that gets invoked when `.next()` is invoked
    ///                 on the returned iterator.
    ///    
    pub fn new(data: D, callback: F) -> Self
    {
        ParamFromFnIter { callback, data }
    }
}

/// Implements Iterator for ParamFromFnIter. 
///
impl<F, D, R> Iterator for ParamFromFnIter<F, D>
//
where F: FnMut(&mut D) -> Option<R>,
{
    type Item = R;
    
    /// Iterator method that returns the next item.
    /// Invokes the client code provided iterator, passing it `&mut self.data`.
    ///
    fn next(&mut self) -> Option<Self::Item>
    {
        (self.callback)(&mut self.data)
    }
}

/// A trait to add the `.iter_map()` method to any existing class.
///
pub trait IntoIterMap<F, I, R, T>
//
where F: FnMut(&mut I) -> Option<R>,
      I: Iterator<Item = T>,
{
    /// Returns a `ParamFromFnIter` iterator which wraps the iterator it's 
    /// invoked on.
    ///
    /// # Arguments
    /// * `callback`  - The callback that gets invoked by `.next()`.
    ///                 This callback is passed the original iterator as its
    ///                 parameter.
    ///
    fn iter_map(self, callback: F) -> ParamFromFnIter<F, I>;
}

/// Adds `.iter_map()` method to all IntoIterator classes.
///
impl<F, I, J, R, T> IntoIterMap<F, I, R, T> for J
//
where F: FnMut(&mut I) -> Option<R>,
      I: Iterator<Item = T>,
      J: IntoIterator<Item = T, IntoIter = I>,
{
    /// Returns an iterator that invokes the callback in `.next()`, passing it
    /// the original iterator as an argument. The callback can return any
    /// arbitrary type within an `Option`.
    ///
    /// ```
    /// use iter_map::IntoIterMap;
    ///
    /// let mut b = true;
    ///
    /// let s = "hello world!".chars().peekable().iter_map(|iter| {
    ///     if let Some(&ch) = iter.peek() {
    ///         if ch == 'o' && b {
    ///             b = false;
    ///             Some('0')
    ///         } else {
    ///             b = true;
    ///             iter.next()
    ///         }
    ///     } else { None }}).collect::<String>();
    ///
    /// assert_eq!(&s, "hell0o w0orld!");
    /// ```
    fn iter_map(self, callback: F) -> ParamFromFnIter<F, I>
    {
        ParamFromFnIter::new(self.into_iter(), callback)
    }
}


#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn sanity_check() {
        let mut i = 0;
        let v = [1, 2, 3, 4, 5, 6].iter_map(move |iter| {
            i += 1;
            if i % 3 == 0 {
                Some(0)
            } else {
                iter.next().copied()
            }
        }).collect::<Vec<_>>();
        assert_eq!(v, vec![1, 2, 0, 3, 4, 0, 5, 6, 0]);
    }
}
