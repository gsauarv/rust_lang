// shadowing is the way to change the variable's value.

fn main()
{
    let x = 5;
    let x = x+1;
    let x = x*2;
    println!("The final value of x is {}",x);
    // this print statement should print the value of x is 12
    // 5+1=6
    // 6*2=12

}