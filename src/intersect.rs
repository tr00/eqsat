// compute N-way set intersection for hash sets
use std::collections::HashSet;


pub fn intersect(sets: &mut Vec<HashSet<i32>>) -> HashSet<i32> {
    let mut result = HashSet::new();

    let n = sets.len();

    if n == 0 { return result; }

    // IDEA: semi-sort using simd circuits
    // IDEA: don't sort, just take min

    *sets = semisort(sets.to_vec(), 0, n); // sets.sort_unstable_by_key(|set| set.len());
    sets.swap(0, n - 1);

    let minset = sets.pop().unwrap();

    
    for value in minset {

        if sets.iter().all(|set| set.contains(&value)) {
            result.insert(value);
        }

    }

    return result;
}


// we use divide and conquer to find the minimum 
// but also shifting the min of the recursive calls to the left
pub fn semisort(mut vec: Vec<HashSet<i32>>, l: usize, r: usize) -> Vec<HashSet<i32>> {
    match r - l {
        0 => (),
        1 => (),
        2 => if vec[l].len() > vec[r - 1].len() { vec.swap(l, r - 1); },
        _ => {
            let m = l + (r - l) / 2;

            vec = semisort(vec, l, m);
            vec = semisort(vec, m, r);

            let min_l = vec[l].len();
            let min_r = vec[m].len();

            if min_l > min_r { vec.swap(l, m); }
        },
    };

    return vec.to_vec();
}


