use std::io::{stdin, stdout, BufWriter, Write};

fn main() -> Result<(), std::io::Error> {
    let mut line = String::with_capacity(16);
    stdin().read_line(&mut line).unwrap();
    let mut n = line.trim().parse::<usize>().unwrap();

    let stdout = stdout();
    let lock = stdout.lock();
    let mut out = BufWriter::with_capacity(10_usize.pow(6), lock);

    write!(out, "{}", n)?;

    while n != 1 {
        match n {
            even if even % 2 == 0 => n /= 2,
            _odd => n = n * 3 + 1,
        }
        write!(out, " {}", n)?;
    }
    write!(out, "\n")?;

    out.flush()
}
