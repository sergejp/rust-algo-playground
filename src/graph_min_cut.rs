use rand::{thread_rng, Rng};
use std::collections::HashMap;

// struct Graph {
//     V: HashMap<usize, Vec<usize>>,
//     E: HashMap<usize, Vec<usize>>,
// }

pub fn min_cut(mut graph: HashMap<usize, Vec<usize>>) -> usize {
    let mut rng = thread_rng();
    while graph.len() > 2 {
        let mut v1_index: usize = rng.gen_range(0..graph.len());
        let mut counter = 0usize;
        for (key, value) in &graph {
            if counter == v1_index {
                let v1 = *key;
                let mut v2_index: usize;
                if graph[&v1].len() == 0 {
                    break;
                } else if graph[&v1].len() == 1 {
                    v2_index = 0;
                } else {
                    v2_index = rng.gen_range(0..graph[&v1].len());
                }

                let v2 = graph[&v1][v2_index];

                // println!("{} with v2={}", graph.get_mut(&v1).unwrap().len(), v2);
                let count = graph.get_mut(&v1).unwrap().len();
                graph.get_mut(&v1).unwrap().retain(|value| value != &v2);
                // println!("{} without v2={}", graph.get_mut(&v1).unwrap().len(), &v2);

                // remove v2 from the hash map
                if let Some(mut v2_adjacent_vertices) = graph.remove(&v2) {
                    for v2_adjacent_vertex in &v2_adjacent_vertices {
                        let mut a: Vec<usize> = graph
                            .get_mut(&v2_adjacent_vertex)
                            .unwrap()
                            .iter()
                            .map(|item| {
                                if item == &v2 {
                                    return v1;
                                }
                                *item
                            })
                            .collect();
                        // assert!(graph.get_mut(&v2_adjacent_vertex).unwrap() == &mut a);
                        // println!("graph {:?}", graph.get_mut(&v2_adjacent_vertex).unwrap());
                        // println!("a {:?}", a);
                        // println!("v1={}, v2={}", v1, v2);
                        // println!("=====================================");
                        *graph.get_mut(&v2_adjacent_vertex).unwrap() = a;
                    }

                    // add v2 adjacent vertices to v1 adjacent vertices
                    // println!("BEFORE");
                    // println!("graph.get_mut(&v1).unwrap().len()={}", graph.get_mut(&v1).unwrap().len());
                    graph
                        .get_mut(&v1)
                        .unwrap()
                        .append(&mut v2_adjacent_vertices);
                    // println!("graph.get_mut(&v1).unwrap().len()={}", graph.get_mut(&v1).unwrap().len());
                    // println!("AFTER");

                    // remove self-loops from v1 (i.e. if there is a v1-v1)
                    // println!("BEFORE");
                    // println!("graph.get_mut(&v1).unwrap().len()={}", graph.get_mut(&v1).unwrap().len());
                    graph.get_mut(&v1).unwrap().retain(|value| value != &v1);
                    // println!("graph.get_mut(&v1).unwrap().len()={}", graph.get_mut(&v1).unwrap().len());
                    // println!("AFTER");
                }
                break;
            }
            counter += 1;
        }
    }

    let result = graph
        .iter()
        .fold(0, |acc, (i, vertices)| acc + vertices.len());
    result / 2
}
