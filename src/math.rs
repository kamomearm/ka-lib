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
    //! 約数の取得
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
    //! `n`以下の正整数の約数の個数
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
    //! `n`について素因数分解
    //!
    //! `O(logN)`
    let mut ret = Vec::new();
    while n > 1 {
        let p = minfactor[n as usize];
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
    //! `..=n`について、最小の素因数を求める
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
