// fn main() {
//     let x = 5u32;

//     let y : u32 = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;

//         // This expression will be assigned to `y`
//         x_cube + x_squared + x //155
//     };

//     let z : u32 = {
//         // The semicolon suppresses this expression and `()` is assigned to `z`
//         2 * x
//     };

//     println!("x  will retutn {:?}", x);
//     println!("y  will retutn {:?}", y);
//     println!("z  will retutn {:?}", z);
// }

// // Make it work with two ways
// fn main() {
//    let v = {
//        let mut x = 1;
//        x + 2
//    };
//    println!("{}" , v);

//    assert_eq!(v, 3);

//    println!("Success!");
// }


// fn main() {
//    let v = 3;
//    assert!(v == 3);

//    println!("Success!");
// }


fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}