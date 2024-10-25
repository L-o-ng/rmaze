pub struct Config {
    pub width: String,
    pub height: String,

}
impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Missing Arguements!");  
        }
        if let Err(_) = args[0].parse::<i32>() {
            return Err("Width must be an integer!");
        }
        if let Err(_) = args[1].parse::<i32>() {
            return Err("Height must be an integer!");
        }

        let width = args[0].clone();
        let height = args[1].clone();

        Ok(Self{width, height})
    }
}