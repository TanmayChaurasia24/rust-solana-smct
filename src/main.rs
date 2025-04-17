fn main() {
    let num: u16 = 20;
    println!("value of num is: {}", num);
    // num = 199 // this will give an error because num is inmutable

    let mut age: u32 = 22; // changed to mutable and value can be changed in future
    println!("value of age is: {}", age); // if we will not use the value of age before changing it then rust will give warning
    age = 32;
    println!("value of age is: {}", age);

    // --------------------STRINGS--------------------------------
    // string are of 2 types
    // 1: &str - stored in some part of memory but not in heap
    // 2: String - heap allocated

    let name: &str = "my name is tanmay"; // fixed size string
    println!("{}", name);

    let mut full_name: String = String::from("my name is tanmay "); // dynamic string 
    full_name.push_str("kumar chaurasia"); // this cant be done with &str type string
    println!("{}", full_name);

    // -------------------------------TUPLES----------------------
    let emp_info: (&str, u8) = ("Tanmay", 22);
    let emp_name: &str = emp_info.0;
    let emp_age: u8 = emp_info.1;

    // destructuring
    let (em_name, em_age) = emp_info;

    println!("age of {} is {}", emp_name, emp_age);
    println!(
        "age of {} is {}, extracted by destructuring",
        em_name, em_age
    );

    // --------------------functions---------------------
    let result: i32 = add(10, 20);
    println!("result is: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
