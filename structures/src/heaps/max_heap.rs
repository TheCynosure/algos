extern crate algorithms;

//For normal sorting purpose of a max_heap comparator function
//returns if the second parameter is larger than the first.
//This is because this is how the sorting algorithms sort backwards, i.e. in the format of
//a max heap.
pub struct MaxHeap<T, F> {
	elems: Vec<T>,
	size: usize,
	comparator_function: F,
}

impl<T, F> MaxHeap<T, F>
where F: Fn(&T, &T) -> bool {
	pub fn new(f: F) -> Self {
		Self {
			elems: Vec::new(),
			size: 0,
			comparator_function: f,
		}
	}

	//This method is based off of quicksorting the array before making it into a heap.
	//Worst case this should create an array in n^2
	pub fn quicksort_heapify(mut arr: Vec<T>, f: F) -> Self {
		algorithms::sorting::quicksort::sort(&mut arr[..], &f);
		let len = arr.len();
		Self {
			elems: arr,
			size: len,
			comparator_function: f,
		}
	}

	//This method is based on the sorting method explained to me by my TA Aritra.
	//It should sort the array worst case n log(n)
	pub fn aritra_heapify(mut arr: Vec<T>, f: F) -> Self {
		let len = arr.len();
		let mut mh = MaxHeap {
			elems: arr,
			size: len,
			comparator_function: f,
		};

		//Start at the end
		for i in (0..len).rev() {
			//While the child is greater than the parent.
			mh.comp_p_and_c_swap(i);
					
		}

		return mh;
	}

	//Takes in the index of a child.
	//Compares parent and child and if the child is greater will swap the child and parent, 
	//continues up the entire heap, returning the new child index.
	fn comp_p_and_c_swap(&mut self, index: usize) -> usize {
		let mut child_index = index; //Copy the Child Index.
		while (self.comparator_function)(&self.elems[self.get_parent(child_index)], &self.elems[child_index]) {
			let parent_index = self.get_parent(child_index);
			//Swap the Child and the parent.
			self.elems.swap(parent_index, child_index);
			//Keep tracking our child index so we can check the next parent.
			child_index = parent_index;
		}
		child_index
	}

	pub fn get_parent(&self, index: usize) -> usize {
		let temp = (index / 2) + (index % 2);
		//If they have a parent we should subtract 1 to get the parent.
		if temp > 0 {
			return temp - 1;
		}
		//Otherwise we should return 0 if they don't have a parent.
		temp
	}

	pub fn get_left_child(&self, index: usize) -> Option<usize> {
		if (index * 2) + 1 > self.elems.len() {
			return None;
		}
		Some((index * 2) + 1)
	}

	pub fn get_right_child(&self, index: usize) -> Option<usize> {
		if (index * 2) + 2 > self.elems.len() {
			return None;
		}
		Some((index * 2) + 2)
	}

	//Pushes an item onto the stack and returns the index.
	pub fn push(&mut self, elem: T) -> usize {
		self.elems.push(elem);
		let len = self.elems.len();
		self.comp_p_and_c_swap(len - 1)
	}

	pub fn length(&self) -> usize {
		self.elems.len()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn heap_basics() {
		//Heapify with a reverse sort.
		let test_heap = MaxHeap::quicksort_heapify(vec![7, 6, 5, 4, 3, 2, 1], |num1, num2| {num2 > num1});
		assert_eq!(test_heap.elems[test_heap.get_parent(6)], 5);

		let test_heap = MaxHeap::quicksort_heapify(vec![1, 5, 7, 3, 2, 6, 4], |num1, num2| {num2 > num1});		
		assert_eq!(test_heap.elems[test_heap.get_parent(1)], 7);
	}

	#[test]
	fn aritra_test() {
		//Heapify with a reverse sort.
		let test_heap = MaxHeap::aritra_heapify(vec![7, 6, 5, 4, 3, 2, 1], |num1, num2| {num2 > num1});
		assert_eq!(test_heap.elems[test_heap.get_parent(6)], 5);

		let test_heap = MaxHeap::aritra_heapify(vec![1, 5, 7, 3, 2, 6, 4], |num1, num2| {num2 > num1});		
		assert_eq!(test_heap.elems[test_heap.get_parent(1)], 7);
	}

	#[test]
	fn push_test() {
		let mut test_heap = MaxHeap::aritra_heapify(vec![7, 8, 2, 9, 1, 10, 14, 8], |num1, num2| {num2 > num1});

		assert_eq!(test_heap.push(1), test_heap.length() - 1);

		test_heap.push(8);
		test_heap.push(3);

		assert_eq!(test_heap.length(), 11);
	}

	#[test]
	fn delete_test() {
		//TODO: Delete something from the heap by index.
		assert!(false);
	}

	#[test]
	fn clear_test() {
		//TODO: Clear the current heap.
		assert!(false);
	}

	#[test]
	fn max_test() {
		//TODO: Get the maximum element in the heap.
		assert!(false);
	}

	fn search_test() {
		//TODO: Find elements in the heap.
		assert!(false);
	}
}