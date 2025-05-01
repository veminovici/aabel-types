pub fn create_printer() -> impl for<'a> Fn(&'a str) {
    |s| println!("Printing: {}", s)
}

pub fn test_create_printer() {
    let printer = create_printer();

    let string1 = "hello";
    printer(string1);

    let string2 = "world";
    printer(string2);

    {
        let short_lived = "temp";
        printer(short_lived);
    }
}

// pub fn create_printer_with_lifetime<'a>() -> impl for<'b> Fn(&'b str) -> &'b str {
//     |s| s
// }