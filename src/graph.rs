pub mod scc;
pub mod unionfind;

use cargo_snippet::snippet;
use unionfind::UnionFind;

pub fn cycle_detection(
    g: &[Vec<usize>],
    v: usize,
    seen: &mut Vec<bool>,
    finished: &mut Vec<bool>,
    history: &mut Vec<usize>,
) -> Option<usize> {
    //! サイクルがあれば、その起点の`Some(usize)`を返す
    //!
    //! 有向グラフのみ
    //! 
    //! `O(|V| + |E|)`
    seen[v] = true;
    history.push(v);
    for &nv in &g[v] {
        if v == nv {
            continue;
        }
        if finished[nv] {
            continue;
        }
        if seen[nv] && !finished[nv] {
            history.push(nv);
            return Some(nv);
        }
        let pos = cycle_detection(g, nv, seen, finished, history);
        if pos.is_some() {
            return pos;
        }
    }
    finished[v] = true;
    history.pop();

    None
}

pub fn cycle_reconstruct(pos: usize, history: &mut Vec<usize>) -> Vec<usize> {
    //! `pos:`サイクルを検出した頂点
    let mut cycle = vec![];
    history.pop();
    while let Some(v) = history.pop() {
        cycle.push(v);
        if v == pos {
            break;
        }
    }
    cycle.reverse();
    cycle
}


// #[snippet]
pub fn euler_tour(v: usize, g: &[Vec<usize>]) -> (Vec<usize>, Vec<usize>) {
    //! 行きがけ順、帰りがけ順を半開区間にして出力。
    //! 部分木に対するクエリを区間のクエリに言い換える。
    //!
    //! v_inの順に配列に入れて、セグ木での処理とか
    let mut cnt = 0;
    let mut seen = vec![false; g.len()];
    let mut v_in = vec![0; g.len()];
    let mut v_out = vec![0; g.len()];

    // 1が行きがけ、2が帰りがけ
    let mut stack = vec![(2, v), (1, v)];

    while let Some((i, v)) = stack.pop() {
        match i {
            1 => {
                seen[v] = true;
                v_in[v] = cnt;
                for &nv in g[v].iter().rev() {
                    if seen[nv] {
                        continue;
                    }
                    stack.push((2, nv));
                    stack.push((1, nv));
                }
            }
            2 => {
                v_out[v] = cnt;
            }
            _ => unreachable!(),
        }
        cnt += 1;
    }
    (v_in, v_out)
}

#[snippet]
pub fn dfs(v: usize, p: usize, g: &[Vec<usize>]) {
    for nv in &g[v] {
        if p == *nv {
            continue;
        }
        dfs(*nv, p, g);
    }
}

#[snippet]
pub fn bfs(st: usize, g: &[Vec<usize>]) -> Vec<isize> {
    let n = g.len();
    let mut dist: Vec<isize> = vec![-1; n];
    let mut q = std::collections::VecDeque::new();
    q.push_back(st);
    dist[st] = 0;
    while let Some(v) = q.pop_front() {
        for nv in &g[v] {
            if dist[*nv] != -1 {
                continue;
            }
            dist[*nv] = dist[v] + 1;
            q.push_back(*nv);
        }
    }
    dist
}

#[snippet]
pub fn warshall_froyd(g: &[Vec<(isize, usize)>]) {
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
pub fn dijkstra(st: usize, g: &[Vec<(usize, usize)>]) -> Vec<usize> {
    //! `(cost, nv)`の隣接リスト
    //!
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
        if kakutei[v] {
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
pub fn bellman_ford(st: usize, g: &[Vec<(isize, usize)>]) -> Vec<isize> {
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
    //! 最小全域木に入る辺と両端点、総コストを返す
    //!
    //! `(cost, v, nv)`の隣接リスト
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
                    iro[nv] = c ^ 1;
                    q.push_back((nv, iro[nv]));
                } else {
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

#[cfg(test)]
mod test {
    use crate::graph::euler_tour;
    use itertools::Itertools;

    #[test]
    fn et() {
        let g = vec![
            vec![1, 5],
            vec![0, 2, 4],
            vec![1, 3],
            vec![2],
            vec![1],
            vec![0],
        ];
        let (v_in, v_out) = euler_tour(0, &g);
        println!("{}", v_in.iter().join("\t"));
        println!("{}", v_out.iter().join("\t"));
    }
}
