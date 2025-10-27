use faer::Mat;

/// Formats the sum of a list of numbers as string.
fn list_sum_string(nums: Vec<f64>) -> String {
    nums.into_iter().sum::<f64>().to_string()
}

/// Formats the sum of a 2D array of numbers as string.
fn array_sum_string(nums: Mat<f64>) -> String {
    nums.sum().to_string()
}

