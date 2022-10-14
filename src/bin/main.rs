use algorithms::huffman::BinaryTree;
use algorithms::{graph, heap, huffman, merge_sort, mwis, quick_sort, selection_sort};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::thread::Builder;
use std::time::{Duration, Instant};
use std::{cmp, fs};

use petgraph::algo::{dijkstra, min_spanning_tree, toposort};
use petgraph::data::FromElements;
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::unionfind::UnionFind;
use petgraph::Graph;

use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut vec: Vec<i64> = (1..=1_000_000).collect();
    vec.shuffle(&mut thread_rng());
    let start = Instant::now();
    let count = 100;
    for i in 1..=count {
        vec.sort();
    }
    let duration = start.elapsed() / count;
    println!("Sorting an array took: {:?}", duration);
}

// fn main() {
// compute_weight_independent_set();
// compute_huffman();
// compute_clusters_big();
// compute_clusters();
// compute_prim_mst();
// compute_job_schedule();
// compute_two_sum();
// compute_median_maintainance();
// compute_dijkstra();

// compute_strongly_connected_components_of_graph();
// compute_strongly_connected_components_of_graph_petgraph();

// compute_graph_min_cut();
// if numbers.is_empty() {
//     println!("No numbers were parsed");
// }

// selection_sort();
// heap_sort();
// merge_sort();
// quick_sort();

// let mut arr = [0, 5, 2, 9, 7];
// println!("Unsorted: {:?}", arr);
// insertion_sort::insertion_sort(&mut arr);
// println!("Insertion Sort: {:?}", arr);

// let mut arr = [0, 5, 2, 9, 7];
// println!("Unsorted: {:?}", arr);
// merge_sort::merge_sort(&mut arr);
// println!("Merge Sort: {:?}", arr);

// let result = rec_mul::rec_mul("98765432", "12345678");
// println!("98765432 * 12345678 = {}", result);
// let result = rec_mul::rec_mul(
//     "3141592653589793238462643383279502884197169399375105820974944592",
//     "2718281828459045235360287471352662497757247093699959574966967627"
// );
// println!("3141592653589793238462643383279502884197169399375105820974944592 * 2718281828459045235360287471352662497757247093699959574966967627 = {}", result);

// let arr = [37, 58, 10, 4, 27, 88, 99];
// let num = second_largest(&arr);
// println!("Second largest: {}", num);

// let mut v1: Vec<u32> = (0..=72834).collect();
// let mut v2: Vec<u32> = (7032..=9999).rev().collect();
// v1.append(&mut v2);
// if let Some(result) = unimodal_search(&v1) {
//     println!("Largest num: {}", result);
// } else {
//     println!("Couldn't compute largest num");
// }

// let contents = fs::read_to_string("_bcb5c6658381416d19b01bfc1d3993b5_IntegerArray.txt")
//     .expect("Something went wrong reading the file");
// let numbers: Vec<u64> = contents
//     .lines()
//     .map(|line| line.parse::<u64>().unwrap())
//     .collect();
// if numbers.is_empty() {
//     println!("No numbers were parsed");
// }
// let (sorted, split_inv) = count_inversions::sort_and_count_inv(&numbers);
// assert_eq!(split_inv, 2407905288, "Number of split inversions is {}, but should be 2407905288", split_inv);

// let contents = fs::read_to_string("_QuickSort_data.txt")
//     .expect("Something went wrong reading the file");
// let mut numbers: Vec<u64> = contents
//     .lines()
//     .map(|line| line.parse::<u64>().unwrap())
//     .collect();
// if numbers.is_empty() {
//     println!("No numbers were parsed");
// }

// let mut numbers = [4, 5, 1, 3, 2];
// let len = numbers.len();
// quick_sort::quick_sort(&mut numbers, 0, len - 1);
// }

