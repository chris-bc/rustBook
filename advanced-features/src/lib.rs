use std::ops::Add;

struct Counter {
    value: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        //
        Some(42)
    }
}

struct Millimetres(u32);
struct Metres(u32);

impl Add<Metres> for Millimetres {
    type Output = Millimetres;

    fn add(self, other: Metres) -> Millimetres {
        Millimetres(self.0 + (other.0 * 1000))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_metres_to_millimetres() {
        assert_eq!(Millimetres(500).add(Metres(3)).0, 3500);
    }
}
