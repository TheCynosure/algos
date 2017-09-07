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

	//TODO: For now this just makes the array in the heap equal to the one passed in.
	//TODO: It does not actually do any ordering yet.
	pub fn heapify(arr: Vec<T>, f: F) -> Self
	where F: Fn(&T, &T) -> bool {
		Self {
			elems: arr,
			size: 0,
			comparator_function: f,
		}
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
		let test_heap = MaxHeap::heapify(vec![7, 6, 5, 4, 3, 2, 1], |num1, num2| {num1 > num2});
		assert_eq!(test_heap.elems[test_heap.get_parent(6)], 5);
	}
}