fn compute_weight_independent_set() {
    let contents = fs::read_to_string("_790eb8b186eefb5b63d0bf38b5096873_mwis.txt")
        .expect("Something went wrong reading the file");

    let number_of_vertices: usize = contents.lines().next().unwrap().parse().unwrap();
    println!("Number of vertices: {}", number_of_vertices);

    // let mut weighted_adjacency_list: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
    let mut weights = Vec::<usize>::with_capacity(number_of_vertices);
    for line in contents.lines().skip(1) {
        let weight: usize = line.parse().unwrap();
        weights.push(weight);
    }
    assert_eq!(weights.len(), number_of_vertices);

    let start = Instant::now();
    let mwis = mwis::compute_mwis(&weights);
    let duration = start.elapsed();
    println!("Computing Weight Independent Set took: {:?}", duration);

    let mut coursera_answer = 0b00000000;
    let question_vertices = [
        (1, 0b10000000),
        (2, 0b01000000),
        (3, 0b00100000),
        (4, 0b00010000),
        (17, 0b00001000),
        (117, 0b00000100),
        (517, 0b00000010),
        (997, 0b00000001),
    ];
    for (v_index, bit_mask) in question_vertices {
        let weight = weights[v_index - 1];
        if mwis.contains(&weight) {
            coursera_answer ^= bit_mask;
        }
    }
    println!("coursera_answer = {:0b}", coursera_answer);
}

fn compute_huffman() {
    let contents = fs::read_to_string("_eed1bd08e2fa58bbe94b24c06a20dcdb_huffman.txt")
        .expect("Something went wrong reading the file");

    let number_of_symbols: usize = contents.lines().next().unwrap().parse().unwrap();
    println!("Number of symbols: {}", number_of_symbols);

    // let mut weighted_adjacency_list: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
    let mut symbol_freqs = Vec::<usize>::with_capacity(number_of_symbols);
    for line in contents.lines().skip(1) {
        let frequency: usize = line.parse().unwrap();
        symbol_freqs.push(frequency);
    }
    assert_eq!(symbol_freqs.len(), number_of_symbols);

    let start = Instant::now();
    let mut code_tree = huffman::compute_code_tree(&symbol_freqs);
    let duration = start.elapsed();
    println!("Computing Huffman codes took: {:?}", duration);

    let start = Instant::now();
    let (min, max) = huffman_longest_codeword_search(code_tree);
    let duration = start.elapsed();
    println!("Computing min and max codeword length took: {:?}", duration);

    println!("min={}, max={}", min, max);
}

fn huffman_longest_codeword_search(tree: BinaryTree<usize>) -> (usize, usize) {
    let mut explored = HashSet::<usize>::new();
    explored.insert(tree.value);

    let mut code_lengths = HashMap::<usize, usize>::new();
    code_lengths.insert(tree.value, 0);

    // create a queue with vertex "s" as a starting point
    let mut deq = VecDeque::from([tree]);

    let mut min_codeword = usize::MAX;
    // traverse the graph starting from "s"
    while !deq.is_empty() {
        let tree = deq.pop_front().unwrap();
        let parent_code_length = code_lengths.get(&tree.value).unwrap().clone();
        // println!("parent_code_length={}", parent_code_length);

        // end leaf of a tree, so we need to record its codeword length
        if tree.left == None && tree.right == None {
            min_codeword = cmp::min(min_codeword, parent_code_length);
        }

        // a vertex might not have adjacent vertices
        if let Some(node) = tree.left {
            if !explored.contains(&node.value) {
                code_lengths.insert(node.value.clone(), parent_code_length + 1);
                explored.insert(node.value);
                deq.push_back(*node);
            }
        }
        if let Some(node) = tree.right {
            if !explored.contains(&node.value) {
                code_lengths.insert(node.value.clone(), parent_code_length + 1);
                explored.insert(node.value);
                deq.push_back(*node);
            }
        }
    }

    let mut vec: Vec<usize> = code_lengths.into_values().collect();
    vec.sort();
    (min_codeword, vec.pop().unwrap())
}

