use std::cell::RefCell;

pub struct Scc {
    pub g: Vec<Vec<usize>>,
    pub rg: Vec<Vec<usize>>,
    pub kaerigake: RefCell<Vec<usize>>,
    pub dag: Vec<Vec<usize>>,
    pub group_vs: Vec<Vec<usize>>,
}
impl Scc {
    pub fn new(g: Vec<Vec<usize>>) -> Self {
        let mut rg = vec![vec![]; g.len()];
        for v in 0..g.len() {
            for &nv in &g[v] {
                rg[nv].push(v);
            }
        }
        Scc {
            g,
            rg,
            kaerigake: RefCell::new(vec![]),
            dag: vec![],
            group_vs: vec![],
        }
    }
    fn dfs(&self, st: usize, seen: &mut Vec<bool>) {
        seen[st] = true;

        for &nv in &self.g[st] {
            if seen[nv] {
                continue;
            }
            self.dfs(nv, seen);
        }
        self.kaerigake.borrow_mut().push(st);
    }
    fn rdfs(&self, st: usize, seen: &mut Vec<bool>, group: &mut Vec<usize>, cnt: usize) {
        seen[st] = true;
        group[st] = cnt;

        for &nv in &self.rg[st] {
            if seen[nv] {
                continue;
            }
            self.rdfs(nv, seen, group, cnt);
        }
    }
    pub fn scc(&mut self) {
        let mut seen = vec![false; self.g.len()];

        for v in 0..self.g.len() {
            if seen[v] {
                continue;
            }
            self.dfs(v, &mut seen);
        }

        seen.fill(false);
        let mut group = vec![usize::MAX; self.g.len()];
        let mut cnt = 0;

        for &v in self.kaerigake.borrow().iter().rev() {
            if seen[v] {
                continue;
            }
            self.rdfs(v, &mut seen, &mut group, cnt);
            cnt += 1;
        }

        self.dag = vec![vec![]; cnt];
        self.group_vs = vec![vec![]; cnt];

        for v in 0..self.g.len() {
            self.group_vs[group[v]].push(v);
            for &nv in &self.g[v] {
                if group[v] == group[nv] {
                    continue;
                }
                self.dag[group[v]].push(group[nv]);
            }
        }
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn refcell() {}
}
