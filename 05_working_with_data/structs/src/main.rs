struct Box {
    width: u32,
    height: u32,
    depth: u32,
}

fn main() {
    let my_box = Box {
        width: 10,
        height: 20,
        depth: 30,
    };
    let tall = my_box.height;
    println!("My box is {:?}", tall);
}
