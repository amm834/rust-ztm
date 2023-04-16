fn main() {
    let mut data = Some(10);
    while let Some(i) = data {
        println!("data = {}", i);
        data = None;
    }

    let numbers = vec![1,2,3,4,5 ];
    let mut numbers_iter = numbers.iter();
    while let Some(i) = numbers_iter.next() {
        println!("number = {:?}", i);
    }
}
