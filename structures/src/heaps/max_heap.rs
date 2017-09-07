extern crate algorithms;

pub struct MaxHeap<T, F> {
	elems: Vec<T>,
	size: usize,
	comparator_function: F,
}

impl<T, F> MaxHeap<T, F> {
	pub fn new(f: F) -> Self
	where F: Fn(&T, &T) -> bool {
		Self {
			elems: Vec::new(),
			size: 0,
			comparator_function: f,
		}
	}

	//This method is based off of quicksorting the array before making it into a heap.
	//Worst case this should create an array in n^2
	pub fn quicksort_heapify(mut arr: Vec<T>, f: F) -> Self
	where F: Fn(&T, &T) -> bool {
		algorithms::sorting::quicksort::sort(&mut arr[..], &f);
		Self {
			elems: arr,
			size: 0,
			comparator_function: f,
		}
	}

	//This method is based on the sorting method explained to me by my TA Aritra.
	//It should sort the array worst case 2^n Log(n) I believe.
	pub fn aritra_heapify() {
		
	}

	pub fn get_parent(&self, index: usize) -> usize {
		(index / 2) + (index % 2) - 1
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
}