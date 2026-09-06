// // use String::from;

// struct User {
//     name : String,
//     email : String,
//     active: bool
// }
// struct Color(i32 , i32 , u8);
// struct Point(i32 , i32 , ());
// fn main(){
//     let mut new = build_new_user(String::from("h@proton.me"), String::from("Saliou"));
//     println!("we have new his called [{}] his email Adress : [{}] and his connected right now! {}" , new.name , new.email , new.active );
//     println!("we have new his called [{}] his email Adress : [{}] and his connected right now! {}" , new.name , new.email , new.active );
//     let  mut same_user = &mut new;
//     same_user.email = String::from("Mouhammed@pront.me");
//     println!("we have new his called [{}] his email Adress : [{}] and his connected right now! {}" , new.name , new.email , new.active );
    

//     // ----------
//     let black = Color(21 , 5, 0);
//     let origin : Point = Point(8 , 10,  ());
// }

// fn build_new_user(email : String , name : String) -> User{
//     User {
//         active : true,
//         email,
//         name
//     }
// }

#[derive(Debug)]

struct  Reactangle {
    width : i32,
    height : i32,
}

fn main(){
    // let w = 23;
    // let h = 23;
    // let rect : (i32 , i32)= (40  , 20);
    let rect : Reactangle = Reactangle { width: 20, height: 30 };
    // let x = rect;
    println!("Our Struct is {:#?}" , rect);
    println!("The area of the rectangle is {} square pixels." , area(&rect));
    eprintln!("Erro Gys");
    // println("jf");
    println!();
    println!();
    dbg!(&rect);
}



fn area(dimension : &Reactangle) -> i32 {
    dimension.width * dimension.height
}