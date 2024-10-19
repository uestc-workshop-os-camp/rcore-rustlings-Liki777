/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// a I AM NOT DONE

fn sort<T:PartialEq+std::cmp::PartialOrd +Copy>(array: &mut [T]){
	//TODO
    //bubble
    let len = array.len();
    for i in 0..len{
        let mut second_len = len;
        for j in 0..second_len{
            if array[i] > array[j]{
                continue;
            }else{
                let mut temp:T = array[i];
                array[i] = array[j];
                array[j] = temp;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}