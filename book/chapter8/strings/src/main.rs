fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string(); // creates a String
    // also works on a string literal directly
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");

    let mut s = String::from("foo");
    //s.push_str("bar"); //push_str takes a string slice so it doesn't take ownership
    let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}"); // can still use s2 because push_str doesn't take ownership

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // s1 has moved here and can no longer be used
    // + calls an add function that takes (self, s: &str), so the one on the left of + will be moved and no longer useable

    // + gets unwieldy if you need to concatenate multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    //let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}"); // format uses references so it doesn't take ownership of anything
    //let h = s[0]; // will create an error

    for c in "Зд".chars() { // will print 2 chars
        println!("{c}");
    }

    for b in "Зд".bytes() { // will print 4 bytes, because each of these cyrillic chars is 2 bytes in UTF-8
        println!("{b}");
    }
    


}
