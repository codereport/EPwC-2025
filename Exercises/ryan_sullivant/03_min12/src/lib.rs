use std::fmt::Debug;
use std::{marker::PhantomData, ops::Deref};

// BinaryCounter and functions

pub struct BinaryCounter<T, Op>
where
    T: PartialEq + Copy + Debug,
    Op: FnMut(&T, &T) -> T,
{
    counter: Vec<T>,
    op: Op,
    zero: T,
}

impl<'a, T, Op> BinaryCounter<T, Op>
where
    T: PartialEq + Copy + Debug,
    Op: FnMut(&T, &T) -> T,
{
    pub fn new(op: Op, zero: T) -> Self {
        Self {
            counter: Vec::new(),
            op,
            zero,
        }
    }

    pub fn with_capacity(op: Op, zero: T, capacity: usize) -> Self {
        Self {
            counter: Vec::with_capacity(capacity),
            op,
            zero,
        }
    }

    pub fn reserve(self: &mut Self, capacity: usize) {
        if self.counter.len() < capacity {
            self.counter.reserve(capacity - self.counter.len());
        }
    }

    pub fn add(self: &mut Self, mut x: T) {
        let last = self.counter.len();
        x = add_to_counter(&mut self.counter, 0usize, last, &mut self.op, &self.zero, x);
        if x != self.zero {
            self.counter.push(x);
        }
    }

    pub fn reduce(self: &mut Self) -> T {
        let last = self.counter.len();
        reduce_counter(&mut self.counter, 0usize, last, &mut self.op, &self.zero)
    }

    pub fn print(self: &Self) {
        println!("counter: {:?}", &self.counter);
    }
}

fn add_to_counter<T, Op>(
    counter: &mut [T],
    mut first: usize,
    last: usize,
    mut op: Op,
    zero: &T,
    mut carry: T,
) -> T
where
    T: PartialEq + Copy,
    Op: FnMut(&T, &T) -> T,
{
    while first != last {
        unsafe {
            if counter.get_unchecked(first) == zero {
                *counter.get_unchecked_mut(first) = carry;
                return *zero;
            }
            carry = op(counter.get_unchecked(first), &carry);
            *counter.get_unchecked_mut(first) = zero.clone();
            first += 1;
        }
    }
    carry
}

fn reduce_counter<T, Op>(
    counter: &mut [T],
    mut first: usize,
    last: usize,
    mut op: Op,
    zero: &T,
) -> T
where
    T: PartialEq + Copy,
    Op: FnMut(&T, &T) -> T,
{
    unsafe {
        while first != last && counter.get_unchecked(first) == zero {
            first += 1;
        }
    }

    if first == last {
        return *zero;
    }

    unsafe {
        let mut result = counter.get_unchecked(first).clone();
        first += 1;
        while first != last {
            if counter.get_unchecked(first) != zero {
                result = op(&result, counter.get_unchecked(first));
            }
            first += 1;
        }

        result
    }
}

// ListPool and functions
type ListType = usize;
#[derive(Default, Debug, PartialEq)]
struct ListPoolNode<T>
where
    T: Default,
{
    value: T,
    next: ListType,
}

#[derive(Default, Debug, PartialEq)]
struct ListPool<T>
where
    T: Default,
{
    pool: Vec<ListPoolNode<T>>,
    free_list: ListType,
}

