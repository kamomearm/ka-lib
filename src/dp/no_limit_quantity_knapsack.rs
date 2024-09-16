#![allow(unused)]
use std::{collections::*};
use proconio::*;

/// 個数制限なしナップザック問題
fn main() {
    input! {
        n: usize, w: usize,
        vw: [(isize, usize); n]
    }
    // dp[i][j] := i番目までで、重さがjの時の価値の最大
    let mut dp = vec![vec![-1; w+1]; n+1];
    dp[0][0] = 0;

    for i in 1..=n {
        for j in 0..=w {
            let (vi, wi) = vw[i-1];
            if j < wi {
                dp[i][j] = dp[i][j].max(dp[i-1][j]);
            }
            else {
                dp[i][j] = dp[i][j].max(dp[i-1][j].max(dp[i][j-wi]+vi));
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap())
}

fn kubaru() {
    input! {
        n: usize, w: usize,
        vw: [(isize, usize); n]
    }
    // dp[i][j] := i番目までで、重さがjの時の価値の最大
    let mut dp = vec![vec![-1; w+1]; n+1];
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..=w {
            let (vi, wi) = vw[i];
            dp[i+1][j] = dp[i+1][j].max(dp[i][j]);
            if j+wi <= w {
                let nj = j+wi;
                // 現在のiに対して遷移させる
                // 
                dp[i][nj] = dp[i][nj].max(dp[i][j]+vi);
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap())
}