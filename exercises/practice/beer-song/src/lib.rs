pub fn bottle_str(n: u32)  -> String{
    match n {
        0 => String::from("no more bottles"),
        1 => String::from("1 bottle"),
        _ => format!("{} bottles", n)
    }
}
pub fn verse(n: u32) -> String {
    // unimplemented!("emit verse {}", n)
    if n==0 {
        return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    if n==1 {
        return format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }
    format!("{} of beer on the wall, {} of beer.\nTake one down and pass it around, {} of beer on the wall.\n", bottle_str(n), bottle_str(n), bottle_str(n-1))
}

pub fn sing(start: u32, end: u32) -> String {
    // unimplemented!("sing verses {} to {}, inclusive", start, end)
    let mut song = String::new();
    for i in (end..=start).rev() {
        if i > end {
            song.push_str(verse(i).as_str());
            song.push_str("\n");
        }
        if  i==end {
            song.push_str(verse(i).as_str());
        }
    }
    return song;
}
