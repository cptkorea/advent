use std::collections::HashMap;

pub struct UnionFind {
    size: usize,
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let mut parents = vec![0; n];
        for (i, p) in parents.iter_mut().enumerate() {
            *p = i;
        }
        let sizes = vec![1; n];

        UnionFind {
            size: n,
            parents,
            sizes,
        }
    }

    /// Root representative with path compression (`O(α(n))` amortized).
    pub fn find(&mut self, n: usize) -> usize {
        let curr = n;
        if self.parents[curr] == curr {
            return curr;
        }
        let p = self.find(self.parents[curr]);
        self.parents[curr] = p;
        p
    }

    pub fn connect(&mut self, n1: usize, n2: usize) {
        let p1 = self.find(n1);
        let p2 = self.find(n2);

        if p1 == p2 {
            return;
        }

        if self.sizes[p1] > self.sizes[p2] {
            self.parents[p1] = p2;
            self.sizes[p2] += self.sizes[p1];
        } else {
            self.parents[p2] = p1;
            self.sizes[p1] += self.sizes[p2];
        }
    }

    /// Partitions `{0 .. self.size}` into disjoint sets (each inner vec sorted).
    /// Outer order is sorted by the smallest index in each group.
    pub fn disjoint_groups(&mut self) -> Vec<(usize, Vec<usize>)> {
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..self.size {
            let r = self.find(i);
            map.entry(r).or_default().push(i);
        }

        let mut groups: Vec<Vec<usize>> = map
            .into_values()
            .map(|mut v| {
                v.sort_unstable();
                v
            })
            .collect();

        groups.sort_by_key(|g| g[0]);

        groups.into_iter().map(|g| (g.len(), g)).collect()
    }

    pub fn component_count(&mut self) -> usize {
        let mut roots = HashMap::new();
        for i in 0..self.size {
            roots.insert(self.find(i), ());
        }
        roots.len()
    }

    pub fn get_parents(&self) -> Vec<usize> {
        self.parents.clone()
    }

    pub fn get_sizes(&self) -> Vec<usize> {
        self.sizes.clone()
    }
}
