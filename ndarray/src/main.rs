use ndarray::array;

fn main() {
    let a = array![[1., 2., 3.], [4., 5., 6.]];
    assert_eq!(a.ndim(), 2);
    assert_eq!(a.len(), 6);
    assert_eq!(a.shape(), [2,3]);
    assert_eq!(a.is_empty(), false);   
}