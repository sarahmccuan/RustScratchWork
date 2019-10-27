fn main() {
	
	let int_vec = vec![1, 2, 3, 4, 5,];
	
	println!("int_vec is {:?}", int_vec);	
	for i in int_vec {
		println!("value is {:?}", i);
	}



	
}

fn mean(vec: [i32]) -> i32 { // should be float instead here
	let mut sum = 0;
	for i in vec {
		sum += i;
	}
	sum / vec.len()
}

#[test]
fn test_mean() {
	let test_vec = vec![1, 2, 3, 4, 5];
	let test_vec_mean = 3;
	assert_eq!(mean(test_vec), test_vec_mean);
}
	
