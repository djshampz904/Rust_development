#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    } 

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 23;
    let width2 = 32;
    let rect = (32, 23);

    let area = get_area(width1, width2);
    println!("{area}");

    let area2 = get_area_dim(rect);
    println!("Area tuples {area2}");

    let rect1 = Rectangle {
        width: 32,
        height: 84,
    };

    let rect2 = Rectangle {
        width: 12,
        height: 11,
    };

    let rect3 = Rectangle {
        width: 200,
        height: 400,
    };

    let rect4 = Rectangle::square(4);


    println!("Square functions {:?} :", rect4);
    println!("Rect struct area {}", get_struct_area(&rect1));
    println!("rect1 : {rect1:#?}");

    println!("Area via method: {}", rect1.area());

    println!("Rect1 one can hold rect2: {} ", rect1.can_hold(&rect2));
    println!("Rect1 one can hold rect3: {} ", rect1.can_hold(&rect3));

}

fn get_area(width1: u64, width2: u64) -> u64 {
    width1 * width2
}

fn get_area_dim(rect: (u32, u32)) -> u32 {

    rect.0 * rect.1

}

fn get_struct_area(rect: &Rectangle) -> u32 {

    rect.width * rect.height
        
}



