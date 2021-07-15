use std::borrow::BorrowMut;
use std::collections::BTreeSet;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut trimmed = String::from(line.trim());
    let mut b = std::mem::take(&mut trimmed).into_bytes();
    let n = b.len();
    let mut set = BTreeSet::new();
    heaps(b.borrow_mut(), &mut set, n);
    println!("{}", set.len());
    set.iter().for_each(|s| println!("{}", s));
}

fn heaps(string: &mut [u8], set: &mut BTreeSet<String>, n: usize) {
    if n == 1 {
        set.insert(String::from_utf8(string.to_vec()).unwrap());
        return;
    }

    for i in 0..n {
        heaps(string, set, n - 1);
        if n % 2 == 0 {
            string.swap(i, n - 1);
        } else {
            string.swap(0, n - 1);
        }
    }
}
