pub mod unionfind;
pub mod scc;

use cargo_snippet::snippet;
use unionfind::UnionFind;
#[snippet]
pub fn warshall_froyd(g: &Vec<Vec<(isize, usize)>>) {
    //! 全始点最短路
    //!
    //! `(cost, nv)`のグラフ
    //!
    //! `O(|V|^3)`

    const INF: isize = isize::MAX;
    let n = g.len();
    let mut dist = vec![vec![INF; g.len()]; g.len()];
    for v in 0..n {
        for &(cost, nv) in &g[v] {
            dist[v][nv] = isize::min(dist[v][nv], cost);
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j])
            }
        }
    }
}

#[snippet]
pub fn dijkstra(st: usize, g: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    //! `O(|E|log|V|)`
    use std::cmp::Reverse;

    let n = g.len();
    let inf: usize = 1 << 60;
    let mut kakutei = vec![false; n];
    let mut dist = vec![inf; n];
    let mut pq = std::collections::BinaryHeap::new();

    dist[st] = 0;
    pq.push(Reverse((dist[st], st)));

    while let Some(Reverse((_, v))) = pq.pop() {
        if kakutei[v] == true {
            continue;
        }

        kakutei[v] = true;
        for (ncos, nv) in &g[v] {
            if dist[*nv] > dist[v] + ncos {
                dist[*nv] = dist[v] + ncos;
                pq.push(Reverse((dist[*nv], *nv)))
            }
        }
    }
    dist
}

#[snippet]
pub fn bellman_ford(st: usize, g: &Vec<Vec<(isize, usize)>>) -> Vec<isize> {
    //! `st`からの単一始点最短路を返す
    //!
    //! `(cost, nv)`のグラフ
    //!
    //! 不閉路になるものは`-inf`にして、返す
    //!
    //! `O(|V| |E|)`
    let n = g.len();
    const INF: isize = isize::MAX;
    const NEG_INF: isize = isize::MIN;

    let mut dist = vec![INF; n];
    dist[st] = 0;

    for _ in 0..n - 1 {
        for v in 0..n {
            if dist[v] == INF {
                continue;
            }
            for (c, nv) in &g[v] {
                if dist[*nv] > dist[v] + c {
                    dist[*nv] = dist[v] + c
                }
            }
        }
    }
    // 負閉路検出
    // 負閉路となるものは-infに変更
    for _ in 0..n {
        for v in 0..n {
            for (c, nv) in &g[v] {
                if dist[*nv] == INF {
                    continue;
                }
                if dist[*nv] > dist[v] + c {
                    dist[*nv] = NEG_INF
                }
            }
        }
    }
    dist
}

#[snippet]
#[snippet(include = "Unionfind")]
pub fn kruskal(mut e: Vec<(isize, usize, usize)>, n: usize) -> (Vec<(isize, usize, usize)>, isize) {
    //! `(cost, v, nv)`の隣接リスト
    //! 
    //! 最小全域木に入る辺と両端点、総コストを返す
    //! 
    //! `O(|E|log|E|)`
    e.sort();
    let mut uf = UnionFind::new(n);

    let mut ans_cost = 0;
    let mut ans_e = vec![];
    for (c, v, u) in e {
        if uf.issame(v, u) {
            continue;
        }
        uf.unite(v, u);
        ans_cost += c;
        ans_e.push((c, v, u));
    }
    (ans_e, ans_cost)
}

// #[snippet]
pub fn isbiparrite(g: &[Vec<usize>]) -> bool {
    //! 二部グラフか否か
    //! 
    //! `O(|V| + |E|)`
    let n = g.len();
    let mut iro = vec![-1; n];
    let mut ok = true;

    for i in 0..n {
        if !ok {
            break;
        }
        if iro[i] != -1 {
            continue;
        }
        let mut q = std::collections::VecDeque::new();
        q.push_back((i, 0));
        iro[i] = 0;

        while let Some((v, c)) = q.pop_front() {
            for &nv in &g[v] {
                if iro[nv] == -1 {
                    iro[nv] = c^1;
                    q.push_back((nv, iro[nv]));
                }
                else {
                    if c == iro[nv] {
                        ok = false;
                        return ok;
                    }
                }
            }
        }
    }
    ok
}

pub fn topological_sort(g: &[Vec<usize>], indeg: &mut [usize]) -> Vec<usize> {
    let mut ret = vec![];
    let mut q = std::collections::VecDeque::new();
    for i in 0..indeg.len() {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }
    while let Some(v) = q.pop_front() {
        for nv in &g[v] {
            indeg[*nv] -= 1;
            if indeg[*nv] == 0 {
                q.push_back(*nv);
            }
        }
        ret.push(v);
    }
    ret
}
