fn main() {
    println!("Hello, world!");
}

fn op(i: i32, a: i64, b: i64) -> i64 {
    if i == 1 {
        a + b
    } else {
        if b == 1 {
            a
        } else {
            op(i - 1, a, op(i, a, b - 1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn op_1() {
        assert_eq!(op(1, 1, 1), 1 + 1);
        assert_eq!(op(1, 2, 3), 2 + 3);
    }

    #[test]
    fn op_2() {
        assert_eq!(op(2, 1, 1), 1 * 1);
        assert_eq!(op(2, 2, 3), 2 * 3);
    }
}
