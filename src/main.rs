struct Rectangle {
    x: i32,
    y: i32,
    height: u32,
    width: u32,
}
trait Area {
    fn area(&self) -> u32;
}

impl Rectangle {
    fn new(height: u32, width: u32) -> Rectangle {
        Rectangle {
            x: 0,
            y: 0,
            height,
            width,
        }
    }
    fn square(size: u32) -> Rectangle {
        Rectangle::new(size, size)
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height >= other.height && self.width >= other.width
    }
    fn contains(&self, other: &Rectangle) -> bool {
        self.x <= other.x
            && self.y <= other.y
            && (self.x as i32 + self.width as i32) >= (other.x as i32 + other.width as i32)
            && (self.y as i32 + self.height as i32) >= (other.y as i32 + other.height as i32)
    }
    fn get_coords(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    fn set_coords(&mut self, coords: (i32, i32)) {
        self.x = coords.0;
        self.y = coords.1;
    }
}

impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}
fn main() {
    let mut rect1 = Rectangle::new(20, 20);
    // get coords
    let (mut x, mut y) = rect1.get_coords();
    // update coords
    x += -20;
    y += -20;
    rect1.set_coords((x, y));
    let (x, y) = rect1.get_coords();
    dbg!(x, y);
    // create a square
    let square = Rectangle::square(10);
    // can rect1 hold square?
    let can_hold = rect1.can_hold(&square);
    assert_eq!(can_hold, true, "rect1 can hold square");
    // does rect1 contain square?
    let contains = rect1.contains(&square);
    assert_eq!(contains, true, "rect1 contains square");
}
