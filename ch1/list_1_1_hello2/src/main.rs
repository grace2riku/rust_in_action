fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "GrüßGott!";
    let japan = "ハロー・ワールド";
    let resions = [southern_germany, japan];

    for region in resions.iter() {
        println!("{}", &region);
    }

}

fn main() {
    greet_world();
}
