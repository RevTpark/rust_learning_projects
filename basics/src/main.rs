fn main() {
    // Basic Print 
    println!("0) {} + {} = {}", 1, 1, 2);
    println!("{0} + {0} = {1}", 1, 2);
    println!("{num1} + {num2} = {result}", num1=1, num2=1, result=2);

    // Variables
    // name: type and all variable are immutable by default
    let new_variable: f32 = 0.0;
    let _unused_variable: f32 = 15.5;
    const NEW_CONSTANT: &str = "This is constant";
    static NEW_STATIC: u8 = 0;
    println!("1) {} {} {}", new_variable, NEW_CONSTANT, NEW_STATIC);

    // Functions
    new_function();
    function_without_return();
    println!("4) {}", parameter_function(10, 12));

    // Arrays and String
    let new_string: String = String::from("The quick brown fox");
    let new_str: &str = &new_string[4..9]; // Splicing
    println!("5) {}", new_str);

    let new_array: [usize; 5] = [1,2,3,4,5];
    let new_spliced_array: &[usize] = &new_array[0..3];
    println!("6) {:?}", new_spliced_array);

    // Structs similar to Classes
    let new_struct = NewStruct::new(0, 0.0, "Hey There".to_string());
    println!("7) {} {} {}", new_struct.field1, new_struct.field2, new_struct.field3);

    // Ownership in Rust(For non primitive data types)
    let first_owner: String = "Hello World".to_string();
    let _second_owner: &String = &first_owner; //Points to same memory location first stays owner.
    let _third_owner: String = first_owner; // 99 doesn't exists in first_owner anymore
    // Now first_owner cant be used as well as second_owner as the value is moved.

    // Vectors -> Dynamic Arrays 
    let mut new_vector: Vec<i32> = vec![1,2,3,4];
    println!("8) {:?}", new_vector);
    new_vector.pop();
    new_vector.push(5);
    new_vector.push(7);

    for x in new_vector.iter_mut(){
        *x *= 2;
    }
    println!("9) {:?}", new_vector);

    // Shorthand if
    let is_true = if 10 > 5 { true } else { false };
    println!("10) {}", is_true);

    // Infinite Loop
    let mut count = 0;
    loop {
        count += 1;
        println!("{}) {}", count+10, count);

        if count == 5{
            break;
        }
    }
    // rust equilavent of for i in range(0, 100) is for x in 0..100 

    // Closure (similar to lambda function)
    let num3: i32 = 15;
    let add_nums = |num1: i32, num2: i32| num1 + num2 + num3;
    println!("16) {}", add_nums(2,3));


    // Tuple Struct
    let color = TupleStruct(255, 0, 0);
    println!("17) {:?}", (color.0, color.1, color.2));

    // Enums
    let (character1, character2, character3, character4) = (Movement::Down, Movement::Left, Movement::Up, Movement::Right);
    move_character(character1);
    move_character(character2);
    move_character(character3);
    move_character(character4);
}

fn new_function() -> u8{
    println!("2) Hello I am new Function!!");
    return 0;
}

fn function_without_return() -> u8{
    println!("3) Return value without explictly mentioning");
    0
}

fn parameter_function(x: u8, y: u8) -> u8{
    x + y
}

#[derive(Debug)]
struct NewStruct{
    field1: u8,
    field2: f32,
    field3: String,
}

impl NewStruct{
    fn new(field1: u8, field2: f32, field3: String) -> Self{
        NewStruct{
            field1,
            field2,
            field3
        }
    }
}

struct TupleStruct(u8, u8, u8);

enum Movement{
    Up,
    Down, 
    Left, 
    Right
}

fn move_character(m: Movement){
    match m {
        Movement::Up => println!("Moving Up!!"),
        Movement::Down => println!("Moving Down!!"),
        Movement::Left => println!("Moving Left!!"),
        Movement::Right => println!("Moving Right!!"),
    }
}