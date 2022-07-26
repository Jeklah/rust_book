#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "the area of the rectangle is {} or {} sq pixels",
        area(width1, height1),
        area_2((height1, width1))
    );

    println!(
        "the area of the rectangle is {} sq pixels",
        rect_area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);
    
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

// the area function is supposed to calculate the area of one rectangle, but the function
// has two parameters and it's not clear anywhere that the parameters are related.
// It would be more readable to group width and height together. We can do that with
// a struct tuple.

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rect_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
