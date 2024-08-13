fn main() {
    // let v: Vec<i32> = Vec::new(); specifies that v is a vector holding i32 data, since there is no data for Rust to infer on
    // let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let third: &i32 = &v[2]; // if you want it to crash if you try an index that's out of range
    println!("The third number is {third}");

    let third: Option<&i32> = v.get(2); // error handling for indices out of the range, .get would return the Option 'None'
    match third {
        Some(third) {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }
    }

    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let x = vec![1, 2, 3, 4];

        // do stuff with x
    } 
    // x goes out of scope and is freed here

}
