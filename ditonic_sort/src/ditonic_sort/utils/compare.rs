pub fn compare<T: Eq>(a: &[T], b: &[T]) -> bool {
    let mut a_iter = a.iter();
    let mut b_iter = b.iter();
    let mut a_len = a.len();
    let b_len = b.len();

    if a_len != b_len {
        return false;
    }

    while a_len > 0 {
        let v = a_iter.next().unwrap();
        let w = b_iter.next().unwrap();

        if v != w {
            return false;
        }

        a_len -= 1;
    }

    return true;
}