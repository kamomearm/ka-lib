use cargo_snippet::snippet;

#[snippet("lcm/gcd")]
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
#[snippet("lcm/gcd")]
pub fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
}

#[snippet("modinv")]
pub fn modinv(mut a: isize, m: isize) -> isize {
    //! `a mod m`の逆元
    //!
    //! `a`と`m`の互いが素を仮定
    let mut b = m;
    let mut u = 1;
    let mut v = 0;

    while b > 0 {
        let t = a.div_euclid(b);
        a -= t * b;
        (a, b) = (b, a);
        u -= t * v;
        (u, v) = (v, u);
    }
    u = u.rem_euclid(m);
    u
}

pub fn get_divisors(n: usize) -> Vec<usize> {
    //! `n`の約数の取得
    let mut ret = vec![];
    for i in 1..=n {
        if i * i > n {
            break;
        }
        if n & i != 0 {
            continue;
        }
        ret.push(i);
        if i * i != n {
            ret.push(n / i)
        }
    }
    ret
}

pub fn divisors_num(n: usize) -> usize {
    //! `n`の約数の個数
    let mut ret = 0;
    for i in 1..=n {
        if i * i > n {
            break;
        }
        if n % i != 0 {
            continue;
        }
        ret += 1;
        if i * i != n {
            ret += 1;
        }
    }
    ret
}
pub fn divisors_num_range(n: usize) -> Vec<usize> {
    //! `..=n`について約数の個数を求める
    let mut ret = vec![0; n + 1];
    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            ret[j] += 1;
        }
    }
    ret
}

pub fn is_prime(n: usize) -> bool {
    for i in 2..=n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
    }
    true
}
pub fn factorize(n: usize) -> Vec<(usize, usize)> {
    //! `n`の素因数分解
    //!
    //! 整数での試し割法
    let mut ret = vec![];
    let mut m = n;
    for i in 2..=n {
        if i * i > n {
            break;
        }

        let mut exp = 0;
        while m % i == 0 {
            exp += 1;
            m /= i;
        }

        if 0 < exp {
            ret.push((i, exp));
        }
    }
    if 2 <= m {
        ret.push((m, 1));
    }
    ret
}
pub fn eratostenes_thive(n: usize) -> Vec<bool> {
    //! エラトステネスの篩
    //!
    //! `O(NloglogN)`
    let mut isprime = vec![true; n + 1];
    isprime[0] = false;
    isprime[1] = true;

    for i in 2..=n {
        if i * i > n {
            break;
        }
        if !isprime[i] {
            continue;
        }
        for j in (2 * i..=n).step_by(i) {
            isprime[j] = false;
        }
    }
    isprime
}
pub fn factorize_by_minfactor(mut n: usize, minfactor: &[usize]) -> Vec<(usize, usize)> {
    //! 最小素因数を用いて`n`について素因数分解
    //!
    //! `O(logN)`
    let mut ret = Vec::new();
    while n > 1 {
        let p = minfactor[n];
        let mut exp = 0;
        while minfactor[n] == p {
            n /= p;
            exp += 1;
        }
        ret.push((p, exp))
    }
    ret
}
pub fn get_minfactor(n: usize) -> Vec<usize> {
    //! `..=n`について、最小素因数を求める
    //!
    //! `O(NloglogN)`
    let mut isprime = vec![true; n + 1];
    let mut minfactor = vec![0; n + 1];
    isprime[0] = false;
    isprime[1] = false;
    minfactor[1] = 1;

    for i in 2..=n {
        if !isprime[i] {
            continue;
        }
        minfactor[i] = i;
        for j in (2 * i..=n).step_by(i) {
            isprime[j] = false;
            if minfactor[j] == 0 {
                minfactor[j] = i;
            }
        }
    }
    minfactor
}

pub fn pow_mod(mut base: isize, mut exp: usize, r#mod: usize) -> usize {
    let mut ans = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            ans *= base;
            ans = ans.rem_euclid(r#mod.try_into().unwrap());
        }
        base *= base;
        base = base.rem_euclid(r#mod.try_into().unwrap());
        exp = exp.div_euclid(2);
    }
    ans as usize
}

#[snippet("NRadixFrom")]
pub trait NRadixFrom {
    /// 10進法から任意の進法へのコンバート
    /// 
    /// n進法から10進法へは、from_str_radixを使う
    fn n_radix_from(&self, n: u64) -> Vec<u64>;
}
#[snippet("NRadixFrom")]
impl NRadixFrom for String {
    fn n_radix_from(&self, n: u64) -> Vec<u64> {
        self.chars().collect::<Vec<char>>().n_radix_from(n)
    }
}
#[snippet("NRadixFrom")]
impl NRadixFrom for &str {
    fn n_radix_from(&self, n: u64) -> Vec<u64> {
        self.chars().collect::<Vec<char>>().n_radix_from(n)
    }
}
#[snippet("NRadixFrom")]
impl NRadixFrom for [char] {
    fn n_radix_from(&self, n: u64) -> Vec<u64> {
        let mut a = 0;
        for c in self.iter() {
            a *= 10;
            a += c.to_digit(10).unwrap() as usize;
        }
        a.n_radix_from(n)
    }
}
#[snippet("NRadixFrom")]
impl NRadixFrom for usize {
    fn n_radix_from(&self, n: u64) -> Vec<u64> {
        let x = self.clone() as u64;
        x.n_radix_from(n)
    }
}
#[snippet("NRadixFrom")]
impl NRadixFrom for u8 {
    fn n_radix_from(&self, n: u64) -> Vec<u64> {
        let x = self.clone() as usize;
        x.n_radix_from(n)
    }
}
#[snippet("NRadixFrom")]
impl NRadixFrom for u16 {
    fn n_radix_from(&self, n: u64) -> Vec<u64> {
        let x = self.clone() as usize;
        x.n_radix_from(n)
    }
}
#[snippet("NRadixFrom")]
impl NRadixFrom for u32 {
    fn n_radix_from(&self, n: u64) -> Vec<u64> {
        let x = self.clone() as usize;
        x.n_radix_from(n)
    }
}
#[snippet("NRadixFrom")]
impl NRadixFrom for u64 {
    fn n_radix_from(&self, n: u64) -> Vec<u64> {
        let mut ret = vec![];
        let mut x = self.clone();
        let n = n as u64;
        while 0 < x {
            let t = x%n;
            ret.push(t);
            x /= n;
        }
        ret.reverse();
        ret
    }
}
#[snippet("NRadixFrom")]
impl NRadixFrom for u128 {
    fn n_radix_from(&self, n: u64) -> Vec<u64> {
        let n = n as u128;
        let mut ret = vec![];
        let mut x = self.clone();
        while 0 < x {
            let t = x%n;
            ret.push(t as u64);
            x /= n;
        }
        ret.reverse();
        ret
    }
}
#[cfg(test)]
mod test {
    use super::NRadixFrom;
    #[test]
    fn from_str_radix() {
        let s = "101";
        assert_eq!(10, u8::from_str_radix(s, 3).unwrap());

        assert_eq!(vec![1, 0], 2usize.n_radix_from(2));

        let v = vec!['2'];
        assert_eq!(vec![1, 0], v.n_radix_from(2));
        
        assert_eq!(vec![1, 0], "2".n_radix_from(2));

        
    }
}
