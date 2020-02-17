fn print_day_of_christmas(day: u32) {
    let converted_day = match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };

    println!("On the {} day of Christmas,", converted_day);
    println!("My true love gave to me:");
}

fn print_empty_line() {
    println!("");
}

fn print_closing_line(day: u32) {
    println!(
        "{} patridge in a pear tree!",
        if day == 1 { "A" } else { "And a" }
    );
}

fn main() {
    print_empty_line();
    for day in 1..13 {
        print_day_of_christmas(day);
        print_closing_line(day);
        print_empty_line();
    }
}
