#[allow(dead_code)]
//Compare func should return true if first is bigger.
pub fn sort<T, F>(arr: &mut [T], compare_func: &F)
where F: Fn(&T, &T) -> bool {
    if arr.len() <= 1 {
        //Array is of length 1 or less and therefore sorted.
        return;     
    }

    //The high value will be our pivot for now!
    let high = arr.len() - 1;
    let mut wall = 0;

    //Between low and arr.len() because this is doesnt include the high bound.
    for current in 0..arr.len() {
        if compare_func(&arr[high], &arr[current]) {
            //If true then this value is less than our pivot value.
            //Swap the element to the at the wall with this element.
            arr.swap(wall, current);
            //Move the wall up 1
            wall += 1;
        }
    }

    //Now swap the pivot into its position.
    arr.swap(wall, high);
    //Now sort left and right of the wall.
    sort(&mut arr[..wall], compare_func);
    if wall < arr.len() - 1 {
        sort(&mut arr[wall + 1..], compare_func);
    }
}


#[cfg(test)]
mod sort_test {
    use super::*;
    
    #[test]
    fn sort_i32() {
        let mut test_vec = vec![1, 5, 8, 2, 6, 1];
        println!("PreSort: {:?}", test_vec);
        sort(&mut test_vec[..], &|num1, num2|{ num1 > num2 });   
        println!("PostSort: {:?}", test_vec);
        assert_eq!(test_vec, vec![1, 1, 2, 5, 6, 8]);  
    }
}