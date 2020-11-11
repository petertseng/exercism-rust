Unfortunate that from the outset, we have nine compile errors
However, they are all of the same form, so fixing one should fix them all.
```the size for values of type `str` cannot be known at compilation time```.
So let's see how we can go about fixing them.
It's clear that I need to choose what type new should take.
And the best way for me to do that is to plan out what I want my struct to hold.
My instinct is that I would like to do:

```rust
#[derive(Clone)]
pub struct Xorcism<'a, I: Iterator<Item = &'a u8> + Clone> {
    key: std::iter::Cycle<I>,
}

impl<'a, I: Iterator<Item = &'a u8> + Clone> Xorcism<'a, I> {
    pub fn new<Key: IntoIterator<Item = &'a u8>>(key: Key) -> Xorcism<'a, I> where Key: IntoIterator<IntoIter = I> {
        Self {
            key: key.into_iter().cycle(),
        }
    }
}
```

However, the types don't match up for that, and I determined this was because I did not pay enough attention to the types that `new` must accept.
I would like to log this failed experiment to record what happens when you haven't thought about what types your `new` should accept.
To rectify these wrongs, I took a look:

* `&[u8]` (`identity`, `munge_output_has_len`, every test that uses `as_bytes`)
* `&&[u8]` (`statefulness`)
* `&str`

Okay, well, we have `impl AsRef<[u8]> for str`, so I guess we're going with AsRef for this one.

```rust
    pub fn new(key: &'a AsRef<[u8]>) -> Xorcism<'a> {
        unimplemented!()
    }
```

I have the following complaint that should be directed to rustc instead of to this exercise:

Attempting this gives you:

```
error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> tests/xorcism.rs:107:41
    |
107 |         let mut xorcism1 = Xorcism::new(key);
    |                                         ^^^ doesn't have a size known at compile-time
    |
    = help: the trait `std::marker::Sized` is not implemented for `str`
    = note: required for the cast to the object type `dyn std::convert::AsRef<[u8]>`
```

Someone looking at this might think "Oh, well, Sized isn't implemented for `str`, and I can't implement it since I control neither, so this was obviously not the right approach."
Now, https://doc.rust-lang.org/std/marker/trait.Sized.html clearly states you can use `?Sized`, but this compiler error doesn't mention this possibility, and I think it should, otherwise someone might wrongfully stray away from this approach.
I'm not sure what we are capable of doing to solve this in the context of this exercise, though.

Oh well, let's used `?Sized` then.

```
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        unimplemented!()
    }
```

Since `as_ref()` is going to get us `&'a [u8]` out of this one, the previous type for Xorcism can still work, with some modification.

```rust
pub struct Xorcism<'a> {
    key: std::iter::Cycle<std::slice::Iter<'a, u8>>,
}
```
