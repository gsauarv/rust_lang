fn main()
{
    let mut x = 5;
    println!("The value of x is: {}",x);
    x = 6;
    
    // this code is not the valid assignment of the variable because 
    // the variable that was assigned at the top is assigned as a immutable variable 
    // to make the variable mutable we need to add mut keyword infront of the x variable
 
     
    println!("The value of x is {}",x);

    const MAX_POINTS : u32  = 100_000;
    // to declare the consttant  we can use const keyword and the CAPITALIZATION WITH
    // The underscore is a rust naming convention the number or vallue can also be  seperated by
    // using underscore to ensure the readablility.

    println!("The value of Max point is {}" , MAX_POINTS);

    


}