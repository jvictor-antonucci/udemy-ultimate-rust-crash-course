#![deny(clippy::all)]

fn main() {
    {
        let name = "John".to_string();

        println!("Hello {}", name);
    }
}
