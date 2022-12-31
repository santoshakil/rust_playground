fn main() {
    // Scalar types
    let x: i32 = 5; // 32-bit signed integer
    let y: f32 = 3.14; // 32-bit floating point
    let z: bool = true; // Boolean
    let c: char = 'a'; // Unicode character

    // Compound types
    let tuple: (i32, f32, char) = (5, 3.14, 'a');
    let array: [i32; 3] = [1, 2, 3];
    let slice: &[i32] = &[1, 2, 3];
    let string: String = "hello".to_string();

    let person: Person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    // let shape: Shape = Shape::Circle { radius: 2.0 };

    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
    println!("c is {}", c);

    println!("tuple is {:?}", tuple);
    println!("array is {:?}", array);
    println!("slice is {:?}", slice);
    println!("string is {}", string);

    println!("{} is {} years old", person.name, person.age);
    // println!("shape is {:?}", shape);
}

struct Person {
    name: String,
    age: u8,
}

// enum Shape {
//     Circle { radius: f32 },
//     Rectangle { width: f32, height: f32 },
// }
