const SQUAWK: u8 = 77;

pub trait Decode<T> {
    fn decode(self) -> T;
}

impl Decode<char> for u8 {
    fn decode(self) -> char {
        char::from(self ^ SQUAWK)
    }
}

impl<I: IntoIterator<Item = u8>> Decode<String> for I {
    fn decode(self) -> String {
        self.into_iter().map(|x| x.decode()).collect()
    }
}

fn main() {
    let input = include_str!("input.txt")
        .split(',')
        .map(|x| x.trim().parse::<u8>().unwrap());
    println!("{}", input.decode());
}
