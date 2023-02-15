use rand::{random, thread_rng, Rng};

pub fn rand_select<T>(a: T, b: T) -> T {
    if random() {
        a
    } else {
        b
    }
}

pub fn rand_select3<T>(a: T, b: T, c: T) -> T {
    match thread_rng().gen_range(0..3) {
        0 => a,
        1 => b,
        _ => c,
    }
}
