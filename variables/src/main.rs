fn main() {

// let number = 11;

// if number < 10 {
//     println!("first condition was true");
// } else if number < 22 {
//     println!("second condition was true");
// } else {
//     println!("condition was false");
// }


// let condition = true;
// let number = if condition {5} else {6};

// println!("number: {}", number);
// let mut counter = 0;

// let result = loop {
//     counter += 1;
//     println!("Counter: {}", counter);
//     if counter == 10 {
//         break counter;
//     }
// };


// println!("final result: {}", result);


//While loops
// let mut number = 3;

// while number != 0 {
//     println!("{}!", number);

//     number -= 1;
// }
// println!("LIFTOFF!");

// For end loop

let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}

for number in 1..4 {
    println!("{}", number);
}





}

fn my_function(x: i32, y:i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y

}