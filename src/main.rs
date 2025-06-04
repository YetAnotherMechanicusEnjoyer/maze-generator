mod generator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generator::generate()
}
