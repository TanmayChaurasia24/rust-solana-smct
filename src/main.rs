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

    // ownership_example();
    // reference_and_derefrence();
    arrays_example();
}

fn arrays_example() {
    // let arr: [&str;3] = ["hello","world","chase"];
    // write_arr(arr); // when we have passed arr directly then arr1 is thw new copy of arr thats why after changes to arr1 arr is still same
    let mut arr: [&str;3] = ["hello","world","chase"];
    write_arr(&mut arr); // passing the reference of arr now changes to arr1 will be shown to arr also
    println!("arr = {:?}", arr);
}

fn write_arr(arr1:&mut [&str;3]) {
    arr1[0] = "fellow";
    println!("arr1 = {:?}", arr1)
}

fn reference_and_derefrence() {
    let mut x: i8 = 5;
    x = x+1; //6
    let y: &mut i8 = &mut x; // y is the reference to the value of x, y is storing the address of x *y will access the value of x.
    *y += 1; //7 ------ derefrencing 
    println!("x = {}", y);

}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn ownership_example() {
    let a: i8 = 1;
    let b: i8 = a;

    println!("a: {}",a);
    println!("b: {}",b);

    // this code will give error because ownership rules are getting violated
    let str1:String = String::from("tanmay"); // str1 is the owner of tanmay
    let str2:String = str1; // we are transfering the ownership of str1 to str2, in rust when we transfer the ownership of one variable to another then the fist owner 
    // println!("str1: {}", str1);
    println!("str2: {}", str2);

    let x:String = String::from("tanmay kumar chaurasia"); // x is the pointer and is the owner of tanmay kumar chaurasia
    process_string(x); // trasfering the owner ship to s and x is now not the owner of tanmay kumar chaurasia
    // println!("the value of x is {}", x); // this will give error as x is now not the owner of anything string

    // example for transfering onwer ship and returning many things from function
    let name: String = String::from("tanmay");
    // let (s2,length) = cal_length(name); // this is transfering the ownership of the name

    // clone method is used for deep copy of the heap data. it is an expensive method
    // let length: usize = cal_length(name.clone()); // we are not transfering the ownership of name, instead we are just passing the clone of name, so that we are also access name

    //-----------borrowing---------------
    let length: usize = cal_length(&name);
    println!("the length of {} is {}",name,length);
}

fn cal_length(s: &String) -> usize{
    let length: usize = s.len();
    return length;
}

fn process_string(s: String) {
    println!("processing string: {}", s); // this will print tanmay kumar chaurasia
}