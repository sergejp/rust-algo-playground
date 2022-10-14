use std::collections::{HashMap, HashSet, VecDeque};

type WeightedAdjacencyList = HashMap<usize, HashSet<(usize, usize)>>;
type AdjacencyList = HashMap<usize, HashSet<usize>>;

pub struct Graph {
    adjacency_list: HashMap<usize, HashSet<usize>>,
    vertices: HashSet<usize>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    pub fn from(
        adjacency_list: AdjacencyList,
        vertices: HashSet<usize>,
        edges: Vec<(usize, usize)>,
    ) -> Self {
        println!(
            "Creating graph from {} vertices, {} edges, adjacency_list.len()={}",
            vertices.len(),
            edges.len(),
            adjacency_list.len()
        );
        Graph {
            adjacency_list,
            vertices,
            edges,
        }
    }

    pub fn depth_first_search(&self, s: usize) {
        // mark all vertices as unexplored
        let mut explored: HashSet<usize> = HashSet::with_capacity(self.vertices.len());

        // create a stack with vertex "s" as a starting point
        let mut stack: Vec<usize> = vec![s];

        // traverse the graph starting from "s"
        while !stack.is_empty() {
            let v = stack.pop().unwrap();

            // if "v" is explored, check the next vertex in main loop
            if explored.contains(&v) {
                continue;
            }

            // mark "v" as explored
            explored.insert(v);

            // a vertex might not have adjacent vertices
            if !self.adjacency_list.contains_key(&v) {
                continue;
            }

            for adjacent_vertex in &self.adjacency_list[&v] {
                stack.push(*adjacent_vertex);
            }
        }
    }

    pub fn toposort_kahn(&self) -> Vec<usize> {
        let mut sorted = Vec::with_capacity(self.vertices.len());
        let mut zero_incoming = VecDeque::<usize>::with_capacity(self.vertices.len() / 10);
        let mut indegrees: HashMap<usize, usize> = HashMap::with_capacity(self.vertices.len());

        // compute indegrees for vertices which have incoming edges
        for v in self.adjacency_list.keys() {
            // println!("v={}", v);
            for adjacent_vertex in &self.adjacency_list[&v] {
                let count = indegrees.entry(*adjacent_vertex).or_insert(0);
                *count += 1;
            }
        }

        // compute vertices which do not have incoming edges
        for v in self.adjacency_list.keys() {
            if !indegrees.contains_key(&v) {
                indegrees.insert(*v, 0);
                zero_incoming.push_back(*v);
            }
        }

        // start processing from those source vertices which do not have incoming edges
        while !zero_incoming.is_empty() {
            let vertex = zero_incoming.pop_front().unwrap();
            sorted.push(vertex);

            if !self.adjacency_list.contains_key(&vertex) {
                continue;
            }

            for adjacent_vertex in &self.adjacency_list[&vertex] {
                let count = indegrees.entry(*adjacent_vertex).or_insert(0);
                *count -= 1;
                if *count == 0 {
                    zero_incoming.push_back(*adjacent_vertex);
                }
            }
        }

        // if assert fails, the graph has a cycle and hence it is not a DAG
        // TODO: replace with error return that says it's not a DAG
        assert!(sorted.len() == indegrees.len());

        sorted
    }

    pub fn kosaraju_scc(&self, order: &HashMap<usize, usize>) -> Vec<(usize, usize)> {
        // mark all vertices as unexplored
        let mut explored: HashSet<usize> = HashSet::with_capacity(self.adjacency_list.len());
        let mut sccs: HashMap<usize, usize> = HashMap::with_capacity(self.adjacency_list.len());
        let mut numSCC = 0;
        let mut sorted_order: Vec<(usize, usize)> =
            order.iter().map(|(key, value)| (*key, *value)).collect();
        sorted_order.sort_by(|a, b| a.1.cmp(&b.1));
        // println!("sorted[0]={:?}, sorted[1]={:?}", sorted_order[0], sorted_order[1]);

        // run through all vertices with outgoing edges in increasing order of its cur_label value
        for (v, label) in sorted_order {
            if !explored.contains(&v) {
                // for each vertex with outgoing edges in adjacency list, we increase its
                // SCC number (index) and run DFS for each such vertex to compute SCC it is part of
                numSCC += 1;
                self.dfs_scc(v, &mut explored, &mut numSCC, &mut sccs);
            }
        }

        // this sorts SCCs by amount of vertices among them
        // so sccs_sorted[0] would be the biggest SCC of the directed graph
        let mut sccs_sorted: Vec<(usize, usize)> = sccs
            .iter()
            // scc_num is just an SCC index in the map of SCCs
            // v_count is the number of vertices in each SCC
            .map(|(scc_num, v_count)| (*scc_num, *v_count))
            .collect();
        sccs_sorted.sort_by(|a, b| b.1.cmp(&a.1));

        sccs_sorted
    }

