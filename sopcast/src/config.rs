pub struct Config {
    channel: u32
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Missing channel argument.");
        } else if args.len() > 2 {
            return Err("Too many arguments.");
        }
        let channel = args[1].parse::<u32>();
        let invalid_input_err_msg = "Invalid argument: A SopCast channel is a 6-digit number.";
        match channel {
            Ok(channel) => {
                if channel >= 100000 && channel <= 999999 {
                    Ok(Config { channel })
                } else {
                    Err(invalid_input_err_msg)
                }
            }
            Err(e) => Err(invalid_input_err_msg)
        }
    }
}