fn compute_clusters_big() {
    let contents = fs::read_to_string("_fe8d0202cd20a808db6a4d5d06be62f4_clustering_big.txt")
        .expect("Something went wrong reading the file");

    let metainfo: Vec<&str> = contents
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .collect();
    let number_of_nodes = metainfo[0].parse::<usize>().unwrap();
    let number_of_bits_per_node = metainfo[1].parse::<usize>().unwrap();
    println!("Number of nodes: {}", number_of_nodes);
    println!("Number of bits per node: {}", number_of_bits_per_node);

    // let mut weighted_adjacency_list: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
    let mut binary_nodes = Vec::<i32>::with_capacity(number_of_nodes);
    let mut binary_nodes_map = HashMap::<i32, Vec<usize>>::with_capacity(number_of_nodes);
    let mut index = 0;
    for line in contents.lines().skip(1) {
        let vertex_parts: String = line.split_whitespace().collect();
        if vertex_parts.is_empty() {
            continue;
        }

        let b = i32::from_str_radix(&vertex_parts, 2).expect("Not a binary number!");
        binary_nodes.push(b);

        let node_indexes = binary_nodes_map.entry(b).or_insert(Vec::<usize>::new());
        node_indexes.push(index);

        index += 1;
    }

    let mut bit_masks = Vec::<i32>::with_capacity(301);
    bit_masks.push(0);
    let mut hash_set = HashSet::<i32>::new();
    hash_set.insert(0);
    for i in 0..24 {
        let a = 1 << i;
        println!("i={}, a={}", i, a);
        bit_masks.push(a);
        hash_set.insert(a);
        for j in (i + 1)..24 {
            let b = 1 << j;
            bit_masks.push(a ^ b);
            hash_set.insert(a ^ b);
        }
    }
    // println!("bit_masks {:?}", bit_masks);
    assert_eq!(hash_set.len(), bit_masks.len());

    // println!("binary_nodes_map.len() {}", binary_nodes_map.len());

    let mut u = UnionFind::<usize>::new(binary_nodes.len());
    for node in binary_nodes_map.keys() {
        // cluster 0-distance nodes
        // let node_indexes = binary_nodes_map.get(node).unwrap();
        // let first_node = node_indexes[0]; //always exist, leader node
        // // merge leader with its own
        // for i in 1..node_indexes.len() {
        //     u.union(first_node, node_indexes[i]);
        // }

        // cluster 1- and 2-distance nodes
        for bit_mask in &bit_masks {
            // 0 1 0 0 1 0 1 0 1 0 1 1 0 0 1 1 1 1 1 0 0 1 0 0
            // ^
            // 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
            // 1 1 0 0 1 0 1 0 1 0 1 1 0 0 1 1 1 1 1 0 0 1 0 0 (XOR)
            let neighbour = node ^ bit_mask;
            if binary_nodes_map.contains_key(&neighbour) {
                let node_indexes = binary_nodes_map.get(node).unwrap();
                let first_node = node_indexes[0]; //always exist, leader node
                                                  // merge leader with its own
                for i in 1..node_indexes.len() {
                    u.union(first_node, node_indexes[i]);
                }
                let neighbour_indexes = binary_nodes_map.get(&neighbour).unwrap();
                for i in 0..neighbour_indexes.len() {
                    u.union(first_node, neighbour_indexes[i]);
                }
            }
        }
    }

    let clusters = u.into_labeling();
    println!("clusters.len()={}", clusters.len());
    let clusters_set = HashSet::<usize>::from_iter(clusters.iter().cloned());
    println!("clusters_set.len()={}", clusters_set.len());
    // let ones = (binary_nodes[1] ^ binary_nodes[2]).count_ones();
    // println!("{:?}", ones);

    // println!("binary_nodes_map {:?}", binary_nodes_map);
}

fn compute_clusters() {
    let contents = fs::read_to_string("_fe8d0202cd20a808db6a4d5d06be62f4_clustering1.txt")
        .expect("Something went wrong reading the file");
    let number_of_nodes: usize = contents.lines().next().unwrap().parse().unwrap();
    println!("Number of nodes: {}", number_of_nodes);

    let mut weighted_adjacency_list: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
    for line in contents.lines().skip(1) {
        let adjacent_vertices_str: Vec<&str> = line.split_whitespace().collect();
        if adjacent_vertices_str.is_empty() {
            continue;
        }

        let v1: usize = adjacent_vertices_str[0].parse().unwrap();
        let v2: usize = adjacent_vertices_str[1].parse().unwrap();
        let distance: usize = adjacent_vertices_str[2].parse().unwrap();

        // insert v2 with edge into v1 connections as the graph is undirected
        let v1_connections = weighted_adjacency_list
            .entry(v1)
            .or_insert(HashSet::<(usize, usize)>::new());
        v1_connections.insert((v2, distance));

        // insert v1 with edge into v2 connections as the graph is undirected
        let v2_connections = weighted_adjacency_list
            .entry(v2)
            .or_insert(HashSet::<(usize, usize)>::new());
        v2_connections.insert((v1, distance));
    }

    // println!("weighted_adjacency_list = {:?}", weighted_adjacency_list);
    let cluster = graph::kruskal_clustering(weighted_adjacency_list, 4);
    // println!("mst={:?}", mst);
    // answer = 106
}