    pub fn dfs_scc(
        &self,
        s: usize,
        explored: &mut HashSet<usize>,
        numSCC: &mut usize,
        sccs: &mut HashMap<usize, usize>,
    ) {
        if !explored.contains(&s) {
            explored.insert(s);

            let count = sccs.entry(*numSCC).or_insert(0);
            *count += 1;

            if !self.adjacency_list.contains_key(&s) {
                return;
            } else {
                for adjacent_vertex in &self.adjacency_list[&s] {
                    self.dfs_scc(*adjacent_vertex, explored, numSCC, sccs);
                }
            }
        }
    }

    pub fn toposort_rec(&self) -> HashMap<usize, usize> {
        // mark all vertices as unexplored
        let mut explored: HashSet<usize> = HashSet::with_capacity(self.vertices.len());

        // let mut cur_label = self.adjacency_list.len();
        let mut cur_label = self.vertices.len();
        let mut order: HashMap<usize, usize> = HashMap::with_capacity(self.vertices.len());

        for v in self.adjacency_list.keys() {
            if !explored.contains(&v) {
                self.dfs_topo_rec(*v, &mut explored, &mut cur_label, &mut order);
            }
        }

        order
    }

    pub fn dfs_topo_rec(
        &self,
        s: usize,
        explored: &mut HashSet<usize>,
        cur_label: &mut usize,
        order: &mut HashMap<usize, usize>,
    ) {
        if !explored.contains(&s) {
            explored.insert(s);

            if !self.adjacency_list.contains_key(&s) {
                order.insert(s, *cur_label);
                *cur_label -= 1;
                return;
            } else {
                for adjacent_vertex in &self.adjacency_list[&s] {
                    self.dfs_topo_rec(*adjacent_vertex, explored, cur_label, order);
                }

                order.insert(s, *cur_label);
                *cur_label -= 1;
            }
        }
    }

    // pub fn toposort_iter(&self) -> HashMap<usize, usize> {
    //     // mark all vertices as unexplored
    //     let mut explored: HashSet<usize> = HashSet::with_capacity(self.vertices.len());

    //     // let mut cur_label = self.adjacency_list.len();
    //     let mut cur_label = self.vertices.len();
    //     let mut order: HashMap<usize, usize> = HashMap::with_capacity(self.vertices.len());

    //     for v in self.adjacency_list.keys() {
    //         if !explored.contains(&v) {
    //             self.dfs_topo_iter(*v, &mut explored, &mut cur_label, &mut order);
    //         }
    //     }

    //     order
    // }

    // pub fn dfs_topo_iter(
    //     &self,
    //     s: usize,
    //     explored: &mut HashSet<usize>,
    //     cur_label: &mut usize,
    //     order: &mut HashMap<usize, usize>
    // ) {
    //     // create a stack with vertex "s" as a starting point
    //     let mut stack: Vec<usize> = vec![s];

    //     while !stack.is_empty() {
    //         let v = stack.pop().unwrap();

    //         if !explored.contains(&v) {
    //             explored.insert(v);

    //             // order.insert(v, *cur_label);
    //             // *cur_label -= 1;

    //             if !self.adjacency_list.contains_key(&v) {
    //                 // order.insert(v, *cur_label);
    //                 // *cur_label -= 1;
    //                 return;
    //             }
    //             else {
    //                 for adjacent_vertex in &self.adjacency_list[&v] {
    //                     stack.push(*adjacent_vertex);
    //                 }
    //             }
    //         } else {
    //             if !order.contains_key(&v) {
    //                 order.insert(v, *cur_label);
    //                 *cur_label -= 1;
    //             }
    //         }
    //     }
    // }
}

