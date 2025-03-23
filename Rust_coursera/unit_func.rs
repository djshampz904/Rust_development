fn main() {

    process_num(&[1, 3, 4]);
}

fn process_num (num: &[i32]) {

    let mut sum = 0;

    for i in num {
        sum += i
    }

    println!("Sum {}", sum);

    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }

}
