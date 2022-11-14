pub fn fill_vec<T>(v: &mut Vec<T>, n: usize, p: fn() -> T) {
    v.clear();

    unsafe { v.set_len(n); }

    for _ in 0..n {
        v.push(p());
    }
}

pub mod sequence {
    pub fn longest_identical_subsequence_of<T: PartialEq + Clone>(vec: &Vec<T>) -> (Option<T>, usize) {
        if vec.len() == 0 {
            return (None, 0);
        }

        let mut iterator = vec.iter();

        let mut last: &T = iterator.next().unwrap();
        let mut longest: (T, usize) = (last.to_owned(), 1);

        let mut count = 1;

        while let Some(v) = iterator.next() {
            if v == last {
                count += 1;
            } else {
                count = 1
            }

            // Update longest
            if count > longest.1 {
                longest.0 = v.to_owned();
                longest.1 = count;
            }

            last = v;
        }

        (Some(longest.0), longest.1)
    }

    pub fn longest_identical_subsequence_of_val<T: PartialEq + Clone>(vec: &Vec<T>, val: T) -> usize {
        if vec.len() == 0 { return 0; }

        let mut iterator = vec.iter();

        let mut longest: usize = 0;

        let mut count = 1;

        while let Some(v) = iterator.next() {
            if v == &val {
                count += 1;
            } else {
                count = 0;
            }

            longest = longest.max(count);
        }

        longest
    }
}