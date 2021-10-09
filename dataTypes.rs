fn main()
{
    data types.
    // 1. scalar types
    -> int,floats,booleans,char
    let x = 2.0 ; //f64 double precision float.
    let y:f32 = 3.0; //f32 single precision float.

    // booleans

    let isKnown : bool = false;

    // char

    let awesome_text = "saurav";

    // tuples cant grow or shrink like arrays. fixed in size

    let tup : (i32,f64,u8) = (500,6.4,1);
    // ascessing elements of the tuples

    println!("Elements of tup {} {} {}",tup.0,tup.1,tup.2);

    // arrays

    let arr = [1,2,3];
    let arrs : [i32;5] = [1,2,3,4,5];

    let same_elements = [3;5];
    // same element value in the arrays
    // this array contains [3,3,3,3,3];

    println!("First element {}",arr[0]);










    // operations
    // add 
    let sum = 5+20;

    // sub 
    let diff = 20-30;

    //mul

    let product = 10*20;

    // division

    let quotient = 56.7/1.0;

    // reminder 

    let reminder = 43%10;

}