impl<T> ListPool<T>
where
    T: Default,
{
    pub fn end() -> ListType {
        0 as ListType
    }
    pub fn is_end(x: ListType) -> bool {
        x == Self::end()
    }
    pub fn empty(self: &Self) -> bool {
        self.pool.is_empty()
    }
    pub fn size(self: &Self) -> usize {
        self.pool.len()
    }
    pub fn capacity(self: &Self) -> usize {
        self.pool.capacity()
    }
    pub fn new() -> Self {
        Self {
            pool: Vec::new(),
            free_list: Self::end(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            pool: Vec::with_capacity(capacity),
            free_list: Self::end(),
        }
    }

    fn node(self: &Self, x: ListType) -> &ListPoolNode<T> {
        unsafe { self.pool.get_unchecked((x - 1) as usize) }
    }

    fn node_mut(self: &mut Self, x: ListType) -> &mut ListPoolNode<T> {
        unsafe { self.pool.get_unchecked_mut((x - 1) as usize) }
    }

    fn new_list(self: &mut Self) -> ListType {
        self.pool.push(ListPoolNode::default());
        self.pool.len() as ListType
    }

    pub fn value(self: &Self, x: ListType) -> &T {
        &self.node(x).value
    }

    pub fn value_mut(self: &mut Self, x: ListType) -> &mut T {
        &mut self.node_mut(x).value
    }

    pub fn next(self: &Self, x: ListType) -> &ListType {
        &self.node(x).next
    }

    pub fn next_mut(self: &mut Self, x: ListType) -> &mut ListType {
        &mut self.node_mut(x).next
    }

    pub fn free(self: &mut Self, x: ListType) -> ListType {
        let tmp = *self.next(x);
        *self.next_mut(x) = self.free_list;
        self.free_list = x;
        tmp
    }

    pub fn allocate(self: &mut Self, val: T, x: ListType) -> ListType {
        let mut list = self.free_list;
        if Self::is_end(self.free_list) {
            list = self.new_list();
        } else {
            self.free_list = *self.next(self.free_list);
        }
        *self.value_mut(list) = val;
        *self.next_mut(list) = x;
        list
    }
}

fn free_list<T>(pool: &mut ListPool<T>, mut x: ListType)
where
    T: Default,
{
    while !ListPool::<T>::is_end(x) {
        x = pool.free(x);
    }
}

fn min_element_list<T, C>(pool: &mut ListPool<T>, mut x: ListType, cmp: C) -> ListType
where
    T: Default,
    C: Fn(&T, &T) -> bool,
{
    if ListPool::<T>::is_end(x) {
        return x;
    }
    let mut current_min = x;
    x = *pool.next(x);
    while !ListPool::<T>::is_end(x) {
        if cmp(pool.value(x), pool.value(current_min)) {
            current_min = x;
        }
        x = *pool.next(x);
    }
    current_min
}

// min_element functions
pub fn min_element<T, C>(rng: &[T], mut first: usize, last: usize, cmp: C) -> usize
where
    T: PartialEq + Copy,
    C: Fn(&T, &T) -> bool,
{
    if first == last {
        return last;
    }
    let mut current_min = first;
    first += 1;

    unsafe {
        while first != last {
            if cmp(rng.get_unchecked(first), rng.get_unchecked(current_min)) {
                current_min = first;
            }
            first += 1;
        }
    }

    current_min
}

fn cmp_deref<'a, T, C>(cmp: C, rng: &'a [T]) -> impl Fn(&usize, &usize) -> bool
where
    C: Fn(&T, &T) -> bool + 'a,
{
    move |x: &usize, y: &usize| unsafe { cmp(rng.get_unchecked(*x), rng.get_unchecked(*y)) }
}

fn min_op<'a, T, C>(cmp: C, rng: &'a [T]) -> impl Fn(&usize, &usize) -> usize
where
    C: Fn(&T, &T) -> bool + 'a,
{
    let cmp_d = cmp_deref(cmp, rng);
    move |&x, &y| {
        if cmp_d(&y, &x) { y } else { x }
    }
}

pub fn min_element_binary<'a, T, C>(rng: &'a [T], mut first: usize, last: usize, cmp: C) -> usize
where
    T: PartialEq + Copy,
    C: Fn(&T, &T) -> bool + 'a,
{
    if first == last {
        return last;
    }

    let mut counter = BinaryCounter::with_capacity(min_op(cmp, rng), last, 16);

    while first != last {
        counter.add(first);
        first += 1;
    }

    counter.reduce()
}

