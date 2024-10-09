fn main() {
    christmas_carol_lyrics()
}

fn christmas_carol_lyrics() {
    const NTH_DAY_WORD: [&str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    const STRING_PER_DAY: [&str; 12] = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five golden rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas", NTH_DAY_WORD[i]);
        println!("my true love gave to me");

        for j in (0..i + 1).rev() {
            println!("{}", STRING_PER_DAY[j]);
        }

        println!();
    }
}
