// 拡張ユークリッドの互除法による
fn calc_nd(n: u64, r: u64) -> u64 {
    let mut x = n as i64;
    let mut m = r as i64;
    let mut a = (0, 1);
    let mut b = (1, 0);
    while x != 0 {
        let t = (m / x, x, m % x);
        let q = t.0;
        m = t.1;
        x = t.2;
        let t = (a.1 - q * a.0, a.0, b.1 - q * b.0, b.0);
        a = (t.0, t.1);
        b = (t.2, t.3);
    }

    (r as i64 - b.1) as u64 % r
}

// アルゴリズム5を使ったモンゴメリー剰余乗算
// 効率等は無視
pub fn multi5(n: u64, x: u64, y: u64) -> Option<u64> {
    const K: u64 = 32;
    const R: u64 = 1 << K;

    if n % 2 != 1 {
        return None;
    }

    let nd = calc_nd(n, R);

    let mut t = x * y;
    let tmp = ((nd as u128 * t as u128) % R as u128) * n as u128;
    t = ((t as u128 + tmp) >> K) as u64;
    if t >= n {
        t -= n;
    }

    Some(t)
}

// アルゴリズム7を使ったモンゴメリー剰余乗算
pub fn multi7(n: u64, x: u64, y: u64) -> Option<u64> {
    const K: u64 = 32; // k
    const B: u64 = 8; // r
    const M: u64 = K / B; // m
    // const R: u64 = 2 ^ K; // R
    const R: u64 = 1 << K;

    fn get_digit(n: u64, i: u64) -> u64 {
        let d = i * B;
        let mask = ((1 << B) - 1) << d;
        (n & mask) >> d
    }

    // n と R (= 2^k) が互いに素であるかの確認
    if get_digit(n, 0) % 2 != 1 {
        return None;
    }

    // calc n'
    let nd = calc_nd(n, R);

    println!("nd = {:x}", nd);
    println!("n * (-nd) = {:x}", (n * (R - nd)) % R);

    let mut t = 0;
    let x_0 = get_digit(x, 0);

    for i in 0..M {
        // let x_i = get_digit(x, i);
        let y_i = get_digit(y, i);
        let q = ((t + x_0 * y_i) * nd) & ((1 << B) - 1);
        // let q = get_digit(q, 0);
        println!("q_{} = {:x}", i, q);
        t = (t + q * n + y_i * x) >> B;
        println!("t_{} = {:x}", i, t);
    }

    if n <= t {
        t -= n;
    }

    Some(t)
}

pub fn restore<F>(n: u64, x: u64, mul: F) -> Option<u64>
where F: Fn(u64, u64, u64) -> Option<u64>
{
    const R: u64 = 1 << 32;
    let r_2 = ( (R as u128 * R as u128) % n as u128) as u64;

    mul(n, x, r_2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;

        let x = 0xf3d72f88;
        let y = 0xf6c6ba98;
        let n = 0xf981892b;
    
        assert_eq!(multi5(n, x, y).unwrap(), 0xdeca8e3e);
        println!("==========");
        assert_eq!(multi7(n, x, y).unwrap(), 0xdeca8e3e);
        println!("==========");
        assert_eq!(multi7(n, y, x).unwrap(), 0xdeca8e3e);
        println!("==========");
        assert_eq!(restore(n, 0xdeca8e3e, multi5).unwrap(), 0xe5481812);
        println!("==========");
        assert_eq!(restore(n, 0xdeca8e3e, multi7).unwrap(), 0xe5481812);
    }
}