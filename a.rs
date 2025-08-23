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

fn try_move(
    x: usize, y: usize,
    v: &[Vec<char>], h: &[Vec<char>], dir: char,
) -> Option<(usize, usize)> {
    let n = 30;
    let (nx, ny) = match dir {
        'U' => (x.wrapping_sub(1), y),
        'D' => (x.wrapping_add(1), y),
        'L' => (x, y.wrapping_sub(1)),
        'R' => (x, y.wrapping_add(1)),
        'S' => (x, y),
        _ => unreachable!(),
    };
    if nx >= n || ny >= n {
        return None;
    }
    let is_blocked = match dir {
        'U' => h[nx][y] == '1',
        'D' => h[x][y] == '1',
        'L' => v[x][ny] == '1',
        'R' => v[x][y] == '1',
        'S' => false,
        _ => panic!(),
    };
    if is_blocked {
        None
    } else {
        Some((nx, ny))
    }
}

fn calc_bitboard(
    ij: &[(usize, usize)], v: &[Vec<char>], h: &[Vec<char>],
    alloc: &[Vec<char>], ops: &[usize],
) -> (Vec<u32>, Vec<(usize, usize)>) {
    let n = 30;
    let mut bitboard = vec![0; n];
    let mut pts = vec![];
    for (i, row) in alloc.iter().enumerate() {
        let (mut x, mut y) = ij[i];
        for &o in ops {
            bitboard[x] |= 1 << y;
            let letter = row[o];
            if let Some((nx, ny)) = try_move(x, y, v, h, letter) {
                x = nx;
                y = ny;
            }
        }
        bitboard[x] |= 1 << y;
        pts.push((x, y));
    }
    (bitboard, pts)
}

fn calc_distance(
    v: &[Vec<char>], h: &[Vec<char>],
    bitboard: &[u32],
) -> Vec<Vec<i32>> {
    let n = 30;
    let mut dist = vec![vec![1000; n]; n];
    let mut que = VecDeque::new();
    for i in 0..n {
        for j in 0..n {
            if (bitboard[i] >> j) & 1 == 0 {
                que.push_back((0, i, j));
            }
        }
    }
    while let Some((d, x, y)) = que.pop_front() {
        if dist[x][y] <= d {
            continue;
        }
        dist[x][y] = d;
        for &(nx, ny) in &[(x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)] {
            if nx >= n || ny >= n {
                continue;
            }
            let is_blocked = if nx < x {
                h[nx][y] == '1'
            } else if nx > x {
                h[x][y] == '1'
            } else if ny < y {
                v[x][ny] == '1'
            } else {
                v[x][y] == '1'
            };
            if !is_blocked && dist[nx][ny] > d + 1 {
                que.push_back((d + 1, nx, ny));
            }
        }
    }
    dist
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
        let (now_bb, now_pts) = calc_bitboard(&ij, &v, &h, &alloc, &ops[..i]);
        let dist = calc_distance(&v, &h, &now_bb);
        let mut best = (1 << 30, 0);
        for i in 0..k {
            let mut sum = 0;
            for j in 0..m {
                let np = try_move(now_pts[j].0, now_pts[j].1, &v, &h, alloc[j][i]);
                let np = np.unwrap_or(now_pts[j]);
                sum += dist[np.0][np.1];
            }
            best = best.min((sum, i));
        }
        ops[i] = best.1;
    }
    let (bitboard, _) = calc_bitboard(&ij, &v, &h, &alloc, &ops);
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
