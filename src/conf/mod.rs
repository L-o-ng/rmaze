struct Config {
    width: String,
    height: String,

}
impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Missing Arguements!");  
        }

        let width = args[0].clone();
        let height = args[1].clone();

        Ok(Self{width, height})
    }
}