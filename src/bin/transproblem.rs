extern crate transproblem;
use transproblem::Transportation;

use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;

fn file_input() -> (Vec<u64>, Vec<u64>, Vec<Vec<u64>>) {
    print!("Введите имя файла: ");
    io::stdout().flush().unwrap();
    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Не удалось считать имя файла");

    let file_name = file_name.trim();
    let mut f = BufReader::new(File::open(file_name).expect("Не удалось открыть файл"));

    let mut a = String::new();
    f.read_line(&mut a).expect("Не удалось считать строку в файле");
    let a: Vec<u64> = a.split_whitespace()
                       .map(|number| {
                           number.parse()
                                 .expect("Не удалось распознасть количество груза у поставщиков")
                       })
                       .collect();

    let mut b = String::new();
    f.read_line(&mut b).expect("Не удалось считать строку в файле");
    let b: Vec<u64> = b.split_whitespace()
                       .map(|number| {
                           number.parse()
                                 .expect("Не удалось распознасть количество заказоного груза у потребителей")
                       })
                       .collect();

    let c: Vec<Vec<u64>> = f.lines()
                            .map(|l| {
                                l.expect("Не удалось распознасть стоимости перевозок")
                                 .split_whitespace()
                                 .map(|number| {
                                     number.parse()
                                           .expect("Не удалось распознасть стоимости перевозок")
                                 })
                                 .collect()
                            })
                            .collect();

    (a, b, c)
}

fn console_input() -> (Vec<u64>, Vec<u64>, Vec<Vec<u64>>) {
    let mut a_size = String::new();
    let mut b_size = String::new();
    let mut a: Vec<u64> = Vec::new();
    let mut b: Vec<u64> = Vec::new();
    let mut c: Vec<Vec<u64>> = Vec::new();

    println!("Введите количество поставщиков");
    print!("A = ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut a_size)
        .expect("не удалось считать линию");
    let a_size: usize = a_size.trim().parse().expect("Не удалось распознать количество поставщиков");

    println!("Введите количество потребителей");
    print!("B = ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut b_size)
        .expect("не удалось считать линию");
    let b_size: usize = b_size.trim().parse().expect("Не удалось распознать количество потребителей");

    let mut buf = String::new();

    println!("Введите количество грузка у каждого поставщика");
    for i in 0..a_size {
        print!("a[{}] = ", i + 1);
        io::stdout().flush().unwrap();
        buf.clear();
        io::stdin()
            .read_line(&mut buf)
            .expect("не удалось считать линию");
        a.push(buf.trim().parse().expect("Не удалось распознать количество груза"));
    }

    println!("Введите количество заказоного груза у каждого потребителя");
    for i in 0..b_size {
        print!("b[{}] = ", i + 1);
        io::stdout().flush().unwrap();
        buf.clear();
        io::stdin()
            .read_line(&mut buf)
            .expect("не удалось считать линию");
        b.push(buf.trim().parse().expect("Не удалось распознать количество заказоного груза"));
    }

    println!("Введите стоимости перевозок");
    for i in 0..a_size {
        c.push(Vec::new());
        for j in 0..b_size {
            print!("c[{}][{}] = ", i + 1, j + 1);
            io::stdout().flush().unwrap();
            buf.clear();
            io::stdin()
                .read_line(&mut buf)
                .expect("не удалось считать линию");
            c[i].push(buf.trim().parse().expect("Не удалось распознать стоимость перевозоки"));
        }
    }

    (a, b, c)
}

fn main() {
    let f;
    loop {
        println!("1) Ввод с клавиатуры");
        println!("2) Ввод из файла");
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("не удалось считать линию");
        match buf.trim().parse::<u8>() {
            Ok(num) => {
                if num == 1 || num == 2 {
                    if num == 1 {
                        f = true;
                    } else {
                        f = false;
                    }
                    break;
                } else {
                    println!("Ошибка, попробуйте снова");
                    continue;
                }
            }
            Err(_) => {
                println!("Ошибка, попробуйте снова");
                continue;
            }
        };
    }

    let (a, b, c) = if f {
        console_input()
    } else {
        file_input()
    };

    let mut t = match Transportation::new(a, b, c) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };
    t.potential_method();
    t.printstd();
}
