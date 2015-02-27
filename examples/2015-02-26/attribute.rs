#[allow(dead_code)] // Won't complain about the function never being called
fn unused_function() { }

#[allow(unused_variables)] // Won't complain about vairables never being used
fn main() {
    let dont_mind_me = 1;
    let great_value = 1u32;

    println!("This is a great value: {:?}", great_value);
}
