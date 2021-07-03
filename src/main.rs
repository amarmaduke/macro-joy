
#[macro_use]
mod joy;


joy_define! { two_add,
    add add
}

fn main() {
    let x : i64 = 6;
    let val1 = joy_eval!{ 5 };
    let val2 = joy_eval!{ add };
    let val3 = joy_eval!{ [ add ] };
    let val4 = joy_eval!{ [1 2 3 5] };
    let val5 = joy_eval!{ 1 2 3 4 5 };
    let val6 = joy_eval!{ [1 2 3] add add [cmp add 9] 7 };
    let val7 = joy_eval!{ 2 3 two_add };
    let val8 = joy_eval!{ (-5 + 6) (-6) };
    let val9 = joy_eval!{ (x) };
    println!("Hello, world!");
}
