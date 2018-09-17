// Modified from Rust core TakeWhile
// Also takes the first item which breaks predicate

pub trait MyIterators {
    type Item;
    fn take_do_while<P>(self, predicate: P) -> TakeDoWhile<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        TakeDoWhile {
            iter: self,
            flag: false,
            predicate,
        }
    }
}

impl<I: Iterator> MyIterators for I {
    type Item = I::Item;
}

pub struct TakeDoWhile<I, P> {
    iter: I,
    flag: bool,
    predicate: P,
}

impl<I: Iterator, P> Iterator for TakeDoWhile<I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
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
