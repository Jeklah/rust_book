fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}",s);

    let s3 = String::from("hello ");
    let s4 = String::from("world!");
    let s5 = s3 + &s4; // note s3 has been moved here and can no longer be used.
    println!("{}", s5);

    let string1 = String::from("tic");
    let string2 = String::from("tac");
    let string3 = String::from("toe");

    // let ttt = string1 + "-" + &string2 + "-" + &string3;   // looks shit
    let ttt = format!("{}-{}-{}", string1, string2, string3); // much better using format macro.
              // format macro returns a String of whatever it was given. doesn't take ownership.
    let st1 = String::from("hello");
    // let h = st1[0];
    println!("{} {}", st1, ttt);
    println!("{st1} {ttt}");
}
