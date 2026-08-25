#[allow(unused)]
// Make it work
// use std::mem::size_of_val;
// fn main() {
//     let c1 : char = 'a';// bytes
//     assert_eq!(size_of_val(&c1),4); 

//     let c2 :char = '中'; // bytes
//     assert_eq!(size_of_val(&c2),4); 

//     println!("Success!");
// } 

// fn main()
// {
//     let chr : char = 'z';
//      let st : &str = "Hello";
//     println!("{} > {}" , st , chr);
// }

// // double quotion for string 
// // single is for characters


// // Make it work
// fn main() {
//     let c1 = '中';
//     print_char(c1);
// } 

// fn print_char(c : char) {
//     println!("{}", c);
// }

// Make println! work
// fn main() {
//     let f: bool = true;

//     let t : bool = true;
//     if !!t {
//         println!("Success!");
//     }
// } 


// // Make it work
// fn main() {
//     let f = true;
//     let t : bool = false && true;
//    assert_ne!(!f , t, "we are testing that the values are not equal");
//     assert_eq!(t, f);
//     println!("{}"  , t);

//     println!("Success!");
// }

// // Make it work, don't modify `implicitly_ret_unit` !
// fn main() {
//     let _v: () = ();

//     let v  = ();
//     assert_eq!(v, implicitly_ret_unit());

//     println!("Success!");
// }

// fn implicitly_ret_unit() {
//     println!("I will return a ()");
// }

// // Don't use this one
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }

// Modify `4` in assert to make it work
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);

//     println!("Success!");
// }