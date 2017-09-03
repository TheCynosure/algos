use std::string;

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
    //If we want to sort the right side we have to check if the wall is at the end
    //of the array, otherwise we could cause an overflow / error.
    if wall < arr.len() - 1 {
        sort(&mut arr[wall + 1..], compare_func);
    }
}

//TODO: Fix ascii casting.
#[allow(dead_code)]
//Compare Func usually return true if the first one is bigger, so here we want to return the 
//string that comes alphabetically last.
pub fn string_alphabetic<S: string::ToString>(str1: &S, str2: &S) -> bool {
    let str1 = str1.to_string();
    let str2 = str2.to_string();
    //Find if any of the characters in str1 alphabetically after
    //str2's chars.
    for c_tuple in str1.chars().zip(str2.chars()) {
        let (str1_c, str2_c) = c_tuple;
        if str1_c.is_alphabetic() && str2_c.is_alphabetic() {
            return (str1_c as u32) > (str2_c as u32);
        }
    } 

    //Else return if str1 is shorter, as it should be first then.
    if str1.len() > str2.len() {
        return false;     
    }
    //Return true otherwise, because str2 should come before one.
    true
}

#[cfg(test)]
mod sort_test {
    use super::*;
    
    #[test]
    fn sort_i32() {
        //Sort a normal array
        let mut test_vec = vec![1, 5, 7, 8, 2, 4];
        sort(&mut test_vec[..], &|&num1, &num2|{ num1 > num2 });
        assert_eq!(test_vec, vec![1, 2, 4, 5, 7, 8]);
    
        //Check on an unnsorted array with repeat elems
        let mut test_vec = vec![1, 5, 8, 2, 6, 1];
        sort(&mut test_vec[..], &|&num1, &num2|{ num1 > num2 });   
        assert_eq!(test_vec, vec![1, 1, 2, 5, 6, 8]);

        //Check on a sorted array
        let mut test_vec = vec![1, 2, 3, 4, 5, 6];
        sort(&mut test_vec[..], &|&num1, &num2|{ num1 > num2 });
        assert_eq!(test_vec, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn sort_reverse_u32() {
        //Sort a normal array, but in reverse order.
        //Todo this just make it return if num2 is bigger, instead of num1.
        let mut test_vec = vec![1, 5, 7, 8, 2, 4];
        sort(&mut test_vec[..], &|&num1, &num2|{ num2 > num1 });
        assert_eq!(test_vec, vec![8, 7, 5, 4, 2, 1]);
    }

    #[test]
    fn sort_string_test() {
        let mut test_vec = vec!["Cat".to_string(), "Dog".to_string(), "Antelope".to_string()];
        sort(&mut test_vec[..], &string_alphabetic);
        assert_eq!(test_vec, vec!["Antelope".to_string(), "Cat".to_string(), "Dog".to_string()]);
    }

    #[test]
    fn sort_str_test() {
        let mut test_vec = vec!["Cat", "Walrus", "Rhino", "Yak", "Zebra", "Bacterium"];
        sort(&mut test_vec[..], &string_alphabetic);
        assert_eq!(test_vec, vec!["Bacterium", "Cat", "Rhino", "Walrus", "Yak", "Zebra"]);     
    }
}