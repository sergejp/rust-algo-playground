use std::cmp;
use std::collections::HashSet;

pub fn compute_mwis(weights: &Vec<usize>) -> HashSet<usize> {
    let wis = compute_weight_independent_set(weights);
    // println!("wis={:?}", wis);
    let mwis = reconstruct_mwis(wis, weights);
    // println!("mwis={:?}", mwis);
    mwis
}

fn compute_weight_independent_set(weights: &Vec<usize>) -> Vec<usize> {
    if weights.is_empty() {
        return vec![0];
    }
    if weights.len() == 1 {
        return vec![0, weights[0]];
    }

    let mut A = vec![0; weights.len() + 1];
    A[0] = 0;
    A[1] = weights[0];
    for i in 2..=weights.len() {
        A[i] = cmp::max(A[i - 1], A[i - 2] + weights[i - 1]);
    }

    A
}

fn reconstruct_mwis(wis: Vec<usize>, weights: &Vec<usize>) -> HashSet<usize> {
    if wis.len() < 2 {
        return HashSet::new();
    }

    let mut result = HashSet::<usize>::with_capacity(wis.len() - 1);
    let mut i = weights.len();
    while i >= 2 {
        if wis[i - 1] >= wis[i] {
            i -= 1;
        } else {
            result.insert(weights[i - 1]);
            i -= 2;
        }
    }

    if i == 1 {
        result.insert(weights[i - 1]);
    }

    result
}
