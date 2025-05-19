#[inline]
fn minmax2<T>(a: usize, b: usize, s: &[T]) -> (usize, usize)
where
    T: Ord,
{
    if &s[b] < &s[a] { (b, a) } else { (a, b) }
}

#[inline]
fn min2<T>(a: usize, b: usize, s: &[T]) -> usize
where
    T: Ord,
{
    if &s[b] < &s[a] { b } else { a }
}

#[inline]
fn max2<T>(a: usize, b: usize, s: &[T]) -> usize
where
    T: Ord,
{
    if &s[b] < &s[a] { a } else { b }
}

#[inline]
fn minmax2_unsafe<T>(a: usize, b: usize, s: &[T]) -> (usize, usize)
where
    T: Ord,
{
    if &unsafe { s.get_unchecked(b) } < &unsafe { s.get_unchecked(a) } {
        (b, a)
    } else {
        (a, b)
    }
}

#[inline]
fn min2_unsafe<T>(a: usize, b: usize, s: &[T]) -> usize
where
    T: Ord,
{
    if &unsafe { s.get_unchecked(b) } < &unsafe { s.get_unchecked(a) } {
        b
    } else {
        a
    }
}

#[inline]
fn max2_unsafe<T>(a: usize, b: usize, s: &[T]) -> usize
where
    T: Ord,
{
    if &unsafe { s.get_unchecked(b) } < &unsafe { s.get_unchecked(a) } {
        a
    } else {
        b
    }
}

#[inline]
pub fn minmax_element<T>(s: &[T]) -> Option<(usize, usize)>
where
    T: Ord,
{
    if s.len() < 1 {
        return None;
    } else if s.len() < 2 {
        return Some((0, 0));
    }
    let (mut min, mut max) = minmax2(0, 1, s);
    let mut curr = 2;

    while curr != s.len() {
        let next = curr + 1;
        if next < s.len() {
            let (p_min, p_max) = minmax2(curr, next, s);
            min = min2(min, p_min, s);
            max = max2(max, p_max, s);
            curr = next + 1;
        } else {
            min = min2(min, curr, s);
            max = max2(max, curr, s);
            curr += 1;
        }
    }

    Some((min, max))
}

#[inline]
pub fn minmax_element_unsafe<T>(s: &[T]) -> Option<(usize, usize)>
where
    T: Ord,
{
    if s.len() < 1 {
        return None;
    } else if s.len() < 2 {
        return Some((0, 0));
    }
    let (mut min, mut max) = minmax2_unsafe(0, 1, s);
    let mut curr = 2;

    while curr != s.len() {
        let next = curr + 1;
        if next < s.len() {
            let (p_min, p_max) = minmax2_unsafe(curr, next, s);
            min = min2_unsafe(min, p_min, s);
            max = max2_unsafe(max, p_max, s);
            curr = next + 1;
        } else {
            min = min2_unsafe(min, curr, s);
            max = max2_unsafe(max, curr, s);
            curr += 1;
        }
    }

    Some((min, max))
}

#[inline]
pub fn minmax_element_iter<T>(s: &[T]) -> Option<(usize, usize)>
where
    T: Ord,
{
    if s.len() < 1 {
        return None;
    }

    let minmax2_ind = |s, ind| {
        let (min, max) = minmax2(0, 1, s);
        (min + 2 * ind, max + 2 * ind)
    };

    let chunks = s.chunks_exact(2);
    let remainder = chunks.remainder();
    let mut enumerated = chunks.enumerate();
    let init = if remainder.len() > 0 {
        // remainder can only be the last element
        (s.len() - 1, s.len() - 1)
    } else {
        let (ind, sl) = enumerated.next().unwrap();
        minmax2_ind(sl, ind)
    };

    let (min, max) = enumerated.fold(init, |acc, e| {
        let ind = e.0;
        let sl = e.1;
        let (min, max) = minmax2_ind(sl, ind);
        (min2(acc.0, min, s), max2(acc.1, max, s))
    });

    Some((min, max))
}

#[inline]
pub fn min_then_max<T>(s: &[T]) -> Option<(usize, usize)>
where
    T: Ord,
{
    if s.len() < 1 {
        return None;
    }
    if s.len() < 2 {
        return Some((0, 0));
    }
    let min = (1..s.len()).fold(0, |acc, ind| min2(acc, ind, s));
    let max = (1..s.len()).fold(0, |acc, ind| max2(acc, ind, s));
    Some((min, max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minmax() {
        let v = vec![1, 3, 4, 5, 3, 5];
        assert_eq!(minmax_element(&v), Some((0, 5)));
        let v = vec![1, 3, 4, 5, 3];
        assert_eq!(minmax_element(&v), Some((0, 3)));
        assert_eq!(minmax_element(&v[0..0]), None);
        let v = vec![1];
        assert_eq!(minmax_element(&v), Some((0, 0)));
        let v = vec![2, 1];
        assert_eq!(minmax_element(&v), Some((1, 0)));
    }

    #[test]
    fn test_minmax_iter() {
        let v = vec![1, 3, 4, 5, 3, 5];
        assert_eq!(minmax_element_iter(&v), Some((0, 5)));
        let v = vec![1, 3, 4, 5, 3];
        assert_eq!(minmax_element_iter(&v), Some((0, 3)));
        assert_eq!(minmax_element_iter(&v[0..0]), None);
        let v = vec![1];
        assert_eq!(minmax_element_iter(&v), Some((0, 0)));
        let v = vec![2, 1];
        assert_eq!(minmax_element_iter(&v), Some((1, 0)));
    }

    #[test]
    fn test_min_then_max() {
        let v = vec![1, 3, 4, 5, 3, 5];
        assert_eq!(min_then_max(&v), Some((0, 5)));
        let v = vec![1, 3, 4, 5, 3];
        assert_eq!(min_then_max(&v), Some((0, 3)));
        assert_eq!(min_then_max(&v[0..0]), None);
        let v = vec![1];
        assert_eq!(min_then_max(&v), Some((0, 0)));
        let v = vec![2, 1];
        assert_eq!(min_then_max(&v), Some((1, 0)));
    }
}
