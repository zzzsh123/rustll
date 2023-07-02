fn greet_world() {
    let southern_germant = "Grüß Gott!";
    let chinese = "世界，你好！";
    let english = "world, hello";
    let regions = [southern_germant, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
fn main() {
    greet_world();
}
