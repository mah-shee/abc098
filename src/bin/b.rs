#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = 0;
    for i in 0..n {
        let mut a = std::collections::HashSet::new();
        for j in 0..i {
            a.insert(s[j]);
        }
        let mut b = std::collections::HashSet::new();
        for j in i..n {
            b.insert(s[j]);
        }
        let c = a.iter().filter(|&x| b.contains(x)).count();
        ans = std::cmp::max(ans, c);
    }
    println!("{}", ans);
}
