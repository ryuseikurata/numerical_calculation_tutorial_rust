use ndarray::array;

fn main() {
    let a = array![[1.0, 2.00, 3.00], [4.0, 5.0, 6.0]];
    assert_eq!(a.ndim(), 2);
    assert_eq!(a.len(), 6);
    assert_eq!(a.shape(), [2,3]);
    assert_eq!(a.is_empty(), false);   
    println!("{:?}", a);
}