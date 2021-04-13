pub trait DedupReplaceFor{
    type Item;
    
    fn dedup_and_replace_for(&mut self, same: &Self::Item, to: Self::Item, occurence: usize);
    fn dedup_and_replace_for_by(&mut self, to: Self::Item, same_as: impl Fn(&Self::Item) -> bool, occurence: usize);
    fn dedup_and_replace_for_by_key<S: PartialEq>(&mut self, same: &S, to: Self::Item, occurence: usize, key: impl Fn(&Self::Item) -> &S);
    fn dedup_and_replace_for_then_build(&mut self, same: &Self::Item, occurence: usize, build: impl Fn(Self::Item) -> Self::Item);
    fn dedup_and_replace_for_by_key_then_build<S: PartialEq> (&mut self, same: &S, occurence: usize, key: impl Fn(&Self::Item) -> &S, build: impl Fn(Self::Item) -> Self::Item);
    fn dedup_and_replace_for_by_then_build(&mut self, occurence: usize, same_as: impl Fn(&Self::Item) -> bool, build: impl Fn(Self::Item) -> Self::Item);
}

impl<T> DedupReplaceFor for Vec<T>
where
    T: PartialEq + Copy,
{
    type Item = T;
    
    #[inline]
    fn dedup_and_replace_for(&mut self, same: &Self::Item, to: Self::Item, occurence: usize) {
        self.dedup_and_replace_for_by_then_build(occurence, |a| a == same, |_| to)
    }

    #[inline]
    fn dedup_and_replace_for_by(&mut self, to: Self::Item, same_as: impl Fn(&Self::Item) -> bool, occurence: usize) {
        self.dedup_and_replace_for_by_then_build(occurence, same_as, |_| to)
    }

    #[inline]
    fn dedup_and_replace_for_by_key<S: PartialEq>(&mut self, same: &S, to: Self::Item, occurence: usize, key: impl Fn(&Self::Item) -> &S) {
        self.dedup_and_replace_for_by_then_build(occurence, |a| key(a) == same, |_| to)
    }

    #[inline]
    fn dedup_and_replace_for_then_build(&mut self, same: &Self::Item, occurence: usize, build: impl Fn(Self::Item) -> Self::Item) {
        self.dedup_and_replace_for_by_then_build(occurence, |a| a == same, build)
    }

    #[inline]
    fn dedup_and_replace_for_by_key_then_build<S: PartialEq>(&mut self, same: &S, occurence: usize, key: impl Fn(&Self::Item) -> &S, build: impl Fn(Self::Item) -> Self::Item) {
        self.dedup_and_replace_for_by_then_build(occurence, |a| key(a) == same, build)
    }

    fn dedup_and_replace_for_by_then_build(&mut self, occurence: usize, same_as: impl Fn(&Self::Item) -> bool, build: impl Fn(Self::Item) -> Self::Item) {
        if occurence == 0 {
            for item in self.into_iter() {
                *item = build(*item)
            }
            return;
        }
        let mut duplicate_index = vec![];
        let mut duplicate_count = 0;
        for (idx, t) in self.iter().enumerate() {
            if same_as(t) {
                duplicate_count += 1;
                if duplicate_count == occurence {
                    duplicate_index.push(idx + 1 - occurence);
                    duplicate_count = 0;
                }
            } else {
                duplicate_count = 0;
            }
        }
        for idx in duplicate_index.into_iter().rev() {
            for _ in 1..occurence {
                self.remove(idx);
            }
            self[idx] = build(self[idx]);
        }
    }
}