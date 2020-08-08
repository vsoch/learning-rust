pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => format!(
            "{number} bottle of beer on the wall, {number} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", number = n),
        2 => format!(
            "{number} bottles of beer on the wall, {number} bottles of beer.\nTake one down and pass it around, {next} bottle of beer on the wall.\n", number = n, next = n - 1
        ),
        _ => format!("{number} bottles of beer on the wall, {number} bottles of beer.\nTake one down and pass it around, {next} bottles of beer on the wall.\n", number = n, next = n - 1),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = vec![];
    // Go from end to start (and reverse) 6, 7, 8 ==> 8, 7, 6
    // pushed to end of a vector means we have [8, 7 ,6] (counting down)
    for i in (end..=start).rev() {
        song.push(verse(i))
    }
    song.join("\n")
}
