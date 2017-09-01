#[allow(dead_code)]
//TODO: Build some default compare funcs for common types like u32 and Strings.
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
    //TODO: Find a more elegant way to deal with this edge case.
    //This happens when the wall of the array never changed its initial position.
    if wall < arr.len() - 1 {
        sort(&mut arr[wall + 1..], compare_func);
    }
}


#[cfg(test)]
mod sort_test {
    use super::*;
    
    #[test]
    fn sort_i32() {
		//Sort a normal array
		let mut test_vec = vec![1, 5, 7, 8, 2, 4];
		sort(&mut test_vec[..], &|num1, num2|{ num1 > num2 });
		assert_eq!(test_vec, vec![1, 2, 4, 5, 7, 8]);
    
    	//Check on an unnsorted array with repeat elems
        let mut test_vec = vec![1, 5, 8, 2, 6, 1];
        sort(&mut test_vec[..], &|num1, num2|{ num1 > num2 });   
        assert_eq!(test_vec, vec![1, 1, 2, 5, 6, 8]);

		//Check on a sorted array
        let mut test_vec = vec![1, 2, 3, 4, 5, 6];
        sort(&mut test_vec[..], &|num1, num2|{ num1 > num2 });
        assert_eq!(test_vec, vec![1, 2, 3, 4, 5, 6]);		
    }

    fn comp_func(num1: &u32, num2: &u32) -> bool {
    	num1 > num2
    }

    #[test]
    fn function_pass_test() {
    	//TODO: We can use something like this to do a String sort.
		let mut test_vec = vec![1, 3, 2];
		sort(&mut test_vec[..], &comp_func);
		assert_eq!(test_vec, vec![1, 2, 3]);  	
    }

	#[test]
	fn sort_reverse_u32() {
		//Sort a normal array, but in reverse order.
		//Todo this just make it return if num2 is bigger, instead of num1.
		let mut test_vec = vec![1, 5, 7, 8, 2, 4];
		sort(&mut test_vec[..], &|num1, num2|{ num2 > num1 });
		assert_eq!(test_vec, vec![8, 7, 5, 4, 2, 1]);
	}    
}