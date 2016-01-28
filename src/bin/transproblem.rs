extern crate getopts;
extern crate transproblem;

use getopts::Options;
use transproblem::Transportation;
use transproblem::Error;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

// Try to read a file into a vector of strings. If successful, then try to parse them.
fn file_input(file: &PathBuf) -> Result<(Vec<u64>, Vec<u64>, Vec<Vec<u64>>), Box<std::error::Error>> {
    let f = BufReader::new(try!(File::open(file)));
    let mut lines: VecDeque<String> = try!(f.lines().collect());
    lines = lines.into_iter()
                 .filter(|line| {
                     line.trim();
                     !line.is_empty()
                 })
                 .collect();

    let a: Vec<u64> = try!(lines.pop_front()
                                .unwrap_or("".to_owned())
                                .split_whitespace()
                                .map(str::parse)
                                .collect());

    let b: Vec<u64> = try!(lines.pop_front()
                                .unwrap_or("".to_owned())
                                .split_whitespace()
                                .map(str::parse)
                                .collect());

    let c: Vec<Vec<u64>> = try!(lines.iter()
                                     .map(|line| {
                                         line.split_whitespace()
                                             .map(|number| number.parse())
                                             .collect()
                                     })
                                     .collect());

    Ok((a, b, c))
}

// Try to read from stdin. If successful, then try to parse. Read it again if parse failed.
fn console_input() -> Result<(Vec<u64>, Vec<u64>, Vec<Vec<u64>>), io::Error> {
    let mut a: Vec<u64>;
    let mut b: Vec<u64>;
    let mut c: Vec<Vec<u64>>;
    let mut buffer = String::new();

    println!("Введите количество поставщиков");
    loop {
        print!("A = ");
        try!(io::stdout().flush());
        buffer.clear();
        try!(io::stdin().read_line(&mut buffer));
        match buffer.trim().parse() {
            Ok(size) => {
                if size != 0 {
                    a = vec![0;size];
                    break;
                } else {
                    println!("Ошибка: количество поставщиков должно быть больше 0");
                }
            }
            Err(_) => println!("Ошибка: количество поставщиков должно быть целым неотрицательным числом"),
        }
    }

    println!("Введите количество потребителей");
    loop {
        print!("B = ");
        try!(io::stdout().flush());
        buffer.clear();
        try!(io::stdin().read_line(&mut buffer));
        match buffer.trim().parse() {
            Ok(size) => {
                if size != 0 {
                    b = vec![0;size];
                    break;
                } else {
                    println!("Ошибка: количество потребителей должно быть больше 0");
                }
            }
            Err(_) => println!("Ошибка: количество потребителей должно быть целым неотрицательным числом"),
        }
    }

    println!("Введите количество груза у каждого поставщика");
    for i in 0..a.len() {
        loop {
            print!("a[{}] = ", i + 1);
            try!(io::stdout().flush());
            buffer.clear();
            try!(io::stdin().read_line(&mut buffer));
            match buffer.trim().parse() {
                Ok(amount) => {
                    a[i] = amount;
                    break;
                }
                Err(_) => println!("Ошибка: количество груза должно быть целым неотрицательным числом"),
            }
        }
    }

    println!("Введите количество заказоного груза у каждого потребителя");
    for i in 0..b.len() {
        loop {
            print!("b[{}] = ", i + 1);
            try!(io::stdout().flush());
            buffer.clear();
            try!(io::stdin().read_line(&mut buffer));
            match buffer.trim().parse() {
                Ok(amount) => {
                    b[i] = amount;
                    break;
                }
                Err(_) => println!("Ошибка: количество груза должно быть целым неотрицательным числом"),
            }
        }
    }

    println!("Введите стоимости перевозок");
    c = vec![vec![0;b.len()];a.len()];
    for i in 0..a.len() {
        for j in 0..b.len() {
            loop {
                print!("c[{}][{}] = ", i + 1, j + 1);
                try!(io::stdout().flush());
                buffer.clear();
                try!(io::stdin().read_line(&mut buffer));
                match buffer.trim().parse() {
                    Ok(cost) => {
                        c[i][j] = cost;
                        break;
                    }
                    Err(_) => println!("Ошибка: стоимость должна быть целым неотрицательным числом"),
                }
            }
        }
    }

    Ok((a, b, c))
}

fn print_usage(opts: &Options, reason: &str) {
    let reason = format!("{}\nusage: {} [options] <file>...",
                         reason,
                         std::env::args_os().next().unwrap().to_string_lossy());
    println!("{}", opts.usage(&reason));
}

fn main() {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(std::env::args().skip(1)) {
        Ok(m) => m,
        Err(f) => {
            print_usage(&opts, &f.to_string());
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_usage(&opts, "");
        std::process::exit(0);
    }

    if matches.free.is_empty() {
        let (a, b, c) = match console_input() {
            Ok((a, b, c)) => (a, b, c),
            Err(err) => panic!(err.to_string()),
        };
        match Transportation::new(a, b, c) {
            Ok(mut t) => {
                t.potential_method();
                t.printstd();
            }
            Err(err) => panic!(err.to_string()),
        };
    } else {
        for file in matches.free.iter().map(PathBuf::from) {
            let (a, b, c) = match file_input(&file) {
                Ok((a, b, c)) => (a, b, c),
                Err(err) => {
                    println!("{:?}:{}", file, err.to_string());
                    continue;
                }
            };
            match Transportation::new(a, b, c) {
                Ok(mut t) => {
                    println!("{:?}", file);
                    t.potential_method();
                    t.printstd();
                }
                Err(err) => {
                    match err {
                        Error::NumOfSupOrCust => {
                            println!("{:?}: количество поставщиков и потребителей не должно быть меньше 0",
                                     file)
                        }
                        Error::NumOfRows => {
                            println!("{:?}: количество поставщиков не равно количеству строк в матрице стоимостей",
                                     file)
                        }
                        Error::NumOfCols => {
                            println!("{:?}: количество потребителей не равно количеству столбцов в матрице стоимостей",
                                     file)
                        }
                    }
                }
            }
        }
    }
}
