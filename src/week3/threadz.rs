use rand::prelude::*;
use rayon::prelude::*;
use std::sync::mpsc::channel;
use std::thread;

struct Matrix(Vec<Vec<u8>>);

impl Matrix {
    fn random() -> Matrix {
        let mut rows = Vec::with_capacity(4096);
        (0..4096).into_iter().for_each(|_| {
            let mut col = Vec::with_capacity(4096);
            (0..4096).into_iter().for_each(|_| {
                col.push(rand::random::<u8>());
            });
            rows.push(col);
        });
        Matrix(rows)
    }

    fn parallel_sum(self) -> u32 {
        self.0
            .par_iter()
            .map(|col| col.par_iter().map(|&x| x as u32).sum::<u32>())
            .sum()
    }
}

pub fn default_main() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        (1..10).into_par_iter().for_each(|i| {
            tx.send(Matrix::random()).unwrap();
            println!("sent matrix {}", i);
        })
    });
    rx.into_iter().for_each(|m| {
        let s = m.parallel_sum();
        println!("matrix sum {}", s)
    })
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_sum() {
        default_main()
    }
}
