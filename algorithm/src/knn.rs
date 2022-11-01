use std::process::exit;

//Each row consists of (x,y,type).
//Calculate the Euclidean distance between two points (x,y). The third char in the array is not considered since it denotes the type.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
#[allow(dead_code)]
fn euclidean_distance(row1: &(f64, f64, char), row2: &(f64, f64, char)) -> usize {
    let mut distance = 0.0; //Initial distance is 0

    distance += (row1.0 - row2.0).powf(2.0);
    distance += (row1.1 - row2.1).powf(2.0);

    distance.sqrt() as usize
}

#[allow(dead_code)]
fn get_neighbors(
    train: &Vec<(f64, f64, char)>,
    test_row: &(f64, f64, char),
    num_neighbors: usize,
) -> Vec<(f64, f64, char)> {
    //Finds the closest neighbors to the point
    let mut distances: Vec<((f64, f64, char), usize)> = vec![];
    for train_row in train {
        let dist = euclidean_distance(test_row, train_row);
        distances.push((*train_row, dist));
    }
    distances.sort_by_key(|tup| tup.1);

    let mut neighbors = vec![];
    for item in distances.iter().take(num_neighbors) {
        neighbors.push(item.0);
    }
    neighbors
}

#[allow(dead_code)]
fn predict_classification(
    train: &Vec<(f64, f64, char)>,
    test_row: &(f64, f64, char),
    num_neighbors: usize,
) -> char {
    //Predicts what type the point is. The type is denoted as a char form.
    let neighbors = get_neighbors(train, test_row, num_neighbors);
    let mut output_values = vec![];
    for row in neighbors {
        output_values.push(row.2);
    }
    let prediction = match output_values.iter().max() {
        Some(prediction) => prediction,
        None => exit(1),
    };
    *prediction
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closer_to_a() {
        //Create distance dataset (x,y,type) (type is either a or b)
        let mut dataset: Vec<(f64, f64, char)> = Vec::new();
        dataset.push((1.465489372, 2.362125076, 'a'));
        dataset.push((3.396561688, 4.400293529, 'a'));
        dataset.push((1.38807019, 5.850220317, 'a'));

        dataset.push((6.922596716, 1.77106367, 'b'));
        dataset.push((7.627531214, 2.759262235, 'b'));
        dataset.push((5.922596716, 1.1236367, 'b'));

        let test_row = (1.0, 3.0, 'n'); //n is placeholder type
        let test = predict_classification(&dataset, &test_row, 3);
        let expected = 'a';
        assert_eq!(test, expected);
    }

    #[test]
    fn closer_to_c() {
        //Create distance dataset (x,y,type) (type is either b or c)
        let mut dataset: Vec<(f64, f64, char)> = Vec::new();
        dataset.push((1.465489372, 6.362125076, 'b'));
        dataset.push((0.396561688, 4.400293529, 'b'));
        dataset.push((1.38807019, 5.850220317, 'b'));

        dataset.push((6.922596716, 1.77106367, 'c'));
        dataset.push((7.627531214, 2.759262235, 'c'));
        dataset.push((10.922596716, 1.1236367, 'c'));

        let test_row = (6.0, 1.0, 'n'); //n is placeholder type
        let test = predict_classification(&dataset, &test_row, 3);
        let expected = 'c';
        assert_eq!(test, expected);
    }

    #[test]
    fn closer_to_f() {
        //Create distance dataset (x,y,type) (type is either d or f)
        let mut dataset: Vec<(f64, f64, char)> = Vec::new();
        dataset.push((25.465489372, 2.362125076, 'd'));
        dataset.push((45.396561688, 4.400293529, 'd'));
        dataset.push((27.38807019, 5.850220317, 'd'));

        dataset.push((6.922596716, 37.77106367, 'f'));
        dataset.push((7.627531214, 98.759262235, 'f'));
        dataset.push((5.922596716, 12.1236367, 'f'));

        let test_row = (12.0, 3.0, 'n'); //n is placeholder type
        let test = predict_classification(&dataset, &test_row, 3);
        let expected = 'f';
        assert_eq!(test, expected);
    }
}
