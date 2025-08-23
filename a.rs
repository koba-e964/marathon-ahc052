#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

struct Rng {
    x: u64,
}

impl Rng {
    fn next(&mut self) -> u32 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        ((x ^ x << 10) >> 32) as _
    }
}

#[allow(unused)]
trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn calc_bitboard(
    ij: &[(usize, usize)], v: &[Vec<char>], h: &[Vec<char>],
    alloc: &[Vec<char>], ops: &[usize],
) -> Vec<u32> {
    let n = 30;
    let mut bitboard = vec![0; n];
    for (i, row) in alloc.iter().enumerate() {
        let (mut x, mut y) = ij[i];
        for &o in ops {
            bitboard[x] |= 1 << y;
            let letter = row[o];
            let (nx, ny) = match letter {
                'U' => (x.wrapping_sub(1), y),
                'D' => (x.wrapping_add(1), y),
                'L' => (x, y.wrapping_sub(1)),
                'R' => (x, y.wrapping_add(1)),
                'S' => (x, y),
                _ => unreachable!(),
            };
            if nx >= n || ny >= n {
                continue;
            }
            let is_blocked = match letter {
                'U' => h[nx][y] == '1',
                'D' => h[x][y] == '1',
                'L' => v[x][ny] == '1',
                'R' => v[x][y] == '1',
                'S' => false,
                _ => panic!(),
            };
            if !is_blocked {
                x = nx;
                y = ny;
            }
        }
        bitboard[x] |= 1 << y;
    }
    bitboard
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize, k: usize,
        ij: [(usize, usize); m],
        v: [chars; n],
        h: [chars; n - 1],
    }
    let mut rng = Rng { x: 0xdead_c0de_0013_3331u64 };
    let mut alloc = vec![vec!['D'; k]; m];
    for i in 0..m {
        for j in 0..k {
            let r = rng.next() as usize % 5;
            alloc[i][j] = b"UDLRS"[r] as char;
        }
    }
    let mut ops = vec![0; 2 * n * n];
    for i in 0..2 * n * n {
        ops[i] = rng.next() as usize % k;
    }
    let bitboard = calc_bitboard(&ij, &v, &h, &alloc, &ops);
    let mut score = 0;
    for i in 0..n {
        score += bitboard[i].count_ones();
    }
    eprintln!("score = {score}");
    // emit ans
    for i in 0..k {
        let mut tmp = vec![];
        for j in 0..m {
            tmp.push(alloc[j][i]);
        }
        putvec!(tmp);
    }
    for o in ops {
        puts!("{o}\n");
    }
}
