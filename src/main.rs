// #[derive(Debug)]
#[allow(unused)]
// #[allow(unused_variables)]

struct Products{
    id : u32,
    name : String,
    desc : String,
    img : String,
    price : u32
}
// #[derive(Debug)]
// struct  Currency {
//     dollar : u32,
//     cfa : u32,
// }

// impl Products {
//     // fn total_price(&self,  count : u32) -> u32{
//     //     self.price * count
//     // }
//     // fn total_price_in_cfa(dollar : u32) -> u32 {
//     //     dollar * 500
//     // }

//     fn total_price(&self ,count : u32) -> Currency{
//         Currency { dollar: self.price * count , cfa: (self.price * count) * 500  }
//     }
//     fn add_discount(&mut self , discount : u32){
//         self.price -= discount;
//     }
// }

// fn main(){
//     let mut x = Products {
//         id : 1,
//         desc :String::from("Some Describtion for this Product"),
//         img : String::from("Kalou"),
//         name : String::from("Kardou"),
//         price : 300
//     };
//     println!("our new Product : {:#?} " , x);
//     let Products {id , ref name  , ref desc ,ref  img , price} = x;
//     // println!("our new Product : {:#?} " , {name , desc , id , img})
//     println!("name : {} , id : {} , desc : {} , img : {}" , name  ,id , x.desc , x.img);
//     println!("our new Product : {:#?} " , x);
//     let total : Currency = x.total_price(10);
//     // println!("the total : [{}]$  | cfa [{}] " , total , Products::total_price_in_cfa(total));
//     println!("the total : [{}]$  | cfa [{}] " , total.dollar , total.cfa);
//     println!("function return a struct : {:#?} " , return_struct());
//     x.add_discount(1000);
//     println!("product after discount : {:#?} " , x );
//     // println!("product after discount : {:#?} " , x.add_discount(100));


// }


// fn return_struct()-> Currency{
//     Currency { dollar: 100, cfa: 60000 }
// }


 fn main() {
     let mut text = String::from("Rust");
          let r1 = &text;
         let r2 = &text;
         println!("Read: {}, {} ", r1, r2);
  let r3 = &mut text;
    println!("Read and Write: {}",  r3);
 }

// struct Player {
//     name: String,
//     score: u32,
// }

// impl Player {
//     fn display_score(&self) {
//         println!("Score: {}", self.score);
//     }
// }

// fn main() {
//     let p = Player { name: String::from("Youssouf"), score: 100 };
//     let Player { ref name, ref  score } = p;
//     p.display_score();
// }

// fn main(){
// let mut x = Products { id: 1, price: 300 /*...*/ };
// let my_borrow = &mut x;

// my_borrow.add_discount(50); // هنا تكمن المشكلة

// println!("{:?}", my_borrow.price);

// }