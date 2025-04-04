/// It takes the ownership!
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn fill_vec_non_moving(vec_ref: &[i32]) -> Vec<i32> {
    let mut new_vec = vec_ref.to_owned();
    new_vec.push(88);
    new_vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);

        // Custom implementation...
        let vec01 = vec![22, 44, 66];
        let vec02 = fill_vec_non_moving(&vec01);

        assert_eq!(vec01, [22, 44, 66]);
        assert_eq!(vec02, [22, 44, 66, 88]);
    }
}