// computes the cost of minimum spanning tree
type WeightedUndirectedAdjacencyList = HashMap<usize, HashSet<(usize, i64)>>;

pub fn prim_mst(
    weighted_adjacency_list: WeightedUndirectedAdjacencyList,
) -> WeightedUndirectedAdjacencyList {
    let mut X: HashSet<usize> = HashSet::new();
    let s = weighted_adjacency_list.iter().next().unwrap();
    // pick arbitrary vertex from the graph
    X.insert(*s.0);

    let mut T = WeightedUndirectedAdjacencyList::with_capacity(weighted_adjacency_list.len());

    while &X.len() != &weighted_adjacency_list.len() {
        let mut min_weight = i64::MAX;
        let mut a = *s.0;
        let mut b = *s.0;
        for v in X.iter() {
            a = *v;
            for (adjacent_vertex, weight) in &weighted_adjacency_list[&v] {
                if !X.contains(adjacent_vertex) {
                    if weight < &min_weight {
                        min_weight = *weight;
                        b = *adjacent_vertex;
                    }
                }
            }
        }

        X.insert(b);

        let edges = T.entry(a).or_insert(HashSet::<(usize, i64)>::new());
        edges.insert((b, min_weight));
    }

    println!("X.len()={}", X.len());
    println!("T.len()={}", T.len());
    println!("T={:?}", T);

    T
}

type WeightedUndirectedAdjacencyListKruskal = HashMap<usize, HashSet<(usize, usize)>>;
use petgraph::unionfind::UnionFind;
pub fn kruskal_mst(
    weighted_adjacency_list: WeightedUndirectedAdjacencyListKruskal,
) -> WeightedUndirectedAdjacencyListKruskal {
    let mut t =
        WeightedUndirectedAdjacencyListKruskal::with_capacity(weighted_adjacency_list.len());

    let mut edges = Vec::<(usize, usize, usize)>::with_capacity(weighted_adjacency_list.len());
    let mut vertices = HashSet::<usize>::new();
    for v in weighted_adjacency_list.keys() {
        vertices.insert(*v);
        for (v2, distance) in &weighted_adjacency_list[&v] {
            edges.push((*distance, *v, *v2));
            vertices.insert(*v2);
        }
    }

    // initially every vertex is its own disjoint subset until we cluster them
    println!(
        "edges.len()={}, vertices.len()={}",
        edges.len(),
        vertices.len()
    );
    let mut u = UnionFind::<usize>::new(vertices.len() + 1);

    // sort by non decreasing (increasing) order
    edges.sort_by(|a, b| a.0.cmp(&b.0));
    // println!("edges[..5]={:?}", &edges[..100]);

    for (dist, v1, v2) in edges {
        // println!("dist={}, v1={}, v2={}", dist, v1, v2);
        if !u.equiv(v1, v2) {
            let set = t.entry(v1).or_insert(HashSet::<(usize, usize)>::new());
            set.insert((v2, dist));
            u.union(v1, v2);
        }
    }

    t
}

