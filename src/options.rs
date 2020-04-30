#[derive(Debug)]
pub struct Args {
    pub flag: String,
    pub value: String,
}

impl Args {
    pub fn new(args: &[String]) -> Result<Args, &str> {
        return parse(args);
    }
}

fn parse(args: &[String]) -> Result<Args, &str> {
    if args.len() == 2 {
        return Ok(Args {
            flag: "--asin".to_string(),
            value: args[1].clone(),
        });
    }

    // spotcheck --asin B00C7C67O2
    // spotcheck B00C7C67O2
    if args.len() == 3 && args[1] == "--asin" {
        return Ok(Args {
            flag: args[1].clone(),
            value: args[2].clone(),
        });
    }
    return Err("unknown args...");
}