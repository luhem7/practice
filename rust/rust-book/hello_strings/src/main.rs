fn greet(name_str: &str) -> String{
    let mut greeting = String::from("Salutations ");
    greeting.push_str(name_str);
    return greeting;
}

fn main() {
    let str1 = "Hello, wurld!"; 
    println!("{}", str1);

    let name : &str= "Bob";
    println!("{}", greet(name));
}
