#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
