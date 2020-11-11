use std::borrow::Borrow;
use std::iter::Cycle;
use std::slice::Iter;

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    key: Cycle<Iter<'a, u8>>,
    orig_len: usize,
}

pub struct Munger<'a, I: Iterator<Item = Itm> + ExactSizeIterator, Itm: Borrow<u8>> {
    pt: I,
    key: Cycle<Iter<'a, u8>>,
}

impl<'a, I: Iterator<Item = Itm> + ExactSizeIterator, Itm: Borrow<u8>> Iterator
    for Munger<'a, I, Itm>
{
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        self.pt
            .next()
            .map(|x| *x.borrow() ^ self.key.next().unwrap())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let sz = self.pt.len();
        (sz, Some(sz))
    }
}

impl<'a, I: Iterator<Item = Itm> + ExactSizeIterator, Itm: Borrow<u8>> ExactSizeIterator
    for Munger<'a, I, Itm>
{
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized>(key: &'a Key) -> Xorcism<'a> {
        let as_ref = key.as_ref();
        Self {
            key: as_ref.iter().cycle(),
            orig_len: as_ref.len(),
        }
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for datum in data {
            *datum ^= self.key.next().unwrap();
        }
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<
        Data: IntoIterator<IntoIter = Iter, Item = Itm>,
        Iter: Iterator<Item = Itm> + ExactSizeIterator,
        Itm: Borrow<u8>,
    >(
        &mut self,
        data: Data,
    ) -> Munger<'a, Iter, Itm> {
        let key = self.key.clone();
        let pt = data.into_iter();
        // When advance_by(n) is stabilised, use that instead.
        for _ in 0..(pt.len() % self.orig_len) {
            self.key.next();
        }
        Munger { pt, key }
    }
}