type Min12EltType<I> = (I, ListType);
fn combine<I>(
    pool: &mut ListPool<I>,
    winner: Min12EltType<I>,
    loser: Min12EltType<I>,
) -> Min12EltType<I>
where
    I: Default,
{
    free_list(pool, loser.1);
    (winner.0, pool.allocate(loser.0, winner.1))
}

// had to resort to taking pool as a mut pointer and unsafe because
// of the min_element_list() call in min_element12()
fn min12_op<'a, T, C>(
    cmp: C,
    rng: &'a [T],
    pool: *mut ListPool<usize>,
) -> impl FnMut(&Min12EltType<usize>, &Min12EltType<usize>) -> Min12EltType<usize>
where
    C: Fn(&T, &T) -> bool + 'a,
{
    let cmp_d = cmp_deref(cmp, rng);
    move |&x, &y| {
        if cmp_d(&y.0, &x.0) {
            combine(unsafe { pool.as_mut() }.unwrap(), y, x)
        } else {
            combine(unsafe { pool.as_mut() }.unwrap(), x, y)
        }
    }
}

pub fn min_element12<'a, T, C>(
    rng: &'a [T],
    mut first: usize,
    last: usize,
    cmp: C,
) -> (usize, usize)
where
    T: PartialEq + Copy + Default + Ord + Debug,
    C: Fn(&T, &T) -> bool + 'a,
{
    if first == last || first + 1 == last {
        return (first, first);
    }

    let mut pool = ListPool::with_capacity(256);
    let op = min12_op(cmp, rng, &mut pool);
    let mut counter = BinaryCounter::with_capacity(op, (last, ListPool::<usize>::end()), 16);
    while first != last {
        counter.add((first, ListPool::<usize>::end()));
        first += 1;
    }
    let (min1, min1_list) = counter.reduce();
    // can't move cmp a second time
    // let min2 = min_element_list(&mut pool, min1_list, cmp_deref(cmp, rng));
    let min2_loc = min_element_list(&mut pool, min1_list, cmp_deref(less(), rng));
    (min1, *pool.value(min2_loc))
}

pub fn min_element12_practical<'a, T, C>(
    rng: &'a [T],
    mut first: usize,
    last: usize,
    cmp: C,
) -> (usize, usize)
where
    T: PartialEq + Copy + Default + Ord + Debug,
    C: Fn(&T, &T) -> bool + 'a,
{
    if first == last || first + 1 == last {
        return (first, first);
    }
    let cmp_d = cmp_deref(cmp, rng);
    let (mut min1, mut min2) = if cmp_d(&(first + 1), &first) {
        (first + 1, first)
    } else {
        (first, first + 1)
    };
    first += 2;
    while first != last {
        if cmp_d(&first, &min2) {
            if cmp_d(&first, &min1) {
                min2 = min1;
                min1 = first;
            } else {
                min2 = first;
            }
        }
        first += 1;
    }
    (min1, min2)
}

pub fn less<T>() -> impl Fn(&T, &T) -> bool
where
    T: Ord,
{
    |x, y| x < y
}

#[cfg(test)]
mod tests {
    use crate::min_element;

    use super::*;

    #[test]
    fn test_min_element() {
        let v = vec![3, 8, 0, 7, 9, 1, 2, 5];
        assert_eq!(min_element(&v, 0, v.len(), less()), 2);
    }

    #[test]
    fn test_min_element_binary() {
        let v = vec![3, 8, 0, 7, 9, 1, 2, 5];
        assert_eq!(min_element_binary(&v, 0, v.len(), less()), 2);
    }

    #[test]
    fn test_min_element12() {
        let v = vec![3, 8, 0, 7, 9, 1, 2, 5];
        assert_eq!(min_element12(&v, 0, v.len(), less()), (2, 5));
    }

    #[test]
    fn test_min_element12_practical() {
        let v = vec![3, 8, 0, 7, 9, 1, 2, 5];
        assert_eq!(min_element12_practical(&v, 0, v.len(), less()), (2, 5));
    }
}
