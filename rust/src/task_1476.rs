struct SubrectangleQueries {
	rectangle: Vec<Vec<i32>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {
	fn new(rectangle: Vec<Vec<i32>>) -> Self {
		SubrectangleQueries { rectangle }
	}

	fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
		for i in row1 as usize..=row2 as usize {
			for j in col1 as usize..=col2 as usize {
				self.rectangle[i][j] = new_value;
			}
		}
	}

	fn get_value(&self, row: i32, col: i32) -> i32 {
		self.rectangle[row as usize][col as usize]
	}
}

#[cfg(test)]
#[test]
fn test_subrectangle_queries() {
	let rectangle = vec![vec![1, 2, 1], vec![4, 3, 4], vec![3, 2, 1], vec![1, 1, 1]];
	let subrectangle_queries = SubrectangleQueries::new(rectangle);
	let value1 = subrectangle_queries.get_value(0, 2);
}