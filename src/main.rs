fn main() {
    
    let config_port: Option<u16> = Some(8080);
// كود مزعج:
// match config_port {
//     Some(port) => println!("Server starting on port: {}", port),
//     _ =>()
// }
    // if let config_port = some(port) { wrong syntax
    if let Some(port) = config_port {
        println!("Server starting on port: {}", port);
    } else {
        println!("No port specified, using default.");
    }

// 2
    let fetch_data: Result<String, String> = Ok(String::from("JSON_PAYLOAD"));

// // كود مزعج:
// match fetch_data {
//     Ok(data) => println!("Processing data: {}", data),
//     // Err(_) => (), // لا نكترث للخطأ هنا
//     _=> (),
// }

if let Ok(res) = fetch_data {
    println!("Processing data: {}", res);
}

let mut messages = vec!["Msg1", "Msg2", "Msg3"];

// كود مزعج ومعقد:
// loop {
//     match messages.pop() {
//         Some(msg) => println!("Processing: {}", msg),
//         None => break, // عندما تفرغ القائمة، نكسر الحلقة
//     }
// }
while let Some(msg) = messages.pop() {
    println!("Processing: {}", msg);
}
// if let Some(msg) = messages.pop() {
//     println!("Processing: {}", msg);
// }
}