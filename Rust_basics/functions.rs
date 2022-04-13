fn main() {
    println!("Hello, world!");

    another_function();
    print_labeled_measurement(5, 'h');
}

fn another_function() {
    println!("Another function.");
}

//functions with parameters and muliple parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is : {}{}", value, unit_label);
}
 

