fn main() {
let x: i32 = 5;
println!("The value of x is:  {} ", x); 
let x = "six";   
println!("The value of x is:  {} ", x); 

const SUBSCRIBER_COUNT: u32 = 100_000;

//Integers
//Floating-point Integers
//Booleans
//Char



//array
let error_codes = [404, 500, 503];
let not_found = error_codes[0];
// let x = error_codes[3]; 

let tup = ("Hello this is a test", 100_000);
let sub_count = tup.1;
let (text, number) = tup; 


println!("tup {:?}", tup);
}