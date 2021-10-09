fn main()
{
    // loop,while,for
    
    // eg of loop
    let mut counter : u32 = 0;
    let result = loop{
        counter+=1;
        if counter ==10
        {
            break counter*2;
        }
    };

    println!("The result of counter = {}",result);

    // while loop

    while counter!=0
    {
        println!("{}!",counter);
        counter -=1;
    }
    println!("Counter stops");


    // for loops
    let arr:[u32;5]=[10,20,30,40,50];
    for element in arr.iter()
    {
        println!("the values are: {}",element);
    }

    


}