pub fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_integers() {
        let mut x = 1;
        let mut y = 2;
        swap(&mut x, &mut y);
        assert_eq!(x, 2);
        assert_eq!(y, 1);
    }

    #[test]
    fn test_swap_strings() {
        let mut a = String::from("hello");
        let mut b = String::from("world");
        swap(&mut a, &mut b);
        assert_eq!(a, "world");
        assert_eq!(b, "hello");
    }
}

fn main() {
    let mut x = 1;
    let mut y = 2;
    println!("Before swap: x={}, y={}", x, y);
    swap(&mut x, &mut y);
    println!("After swap: x={}, y={}", x, y);
}