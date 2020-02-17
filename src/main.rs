fn print_empty_line() {
    println!("");
}

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

fn print_gift(day: u32) {
    let gift = match day {
        2 => "Two turtle doves,",
        3 => "Three French hens,",
        4 => "Four calling birds,",
        5 => "Five golden rings,",
        6 => "Six geese laying,",
        7 => "Seven swans a swimming,",
        8 => "Eight maids a milking,",
        9 => "Nine ladies dancing",
        10 => "Ten lords a leaping",
        11 => "Eleven pipers piping ",
        12 => "Twelve drummers drumming",
        _ => return,
    };

    println!("{}", gift);
}

fn print_closing_line(day: u32) {
    println!(
        "{} patridge in a pear tree!",
        if day == 1 { "A" } else { "And a" }
    );
}

fn main() {
    print_empty_line();
    for day in 1..=12 {
        print_day_of_christmas(day);
        for gift_day in (1..=day).rev() {
            print_gift(gift_day);
        }
        print_closing_line(day);
        print_empty_line();
    }
}
