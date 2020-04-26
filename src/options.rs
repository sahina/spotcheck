#[derive(Debug)]
pub struct Arguments {
    pub flag: String,
    pub url: String,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &str> {
        return parse(args);
    }
}

fn parse(args: &[String]) -> Result<Arguments, &str> {
    if args.len() == 2 {
        return if args[1] == "--help" {
            Ok(Arguments {
                flag: "--help".to_string(),
                url: "".to_string(),
            })
        } else {
            Err("unknown flag")
        };
    }

    // spotcheck --url https://www.amazon.com/gp/buy/shipoptionselect/handlers/display.html?hasWorkingJavascript=1
    if args.len() == 3 && args[1] == "--url" {
        return Ok(Arguments {
            flag: args[1].clone(),
            url: args[2].clone(),
        });
    }
    return Err("unknown args...");
}
