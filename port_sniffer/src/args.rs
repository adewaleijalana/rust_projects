use std::net::IpAddr;

#[derive(Debug)]
pub struct Args {
    flag: String,
    ip: IpAddr,
    threads: u16,
}

impl Args {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        println!("args: {args:?}");
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        if args.len() > 4 {
            return Err("too many arguments");
        }

        let flag = args[1].clone();
        println!("flag: {flag}; length: {}", args.len());

        if let Ok(ip) = args[1].parse::<IpAddr>() {
            Ok(Args {
                flag: String::from(""),
                ip,
                threads: 4,
            })
        } else {
            let flag = args[1].clone();
            if (flag.contains("-h") || flag.contains("-help")) && args.len() == 2 {
                println!(
                    "Usage: j to specify the number of threads (default is 4)
                    \r\n -h or -help to display this help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") && args.len() == 4 {
                let ip = match args[3].parse::<IpAddr>() {
                    Ok(ip) => ip,
                    Err(_) => return Err("invalid IP address"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(n) if n > 0 => n,
                    _ => return Err("invalid number of threads"),
                };
                Ok(Args { flag, ip, threads })
            } else {
                return Err("invalid flag");
            }
        }
    }

    pub fn threads(&self) -> u16 {
        self.threads
    }

    pub fn ip(&self) -> IpAddr {
        self.ip
    }
}
