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

I actually implemented `munge_in_place` first (it was quite trivial), which knocks out a good chunk of tests.
But `munge` gets more interesting.

I'm not sure holding `Cycle<Iter<'a, u8>>` will work here.
Especially since the statefulness implies that the `MungeOutput` will probably need to have `mut` on `key`.
I don't think this is possible since it's not possible to prove that `Xorcism` won't also mutate it for the entire time that the `MungeOutput` lives.
I would have to clone the `key` iterator and advance the Xorcism's the appropriate amount.
However, for certain inputs to `munge` (for exampkle `munge` could take an iterator) it's may not be possible to figure out what "the appropriate amount" should be.

Well, let's see what `munge` takes.

* `&[u8]` (`as_bytes`, `statefulness`)
* `&&[u8]` (statefulness)
* impl MungeOutput (`round_trip`)
* `Vec<u8>`

Okay, well that means it's gotta be:

```rust
    pub fn munge<Data: IntoIterator<Item = ???>>(&mut self, data: Data) -> impl MungeOutput
```

Now, Item could be either `u8` or `&u8` here, how to accept both?
That'd be `Borrow`, wouldn't it. Great.

```rust
    pub fn munge<Data: IntoIterator<Item = Itm>, Itm: std::borrow::Borrow<u8>>(&mut self, data: Data) -> impl MungeOutput
```

(Actually I spent some time seeing if it was `AsRef` before understanding that it would be `Borrow`)
... and I'm going to have to restirct the IntoIterator's IntoIter as ExactSizeIterator if I'm going to do my weirdo clone-and-advance solution.
... and now that I'm done and looked at the example to compare, now I see that the example has to do the same, for similar reasons.

I note that I did not need to keep the MungeOutput, and honestly I prefer to return concrete types anyway.

Took about five hours all told (note I haven't done the io section yet).
I see that all the traits that ended up being useful were in the "Useful Traits" section.
Some of my time was wasted because I flailed around uselessly instead of reading that section.
There's nothing that can be done about that though, since if a student chooses not to read the README and just plow ahead, that's a self-inflicted problem.

`reader` was pretty straightforward.
A few minutes to read the docs, a minute or two to think about what values the struct should store, and the implementation was straightforward.
5-10 minutes.

I notice that `writer` is going to be trickier.
Don't know in advance how many bytes the underlying `Write` is going to be able to write.
I ended up writing one byte at a time by making the key peekable.
I don't really like it because I don't know if there is overhead involved in making that many `write` calls (one per byte!).
Other alternative is to block until I can write the entire buffer - I see the example took that approach.
I could also attempt to write more bytes at a time instead of one, which will require more logic to peek at multiple bytes from the key stream (or "rewind" the key stream appropriately by advancing it the appropriate number of times).
10-20 minutes.

Depending on how thorough this exercise aims to be, some tests to consider adding:

* An underlying Read that sometimes errors.
* An underlying Write that sometimes errors.
* An underlying Write that sometimes successfully writes some amount that isn't the entire buffer.
* With either of the two above, test that `write` returns correct number of bytes written - currently `write` is never called directly (actually, that might be okay if the tests calling `write_all` will already fail in the face of incorrect `write` behaviour).
* I understand we're already exploring that possibility in https://github.com/exercism/rust/issues/992, so the argument here would either be "we're already teaching what to do in the face of errors in the other exercise and therefore don't need to repeat the lesson here" vs "we should always instill the habits of doing the right thing in the face of errors, so we should do it here too"
