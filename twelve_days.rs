fn main() {
    twelve_days()
}

fn twelve_days() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let sentences = [
        "A song and a Christmas tree",
        "Two candy canes
    And a song for the Christmas tree",
        "Three boughs of holly
    Two candy canes
    And a song for the Christmas tree",
        "Four colored lights
    Three boughs of holly
    Two candy canes
    And a song for the Christmas tree",
        "A shining star
    Four colored lights
    Three boughs of holly
    Two candy canes
    And a song for the Christmas tree",
        "Little silver bells
    A shining star
    Four colored lights
    Three boughs of holly
    Two candy canes
    And a song for the Christmas tree",
        "Candles a-glowing
    Little silver bells
    A shining star
    Four colored lights
    Three boughs of holly
    Two candy canes
    And a song for the Christmas tree",
        "Gold and silver tinsel
    Candles a-glowing
    Little silver bells
    A shining star
    Four colored lights
    Three boughs of holly
    Two candy canes
    And a song for the Christmas tree",
        "A guardian angel
    Gold and silver tinsel
    Candles a-glowing \n Little silver bells
    A shining star
    Four colored lights
    Three boughs of holly
    Two candy canes
    And a song for the Christmas tree",
        "Some mistletoe
    A guardian angel
    Gold and silver tinsel
    Candles a-glowing
    \n Little silver bells
    A shining star
    Four colored lights
    Three boughs of holly
    Two candy canes
    And a song for the Christmas tree",
        "Gifts for one and all
    Some mistletoe
    A guardian angel
    Gold and silver tinsel
    Candles a-glowing \n Little silver bells
    A shining star
    Four colored lights
    Three boughs of holly
    Two candy canes
    And a song for the Christmas tree",
        "All their good wishes
    Gifts for one and all
    Some mistletoe
    A guardian angel
    Gold and silver tinsel
    Candles a-glowing
    Little silver bells
    A shining star
    Four colored lights
    Three boughs of holly
    Two candy canes
    And a song for the Christmas tree",
    ];

    for (index, day) in days.iter().enumerate() {
        println!("On the {day} day of Christmas My good friends brought to me \n");
        println!("{}", sentences[index]);
    }
}
