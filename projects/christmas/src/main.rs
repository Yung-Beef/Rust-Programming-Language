fn main() {
    let mut g = ["A partridge in a pear tree.", "Two turtle doves,", "Three French hens,", "Four calling birds,", "Five golden rings,", "Six geese a-laying,", 
                        "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];

    let d = ["first", "second", "third", "fourth", "fifth", "sixth", 
            "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    for i in 0..=11 {
        print!("On the ");
        print!("{}", d[i]);
        println!(" day of Christmas,\nmy true love gave to me");
        for j in (0..=i).rev() {
            println!("{}", g[j]);
        }
        print!("\n");
        g[0] = "And a partridge in a pear tree.";
    }
}
