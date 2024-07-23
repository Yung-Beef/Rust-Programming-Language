use std::collections::HashMap;

fn main() {
    
    let mut dict = HashMap::new();
    dict.insert("1", "A partridge in a pear tree.");
    dict.insert("2", "Two turtle doves,");
    dict.insert("3", "Three French hens,");
    dict.insert("4", "Four calling birds,");
    dict.insert("5", "Five golden rings,");
    dict.insert("6", "Six geese a-laying,");
    dict.insert("7", "Seven swans a-swimming,");
    dict.insert("8", "Eight maids a-milking,");
    dict.insert("9", "Nine ladies dancing,");
    dict.insert("10", "Ten lords a-leaping,");
    dict.insert("11", "Eleven pipers piping,");
    dict.insert("12", "Twelve drummers drumming,");

    let d = ["first", "second", "third", "fourth", "fifth", "sixth", 
            "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    for i in 1..=12 {
        println!("On the ");
        println!("{}", d[i]);
        println!(" day of Christmas,\nmy true love gave to me");
        let phrase = match dict.get(&i) {
            Some(p) => p,
            None => panic!("Error")
        };
        println!("{}", phrase);

        dict.insert("1", "And a partridge in a pear tree.");
    }

}
