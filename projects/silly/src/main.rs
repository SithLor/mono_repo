

struct MyPlugin;
impl Drop for MyPlugin {
    fn drop(&mut self) {
        println!("MyPlugin drop");
    }
}

fn main() {
    let _plugin = MyPlugin;
    println!("Hello, world!");
}