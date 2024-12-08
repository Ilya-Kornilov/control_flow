fn main() {

    'outer: loop {                      // labeling the loop to break
        println!(" > outer loop");
        loop {
            println!(" > inner one");
            break 'outer;               // label lets break the outer loop
        }
    }
    println!("Hello, world!");

        // assigning a value as a loop result
    let x = loop {break 5;};        // breaking out of the loop 
                                         // and the value to assign
        // assigning with if-statement
    let y = if x > 50 {x - 2} else {x + 2};
    println!(" x = {x} => y = {y}");

    let mut z = 0;
    while z < 5 {
        println!("\tz = {z}");
        z += 1;
    }

    // "for" loops are for collections
    let collection = [1, 2, 3, 4, 5];   // an array of int
        // "element" is a random name for each element in the collection
    for element in collection {     
        println!(" > element: {element}");
    }

    // line comment
    /* block comment
     * 1
     * 2
     * 3
     */
}
