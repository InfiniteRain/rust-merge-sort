fn main() {
    let vec_to_sort = vec![-3, 1, 3, -4, 1, 2, 3, 1, 10, 16, -10, -76, 12, 13];

    let sorted = merge_sort(&vec_to_sort);

    println!("{:?} => {:?}", vec_to_sort, sorted);
}

fn merge(first_vec: &Vec<i32>, second_vec: &Vec<i32>) -> Vec<i32> {
    let mut merged_vec = Vec::with_capacity(first_vec.len() + second_vec.len());
    let mut first_vec_clone = first_vec.clone();
    let mut second_vec_clone = second_vec.clone();

    while first_vec_clone.len() > 0 && second_vec_clone.len() > 0 {
        if first_vec_clone[0] < second_vec_clone[0] {
            merged_vec.push(first_vec_clone[0]);
            first_vec_clone.remove(0);
            continue;
        }

        merged_vec.push(second_vec_clone[0]);
        second_vec_clone.remove(0);
    }

    merged_vec.append(&mut first_vec_clone);
    merged_vec.append(&mut second_vec_clone);

    merged_vec
}

fn merge_sort(vector: &Vec<i32>) -> Vec<i32> {
    let vector_length = vector.len();
    let central_index = vector_length / 2;

    if vector_length < 2 {
        return vector.clone();
    }

    let left_side = (&vector[0..central_index]).to_vec();
    let right_side = (&vector[central_index..vector_length]).to_vec();

    let left_sorted = merge_sort(&left_side);
    let right_sorted = merge_sort(&right_side);

    merge(&left_sorted, &right_sorted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let test1 = merge(&vec![1, 3, 5, 7, 9], &vec![2, 4, 6, 8, 10]);
        let test2 = merge(&vec![1, 5, 7], &vec![1, 2, 3, 4, 5]);
        let test3 = merge(&vec![], &vec![1]);
        let test4 = merge(&vec![1], &vec![]);
        let test5 = merge(&vec![], &vec![]);

        assert_eq!(test1, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(test2, vec![1, 1, 2, 3, 4, 5, 5, 7]);
        assert_eq!(test3, vec![1]);
        assert_eq!(test4, vec![1]);
        assert_eq!(test5, vec![]);
    }

    #[test]
    fn test_merge_sort() {
        let test1 = merge_sort(&vec![1, 2, 5, -2, -3, 1, 4, 1, 2, 3]);
        let test2 = merge_sort(&vec![2, 2, 1, 3, 5, 1, 2, 3, 4, 1, 10, 1]);
        let test3 = merge_sort(&vec![1]);
        let test4 = merge_sort(&vec![]);

        assert_eq!(test1, vec![-3, -2, 1, 1, 1, 2, 2, 3, 4, 5]);
        assert_eq!(test2, vec![1, 1, 1, 1, 2, 2, 2, 3, 3, 4, 5, 10]);
        assert_eq!(test3, vec![1]);
        assert_eq!(test4, vec![]);
    }
}
