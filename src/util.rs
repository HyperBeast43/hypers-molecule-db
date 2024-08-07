use std::cmp::Ordering;

pub(crate) trait IteratorExt: Iterator {
    fn _cmp_by<I, F>(self, other: I, cmp: F) -> Ordering
    where
        Self: Sized,
        I: IntoIterator,
        F: FnMut(Self::Item, I::Item) -> Ordering,
    {
        use std::ops::ControlFlow;

        #[inline]
        fn iter_compare<A, B, F, T>(mut a: A, mut b: B, f: F) -> ControlFlow<T, Ordering>
        where
            A: Iterator,
            B: Iterator,
            F: FnMut(A::Item, B::Item) -> ControlFlow<T>,
        {
            #[inline]
            fn compare<'a, B, X, T>(
                b: &'a mut B,
                mut f: impl FnMut(X, B::Item) -> ControlFlow<T> + 'a,
            ) -> impl FnMut(X) -> ControlFlow<ControlFlow<T, Ordering>> + 'a
            where
                B: Iterator,
            {
                move |x| match b.next() {
                    None => ControlFlow::Break(ControlFlow::Continue(Ordering::Greater)),
                    Some(y) => match f(x, y) {
                        ControlFlow::Continue(x) => ControlFlow::Continue(x),
                        ControlFlow::Break(x) => ControlFlow::Break(ControlFlow::Break(x)),
                    },
                }
            }

            match a.try_for_each(compare(&mut b, f)) {
                ControlFlow::Continue(()) => ControlFlow::Continue(match b.next() {
                    None => Ordering::Equal,
                    Some(_) => Ordering::Less,
                }),
                ControlFlow::Break(x) => x,
            }
        }

        #[inline]
        fn compare<X, Y, F>(mut cmp: F) -> impl FnMut(X, Y) -> ControlFlow<Ordering>
        where
            F: FnMut(X, Y) -> Ordering,
        {
            move |x, y| match cmp(x, y) {
                Ordering::Equal => ControlFlow::Continue(()),
                non_eq => ControlFlow::Break(non_eq),
            }
        }

        match iter_compare(self, other.into_iter(), compare(cmp)) {
            ControlFlow::Continue(ord) => ord,
            ControlFlow::Break(ord) => ord,
        }
    }
}

impl<T: Iterator> IteratorExt for T {}
