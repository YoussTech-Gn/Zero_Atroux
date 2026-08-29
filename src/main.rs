//          Rust has three kinds of loops : Loop , while , for



// fn main()
// {
//     let mut count = 0;
//     's : loop {
//         println!(" >> count is = {}" , count);
        
//         let mut rem = 10;
//         loop {
//             println!(" >> rem is = {}" , rem);
//             if rem == 0 {
//                 println!("rem has reached 0 = {} and count is {} and the main loop will now continue" , rem , count);
//                 break ;
//             }
//             if count == 2{
//                 println!("count has reached 2 = {}" , count);
//                 break 's;
//             }
//             rem -= 1;
//         }
//         count +=1;
//     }

//     println!("End Count = {}" , count)
// }



// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
        
//     }
//     println!("-----------------------------------------");
//     while number < 20 {
//         println!("{number}!");
//         number += 1;
//     }

//     println!("LIFTOFF!!! \n num : {}" , number );
// }

// fn main() {
//     let a = ['a' , 'b' , 'c' , 'd' , 'f' , 'h' , 'i' , 'j' , 'k' , 'l' , 'm' , 'n' , 'o'];
//     let mut index = 0;

//     while index < a.len() {
//         println!("the value is: [{}] and length of a collection wich are rest to see {}", a[index] , a.len() - index);

//         index += 1;
//     }
//     for element in a {
// println!("the value is: {element}");
// }
// }

fn main(){
    for number in (1..=20).rev(){
        println!("num : {}" , number)
    }
}

