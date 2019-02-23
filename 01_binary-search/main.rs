fn binary_search<T: PartialOrd>(list: &[T], item: &T) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = list.len();

    println!("high: ==>  {}", high);
    println!("low: ==>  {}", low);

    while low < high {
        let mid: usize = (high + low) / 2 - low;

        if *item == list[mid] {
            return Some(mid);
        } else if *item < list[mid] {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    None
}

fn main() {

    match binary_search(&[0, 1, 2, 4, 8, 16], &4) {
        Some(item) => println!("{}", item),
        None => println!("Could not find item {}", &4),
    }

    match binary_search(&[1, 3, 4, 5, 6, 7, 9], &3) {
        Some(item) => println!("{}", item),
        None => println!("Could not find item {}", &3),
    }
}
