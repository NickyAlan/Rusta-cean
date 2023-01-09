#[allow(dead_code)]
enum WEEKEND {
    SATURDAY,
    SUNDAY,
}

fn main() {
    // simple loop
    let mut number = 0;
    loop {
        if number == 3 {
            println!("--skip this--");
            number += 1;
            continue;
        }
        if number > 5 {
            println!("[INFO] Finished");
            break;
        }
        println!("number {}", number);
        number += 1;
    }

    // switch case
    let text = String::from("Sample text");
    match text.as_str() {
        "word" => println!("[INFO] Matched {}", text),
        "Sample text" => println!("[INFO] Matched {}", text),
        _ => println!("[INFO] Not matched")
    }

    let weekend = WEEKEND::SUNDAY;
    match weekend {
        WEEKEND::SUNDAY => {
            println!("SUNDAY");
        }
        WEEKEND::SATURDAY => {
            println!("SATURDAY");
        }
        _ => {
            println!("[INFO] Not matched");
        }
    }

    let day = 3;
    match  day {
        1 ..= 5 => println!("WEEKDAY"),
        6| 7 => println!("WEEKEND"),
        _ => println!("Invalid day")
    }

    // tupple multi type variable
    let person: (u32, &str, bool) = (1, "John", true);
    println!("{:?}", person); // :#? like jason print
    println!("{}", person.1); // indexing

    // array single type variable
    let array1: [&str; 3] = ["John", "Jame", "Macc"];
    for index in 0..array1.len() {
        println!("{}: {}", index, array1[index]);
    }

    // enumerate
    for item in array1.iter().enumerate() {
        let (i, x) = item;
        println!("{}: {}", i, x);
    }

    let mut x = [0; 4]; // [0, 0, 0, ..] 4 element
    println!("{:?}", x);
    println!("{}", x[2]);
    for index in 0..x.len() {
        x[index] += 2 ;
        println!("{:?}", x);
    }

    // vector -> glowable array
    let mut v = vec![2, 3, 5];
    for number in 1..5 {
        v.push(number);
    }
    println!("{:?}",v);
    println!("{:?}", &v[1..4]); // slicing

    let mut v: Vec<i32> = vec![];
    for number in 1..6 {
        v.push(number);
    }
    println!(">> {:?}", v);

    let mut vec: Vec<i32> = vec![];
    let mut sum: i32 = 0;
    for num in 0..11 {
        vec.push(num);
        sum += num;
    } 
    println!("vec: {:?}", vec);
    println!("sum: {}", sum);

    // input field
    let mut input_text = String::new();
    println!("Enter your name : ");
    let text = std::io::stdin().read_line(&mut input_text).unwrap(); // this line use for add input for input_text
    println!("Hello {}", input_text);
 
}
