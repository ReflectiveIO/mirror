pub fn clamp<T: std::cmp::PartialOrd>(val: T, low: T, high: T) -> T {
    if val > low { if val < high { val } else { high } } else { low }
}
