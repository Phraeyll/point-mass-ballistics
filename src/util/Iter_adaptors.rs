// Modified from Rust core TakeWhile
// Also takes the first item which breaks predicate
// Some(thing), rather than None on first false predicate
pub trait MyIterators {
    type Item;
    fn do_take_while<P>(self, predicate: P) -> DoTakeWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        DoTakeWhile {
            iter: self,
            flag: false,
            predicate,
        }
    }
}

impl<I: Iterator> MyIterators for I {
    type Item = I::Item;
}

pub struct DoTakeWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I: Iterator, P> Iterator for DoTakeWhile<I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.flag {
            None
        } else {
            self.iter.next().and_then(|x| {
                if (self.predicate)(&x) {
                    Some(x)
                } else {
                    self.flag = true;
                    Some(x)
                }
            })
        }
    }
}
