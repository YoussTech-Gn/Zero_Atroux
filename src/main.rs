// #[derive(Debug)]
// enum PaymentMethod {
//     Cash,
//     CreditCard(String),
//     BankTransfer(u32),
// }

// fn main() {
//     let mut pay1 = PaymentMethod::Cash;
//     pay1 = PaymentMethod::CreditCard(String::from("1234-5678-9012-3456"));
//     let pay2 = PaymentMethod::CreditCard(String::from("4111-2222-3333-4444"));
//     let pay3 = PaymentMethod::BankTransfer(99887766);
    
//     // println!("{:?}, {:?}, {:?}", pay1, pay2, pay3);

//     match pay1 {
//         PaymentMethod::Cash => println!("Paid in cash"),
//         PaymentMethod::CreditCard(card_number) => println!("Paid with credit card: {}", card_number),
//         PaymentMethod::BankTransfer(account_number) => println!("Paid with bank transfer: {}", account_number),
//     }
// }


// enum ApiResponse {
//     Loading,
//     Success(String),
//     Error(String),
// }

// fn main() {
//     let response = ApiResponse::Error(String::from("404 Not Found"));

//     match response {
//         ApiResponse::Loading => println!("جاري التحميل..."),
//         ApiResponse::Success(data) => println!("البيانات: {}", data),
//         // اكتب سطر التفكيك والطباعة لحالة Error هنا!s
//         ApiResponse::Error(error_message) => println!("حدث خطأ: {}", error_message),
//     }
// }


