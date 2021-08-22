fn main() {
    var_decl();
}


fn var_decl(){
    let x ;
    x = 42;

    let x = 42;

    let x: i32;

    let x: i32 = 42;

    let x;
    // println!("{0}", x);
    x = 42;
}


// fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
//     if rhs == 0 {
//         return false;
//     }

//     lhs % rhs == 0
// }

// fn fizzbuzz(n: u32) -> () {
//     if is_divisible_by(n, 15) {
//         println!("fizzbuzz");
//     } else if is_divisible_by(n, 3) {
//         println!("fizz");
//     } else if is_divisible_by(n, 5) {
//         println!("buzz");
//     } else {
//         println!("{}", n);
//     }
// }

// fn fizzbuzz_to(n: u32) {
//     for n in 1..n + 1 {
//         fizzbuzz(n);
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