pub fn kruskal_clustering(
    weighted_adjacency_list: WeightedUndirectedAdjacencyListKruskal,
    max_clusters: usize,
) -> Vec<(usize, usize, usize)> {
    let mut result = Vec::<(usize, usize, usize)>::with_capacity(weighted_adjacency_list.len());

    let mut edges = Vec::<(usize, usize, usize)>::with_capacity(weighted_adjacency_list.len());
    let mut vertices = HashSet::<usize>::new();
    for v in weighted_adjacency_list.keys() {
        vertices.insert(*v);
        for (v2, distance) in &weighted_adjacency_list[&v] {
            edges.push((*v, *v2, *distance));
            vertices.insert(*v2);
        }
    }

    // initially every vertex is its own disjoint subset until we cluster them
    println!(
        "edges.len()={}, vertices.len()={}",
        edges.len(),
        vertices.len()
    );
    let mut u = UnionFind::<usize>::new(vertices.len() + 1);

    // sort by non decreasing (increasing) order
    edges.sort_by(|a, b| a.2.cmp(&b.2));
    println!("edges[..5]={:?}", &edges[..5]);

    for (v1, v2, dist) in &edges {
        // println!("t.len()={}, edges.len() - max_clusters = {}", result.len(), vertices.len() - max_clusters);
        if result.len() == vertices.len() - max_clusters {
            println!("v1={}, v2={}, dist={}", v1, v2, dist);
            break;
        }
        // println!("dist={}, v1={}, v2={}", dist, v1, v2);
        if !u.equiv(*v1, *v2) {
            // let set = t.entry(*v1).or_insert(HashSet::<(usize, usize)>::new());
            // set.insert((*v2, *dist));
            result.push((*v1, *v2, *dist));
            u.union(*v1, *v2);
        }
    }

    for (v1, v2, dist) in &edges {
        if !u.equiv(*v1, *v2) {
            println!("v1={}, v2={}, dist={}", v1, v2, dist);
            break;
        }
    }

    for (v1, v2, dist) in &result {
        if vertices.contains(&v1) {
            vertices.remove(v1);
        }
        if vertices.contains(&v2) {
            vertices.remove(v2);
        }
    }

    println!("vertices={:?}", vertices);

    let clusters = u.into_labeling();
    println!("clusters={:?}", clusters);

    result
}

// pub fn breadth_first_search(adjacency_list: AdjacencyList, s: usize) {
//     let mut explored: HashSet<usize> = HashSet::new();
//     // mark vertex "s" as explored
//     explored.insert(s);

//     // create a queue with vertex "s" as a starting point
//     let mut deq = VecDeque::from([s]);

//     // traverse the graph starting from "s"
//     while !deq.is_empty() {
//         let v = deq.pop_front().unwrap();

//         // a vertex might not have adjacent vertices
//         if !adjacency_list.contains_key(&v) {
//             continue;
//         }

//         for adjacent_vertex in &adjacency_list[&v] {
//             if !explored.contains(&adjacent_vertex) {
//                 explored.insert(*adjacent_vertex);
//                 deq.push_back(*adjacent_vertex);
//             }
//         }
//     }
// }

pub fn dijkstra(weighted_adjacency_list: WeightedAdjacencyList, s: usize) -> HashMap<usize, usize> {
    let mut X: HashSet<usize> = HashSet::new();
    X.insert(s);

    // key = vertex, value = distance from s to vertex
    let mut lengths = initialize_lengths_to_1_000_000(&weighted_adjacency_list, s);

    let s_length = lengths.entry(s).or_insert(0);
    *s_length = 0;

    while X.len() != lengths.len() {
        let mut min_dist_from_s = 1_000_000;
        let mut min_vertex = s;

        for v in X.iter() {
            let v_length = lengths.get(&v).unwrap();
            for (adjacent_vertex, weight) in &weighted_adjacency_list[&v] {
                if !X.contains(adjacent_vertex) {
                    let dist_from_s = v_length + weight;
                    if dist_from_s < min_dist_from_s {
                        min_dist_from_s = dist_from_s;
                        min_vertex = *adjacent_vertex;
                    }
                }
            }
        }

        // println!(
        //     "Inserting min_vertex {} with distance from s {}",
        //     min_vertex, min_dist_from_s
        // );
        X.insert(min_vertex);
        lengths.insert(min_vertex, min_dist_from_s);
    }

    lengths
}

// initialize lengths to 1_000_000 via BFS
fn initialize_lengths_to_1_000_000(
    weighted_adjacency_list: &WeightedAdjacencyList,
    s: usize,
) -> HashMap<usize, usize> {
    let mut lengths: HashMap<usize, usize> = HashMap::new();

    let mut explored: HashSet<usize> = HashSet::new();
    explored.insert(s);

    let mut deq = VecDeque::from([s]);

    while !deq.is_empty() {
        let v = deq.pop_front().unwrap();
        lengths.insert(v, 1_000_000);
        if weighted_adjacency_list.contains_key(&v) {
            for (adjacent_vertex, weight) in &weighted_adjacency_list[&v] {
                if !explored.contains(&adjacent_vertex) {
                    explored.insert(*adjacent_vertex);
                    deq.push_back(*adjacent_vertex);
                }
            }
        }
    }

    lengths
}