fn compute_prim_mst() {
    let contents = fs::read_to_string("_d4f3531eac1d289525141e95a2fea52f_edges1.txt")
        .expect("Something went wrong reading the file");
    let metainfo: Vec<&str> = contents
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .collect();
    let number_of_nodes = metainfo[0].parse::<usize>().unwrap();
    let number_of_edges = metainfo[1].parse::<usize>().unwrap();
    println!("Number of nodes: {}", number_of_nodes);
    println!("Number of edges: {}", number_of_edges);

    let mut weighted_adjacency_list: HashMap<usize, HashSet<(usize, i64)>> = HashMap::new();
    for line in contents.lines().skip(1) {
        let adjacent_vertices_str: Vec<&str> = line.split_whitespace().collect();

        let v1: usize = adjacent_vertices_str[0].parse().unwrap();
        let v2: usize = adjacent_vertices_str[1].parse().unwrap();
        let edge_weight: i64 = adjacent_vertices_str[2].parse().unwrap();

        // insert v2 with edge into v1 connections as the graph is undirected
        let v1_connections = weighted_adjacency_list
            .entry(v1)
            .or_insert(HashSet::<(usize, i64)>::new());
        v1_connections.insert((v2, edge_weight));

        // insert v1 with edge into v2 connections as the graph is undirected
        let v2_connections = weighted_adjacency_list
            .entry(v2)
            .or_insert(HashSet::<(usize, i64)>::new());
        v2_connections.insert((v1, edge_weight));
    }

    println!(
        "weighted_adjacency_list = {:?}",
        weighted_adjacency_list[&2]
    );
    let mst = graph::prim_mst(weighted_adjacency_list);
    let mut acc = 0;
    for (v, edges) in &mst {
        println!("v={:?}", v);
        println!("edges={:?}", edges);
        for (v, weight) in edges {
            acc += weight;
        }
    }
    println!("total mst cost = {}", acc);
}

