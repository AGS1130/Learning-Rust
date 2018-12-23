fn main() {
    // `let` by default is immutable
    // adding the `mut` expression allows the variable x to be mutable
    // HOWEVER, the data type must be the same as the declared variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // There is a paradigm called `shadowing`
    // `Shadowing` allows you to decalre a variable of the same name
    // that is declared earlier in the thread to have a different value
    // of any data type

    let y = "string";
    println!("The value of y is a: {}", y);

    let y = 5;
    println!("The value of y is a: {}", y);

    // The primary difference in `shadowing`
    // has to do with how the variable is lexically scoped
    // ``` The outer scope will not be mutated by the inner scope ```

    // const is always immutable, you cannot make it mutable
    const MAX_POINTS: u32 = 100_000;

    println!("MAX_POINTS is: {}", MAX_POINTS);
}
