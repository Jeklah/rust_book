#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
   let rect1 = Rectangle {
       width: 30,
       height: 50,
   };

   println!(
       "the area of the rectangle is {}",
       rect1.area()
   );
   if rect1.width() {
       println!("the rectangle has a nonzero width; {}", rect1.width);
   }

   let rect2 = Rectangle {
       width: 30,
       height: 50,
   };
   let rect3 = Rectangle {
       width: 10,
       height: 40,
   };
   let rect4 = Rectangle {
       width: 60,
       height: 45,
   }; 

   println!("can rect2 hold rect3? {}", rect2.can_hold(&rect3));
   println!("can rect2 hold rect4? {}", rect2.can_hold(&rect4));
   Rectangle::square(30);
}