fn compute_job_schedule() {
    let contents = fs::read_to_string("_642c2ce8f3abe387bdff636d708cdb26_jobs.txt")
        .expect("Something went wrong reading the file");
    let number_of_jobs: usize = contents.lines().next().unwrap().parse::<usize>().unwrap();
    let mut jobs: Vec<(i64, i64)> = contents
        .lines()
        .skip(1)
        .map(|line| {
            let job_info: Vec<&str> = line.split_whitespace().collect();
            let job_weight: i64 = job_info[0].parse().unwrap();
            let job_length: i64 = job_info[1].parse().unwrap();
            (job_weight, job_length)
        })
        .collect();
    if jobs.is_empty() {
        println!("No numbers were parsed");
    }

    jobs.sort_by(|a, b| {
        let a_order: f64 = (a.0 as f64) / (a.1 as f64);
        let b_order: f64 = (b.0 as f64) / (b.1 as f64);
        if a_order <= b_order {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    let mut sum_of_weighted_completion_times = 0;
    let mut completion_time = 0;
    for job in &jobs {
        completion_time += job.1;
        sum_of_weighted_completion_times += job.0 * completion_time;
    }

    assert!(jobs.len() == number_of_jobs);
    println!("jobs[0]={:?}", jobs[0]);
    println!("jobs[last]={:?}", jobs[jobs.len() - 1]);
    println!("jobs[..5]={:?}", &jobs[..20]);
    println!(
        "sum_of_weighted_completion_times={}",
        sum_of_weighted_completion_times
    );
}

fn compute_two_sum() {
    let contents =
        fs::read_to_string("_6ec67df2804ff4b58ab21c12edcb21f8_algo1-programming_prob-2sum.txt")
            .expect("Something went wrong reading the file");
    let mut numbers: Vec<i64> = contents
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    if numbers.is_empty() {
        println!("No numbers were parsed");
    }

    println!("first num is {}", numbers[0]);

    let start = Instant::now();
    let mut all_numbers = HashMap::<i64, i64>::with_capacity(numbers.len());
    for number in &numbers {
        all_numbers.insert(*number, *number);
    }

    let mut targets_set = HashSet::<i64>::with_capacity(20001);
    let mut targets_vec = Vec::<i64>::with_capacity(20001);
    for t in -10000..=10000 {
        println!("t={}", t);
        for x in &numbers {
            let y = t - *x;
            if y != *x && all_numbers.contains_key(&y) {
                targets_set.insert(t);
                targets_vec.push(t);
                break;
            }
        }
    }

    // println!("first hash map number {:?}", all_numbers.get(&numbers[0]));
    println!(
        "targets set length={}, targets arr length={}",
        targets_set.len(),
        targets_vec.len()
    );
    let duration = start.elapsed();
    println!("Computing two=sum: {:?}", duration);
}

fn compute_median_maintainance() {
    let contents = fs::read_to_string("_6ec67df2804ff4b58ab21c12edcb21f8_Median.txt")
        .expect("Something went wrong reading the file");
    let mut numbers: Vec<u64> = contents
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    if numbers.is_empty() {
        println!("No numbers were parsed");
    }

    let start = Instant::now();
    let medians = heap::compute_median_stream(&mut numbers);
    let duration = start.elapsed();
    println!("Computing stream of medians: {:?}", duration);
}

fn selection_sort() {
    let contents = fs::read_to_string("_bcb5c6658381416d19b01bfc1d3993b5_IntegerArray.txt")
        .expect("Something went wrong reading the file");
    let mut numbers: Vec<u64> = contents
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    if numbers.is_empty() {
        println!("No numbers were parsed");
    }

    let start = Instant::now();
    selection_sort::selection_sort(&mut numbers);
    let duration = start.elapsed();
    println!("Selection sort: {:?}", duration);
}

fn heap_sort() {
    let contents = fs::read_to_string("_bcb5c6658381416d19b01bfc1d3993b5_IntegerArray.txt")
        .expect("Something went wrong reading the file");

    let mut numbers: Vec<u64> = contents
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    if numbers.is_empty() {
        println!("No numbers were parsed");
    }

    let start = Instant::now();
    let sorted = heap::heap_sort(&mut numbers);
    let duration = start.elapsed();
    println!("Heap sort: {:?}, {:?}", duration, &sorted[..30]);
}

fn merge_sort() {
    let contents = fs::read_to_string("_bcb5c6658381416d19b01bfc1d3993b5_IntegerArray.txt")
        .expect("Something went wrong reading the file");

    let mut numbers: Vec<u64> = contents
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    if numbers.is_empty() {
        println!("No numbers were parsed");
    }

    let start = Instant::now();
    merge_sort::merge_sort(&mut numbers);
    let duration = start.elapsed();
    println!("Merge sort: {:?}, {:?}", duration, &numbers[..30]);
}

fn quick_sort() {
    let contents = fs::read_to_string("_bcb5c6658381416d19b01bfc1d3993b5_IntegerArray.txt")
        .expect("Something went wrong reading the file");

    let mut numbers: Vec<u64> = contents
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();
    if numbers.is_empty() {
        println!("No numbers were parsed");
    }

    let start = Instant::now();
    let len = numbers.len();
    quick_sort::quick_sort(&mut numbers, 0, len - 1);
    let duration = start.elapsed();
    println!("Quick sort: {:?}, {:?}", duration, &numbers[..30]);
}

fn compute_dijkstra() {
    let contents = fs::read_to_string("_dcf1d02570e57d23ab526b1e33ba6f12_dijkstraData.txt")
        .expect("Something went wrong reading the file");
    let mut weighted_adjacency_list: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
    for line in contents.lines() {
        let adjacent_vertices_str: Vec<&str> = line.split_whitespace().collect();
        // println!(
        //     "adjacent_vertices_str.len()={}",
        //     adjacent_vertices_str.len()
        // );

        let vertex_weight_pair: Vec<&str> = adjacent_vertices_str[0].split(",").collect();
        let index: usize = vertex_weight_pair[0].parse().unwrap();
        // println!("index={}", index);

        let mut set: HashSet<(usize, usize)> =
            HashSet::with_capacity(adjacent_vertices_str.len() - 1);
        for i in 1..adjacent_vertices_str.len() {
            let vertex_weight_pair: Vec<&str> = adjacent_vertices_str[i].split(",").collect();
            let vertex: usize = vertex_weight_pair[0].parse().unwrap();
            let weight = vertex_weight_pair[1].parse().unwrap();
            // println!("Inserting vertex={}, weight={}", vertex, weight);
            set.insert((vertex, weight));
        }
        // println!("set.len()={}", set.len());
        // println!("vec.len()={}", vec.len());
        // graph.insert(index, vec);
        weighted_adjacency_list.insert(index, set);
    }

    println!(
        "Imported adjacency list length={}",
        weighted_adjacency_list.len()
    );

    let start = Instant::now();
    let shortest_paths = graph::dijkstra(weighted_adjacency_list, 1);
    let duration = start.elapsed();
    // 2599,2610,2947,2052,2367,2399,2029,2442,2505,3068
    println!(
        "Dijkstra shortest paths computed in {:?}: {},{},{},{},{},{},{},{},{},{}",
        duration,
        shortest_paths[&7],
        shortest_paths[&37],
        shortest_paths[&59],
        shortest_paths[&82],
        shortest_paths[&99],
        shortest_paths[&115],
        shortest_paths[&133],
        shortest_paths[&165],
        shortest_paths[&188],
        shortest_paths[&197]
    );
}

fn compute_strongly_connected_components_of_graph_petgraph() {
    let (adjacency_list_rev, vertices_rev, edges_rev) = collect_graph_entries_u32(true);
    let gr: Graph<(), ()> = Graph::from_edges(&edges_rev);

    // let mut sccs = petgraph::algo::kosaraju_scc(&gr);
    // sccs.sort_by(|a, b| b.len().cmp(&a.len()));
    // println!("sccs[0]={}, sccs[1]={}, sccs[2]={}, sccs[3]={}, sccs[4]={}", sccs[0].len(), sccs[1].len(), sccs[2].len(), sccs[3].len(), sccs[4].len());

    let result = toposort(&gr, None);
    println!("Toposort result {:?}", result);
}

fn collect_graph_entries_u32(
    reversed: bool,
) -> (HashMap<u32, HashSet<u32>>, HashSet<u32>, Vec<(u32, u32)>) {
    let start = Instant::now();
    let contents =
        fs::read_to_string("graph_edges_SCC.txt").expect("Something went wrong reading the file");
    let duration = start.elapsed();
    println!("{:?} fs::read_to_string graph_edges_SCC.txt", duration);

    let start = Instant::now();
    let mut edges_count = 0;
    for line in contents.lines() {
        edges_count += 1;
    }
    let duration = start.elapsed();
    println!("{:?} computing number of edges", duration);

    let start = Instant::now();
    let mut graph: HashMap<u32, HashSet<u32>> = HashMap::with_capacity(edges_count);
    let mut vertices = HashSet::<u32>::with_capacity(edges_count);
    let mut edges = Vec::<(u32, u32)>::with_capacity(edges_count);
    let duration = start.elapsed();
    println!(
        "{:?} allocating graph and vertices hashmap and hashset with {} elements",
        duration, edges_count
    );

    let start = Instant::now();
    for line in contents.lines() {
        let adjacent_vertices_str: Vec<&str> = line.split_whitespace().collect();
        let first_index = if reversed { 1 } else { 0 };
        let second_index = if reversed { 0 } else { 1 };
        let v1: u32 = adjacent_vertices_str[first_index].parse().unwrap();
        let v2: u32 = adjacent_vertices_str[second_index].parse().unwrap();

        let vec = graph.entry(v1).or_insert(HashSet::with_capacity(3));
        vec.insert(v2);

        vertices.insert(v1);
        vertices.insert(v2);

        edges.push((v1, v2));
    }
    let duration = start.elapsed();
    println!("{:?} parsing {} edges", duration, edges_count);

    (graph, vertices, edges)
}

fn compute_strongly_connected_components_of_graph() {
    let (adjacency_list, vertices, edges) = collect_graph_entries(false);
    let (adjacency_list_rev, vertices_rev, edges_rev) = collect_graph_entries(true);

    let graph = graph::Graph::from(adjacency_list, vertices, edges);
    let graph_rev = graph::Graph::from(adjacency_list_rev, vertices_rev, edges_rev);
    let builder = Builder::new()
        .name("reductor".into())
        .stack_size(100 * 1024 * 1024); // 32MB of stack space

    let handler = builder
        .spawn(move || {
            // stack-intensive operations
            let order = graph_rev.toposort_rec();
            let sccs = graph.kosaraju_scc(&order);
            println!("top 5 sccs = {:?}", &sccs[..5]);
            let mut max = 0;
            for scc in sccs {
                max = cmp::max(max, scc.1);
            }
            println!("Top 1 scc={}", max);
        })
        .unwrap();

    handler.join().unwrap();
}

fn collect_graph_entries(
    reversed: bool,
) -> (
    HashMap<usize, HashSet<usize>>,
    HashSet<usize>,
    Vec<(usize, usize)>,
) {
    let start = Instant::now();
    let contents =
        fs::read_to_string("graph_edges_SCC.txt").expect("Something went wrong reading the file");
    let duration = start.elapsed();
    println!("{:?} fs::read_to_string graph_edges_SCC.txt", duration);

    let start = Instant::now();
    let mut edges_count = 0;
    for line in contents.lines() {
        edges_count += 1;
    }
    let duration = start.elapsed();
    println!("{:?} computing number of edges", duration);

    let start = Instant::now();
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::with_capacity(edges_count);
    let mut vertices = HashSet::<usize>::with_capacity(edges_count);
    let mut edges = Vec::<(usize, usize)>::with_capacity(edges_count);
    let duration = start.elapsed();
    println!(
        "{:?} allocating graph and vertices hashmap and hashset with {} elements",
        duration, edges_count
    );

    let start = Instant::now();
    for line in contents.lines() {
        let adjacent_vertices_str: Vec<&str> = line.split_whitespace().collect();
        let first_index = if reversed { 1 } else { 0 };
        let second_index = if reversed { 0 } else { 1 };
        let v1: usize = adjacent_vertices_str[first_index].parse().unwrap();
        let v2: usize = adjacent_vertices_str[second_index].parse().unwrap();

        let vec = graph.entry(v1).or_insert(HashSet::with_capacity(3));
        vec.insert(v2);

        vertices.insert(v1);
        vertices.insert(v2);

        edges.push((v1, v2));
    }
    let duration = start.elapsed();
    println!("{:?} parsing {} edges", duration, edges_count);

    (graph, vertices, edges)
}

// fn compute_graph_min_cut() {
//     let contents =
//     fs::read_to_string("_kargerMinCut.txt").expect("Something went wrong reading the file");
//     let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
//     for line in contents.lines() {
//         let adjacent_vertices_str: Vec<&str> = line.split_whitespace().collect();
//         // println!("adjacent_vertices_str.len()={}", adjacent_vertices_str.len());
//         let index = adjacent_vertices_str[0].parse().unwrap();
//         let mut vec: Vec<usize> = Vec::with_capacity(adjacent_vertices_str.len() - 1);
//         for vertex in 1..adjacent_vertices_str.len() {
//             vec.push(adjacent_vertices_str[vertex].parse().unwrap());
//         }
//         // println!("vec.len()={}", vec.len());
//         graph.insert(index, vec);
//     }

//     let mut min_count = usize::MAX;
//     for i in 0..500 {
//         let tmp = graph.clone();
//         min_count = cmp::min(graph_min_cut::min_cut(tmp), min_count);
//     }
//     // let min_cut = graph_min_cut::min_cut(graph);
//     println!("Min cut for graph is: {}", min_count);
// }

fn unimodal_search(arr: &[u32]) -> Option<u32> {
    println!("unimodal_search arr.len() {}", arr.len());

    if arr.is_empty() {
        return None;
    }
    if arr.len() == 1 {
        return Some(arr[0]);
    }
    if arr.len() == 2 {
        if arr[0] > arr[1] {
            return Some(arr[0]);
        } else {
            return Some(arr[1]);
        }
    }

    let mid = arr.len() / 2;
    if arr[mid] > arr[mid - 1] && arr[mid] > arr[mid + 1] {
        return Some(arr[mid]);
    }

    if arr[mid] > arr[mid - 1] {
        unimodal_search(&arr[mid..])
    } else {
        unimodal_search(&arr[..mid])
    }
}

fn second_largest(arr: &[u32]) -> u32 {
    assert!(arr.len() > 1, "Array should have a length of 2 or more");
    let mut first_largest = arr[0];
    let mut second_largest = first_largest;
    for (index, item) in arr.iter().enumerate() {
        println!("index: {}", index);
        if item > &first_largest {
            second_largest = first_largest;
            first_largest = *item;
        }
    }
    second_largest
}
