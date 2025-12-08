use rust::utils::read_input;

#[allow(dead_code)]
const TEST_STRING: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

const DAY: usize = 8;

#[derive(Debug, Clone, Copy)]
struct Pos3d {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_line(line: &str) -> Pos3d {
    let parts: Vec<&str> = line.split(',').collect();
    Pos3d {
        x: parts[0].parse().unwrap(),
        y: parts[1].parse().unwrap(),
        z: parts[2].parse().unwrap(),
    }
}

fn distance(a: &Pos3d, b: &Pos3d) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    (dx * dx + dy * dy + dz * dz) as f64
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        // path compression
        while self.parent[x] != root {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }
        root
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        }
        // union by size
        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
        true
    }

    fn component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut counts = vec![0usize; n];
        for i in 0..n {
            let r = self.find(i);
            counts[r] += 1;
        }
        counts.into_iter().filter(|&c| c > 0).collect()
    }
}

fn main() {
    let input = read_input(DAY).unwrap_or_else(|_| TEST_STRING.to_string());
    let now = std::time::Instant::now();
    let positions: Vec<Pos3d> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect();

    let n = positions.len();
    // compute all pair distances
    let mut pairs: Vec<(f64, usize, usize)> = Vec::with_capacity(n * (n.saturating_sub(1)) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let d = distance(&positions[i], &positions[j]);
            pairs.push((d, i, j));
        }
    }
    // sort ascending by distance
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Part 1: connect the 1000 closest pairs (counting pairs even if they don't merge components)
    let mut uf = UnionFind::new(n);
    let mut edges_used = 0usize;
    let target = 1000usize.min(pairs.len());
    for &(_d, i, j) in pairs.iter() {
        if edges_used >= target {
            break;
        }
        uf.union(i, j);
        edges_used += 1;
    }

    let mut sizes = uf.component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    // product of three largest components
    let three = sizes.iter().take(3).cloned().collect::<Vec<_>>();
    let product: usize = three.iter().product();

    println!("Part 1: {}", product);

    // Part 2: keep connecting the closest pairs but only count pairs that actually connect two different components
    let mut uf2 = UnionFind::new(n);
    let mut components = n;
    let mut last_pair: Option<(usize, usize)> = None;
    for &(_d, i, j) in pairs.iter() {
        if uf2.union(i, j) {
            components -= 1;
            last_pair = Some((i, j));
            if components == 1 {
                break;
            }
        }
    }

    if let Some((i, j)) = last_pair {
        let prod_x = positions[i].x * positions[j].x;
        println!("Part 2: {}", prod_x);
    }

    println!("Elapsed time: {:?}", now.elapsed());
}
