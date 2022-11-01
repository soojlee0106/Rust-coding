use grid::Grid;

#[allow(dead_code, clippy::similar_names)]
//Longest common substring among two words
fn dynamic<'a>(mut cell: Grid<usize>, word_a: &'a str, word_b: &'a str) -> String {
    let mut common_substring_chars: Vec<char> = vec![];
    let word_a_chars: Vec<char> = word_a.chars().collect();
    let word_b_chars: Vec<char> = word_b.chars().collect();

    for mut i in 0..cell.rows() {
        for item in word_b_chars.iter().enumerate().take(cell.cols()) {
            if word_a_chars[i] == word_b_chars[item.0] {
                cell[i][item.0] = cell[i - 1][item.0 - 1] + 1;
                common_substring_chars.push(word_a_chars[i]);
                i += 1;
                if i > item.0 {
                    break;
                }
            } else {
                cell[i][item.0] = 0;
            }
        }
    }

    let common_substring = common_substring_chars.iter().collect();
    common_substring
}

#[cfg(test)]
mod tests {
    use super::*;
    use grid::grid;

    #[test]
    fn no_cell() {
        let cell = grid![];
        let test = dynamic(cell, "cat", "bat");
        let expected = "";
        assert_eq!(test, expected);
    }

    #[test]
    fn simple_words() {
        let cell = grid![[0,0,0][0,0,0][0,0,0]];
        let test = dynamic(cell, "cat", "bat");
        let expected = "at";
        assert_eq!(test, expected);
    }

    #[test]
    fn complex_words() {
        let cell =
            grid![[0,0,0,0,0,0][0,0,0,0,0,0][0,0,0,0,0,0][0,0,0,0,0,0][0,0,0,0,0,0][0,0,0,0,0,0]];
        let test = dynamic(cell, "appley", "cpplet");
        let expected = "pple";
        assert_eq!(test, expected);
    }

    #[test]
    fn very_complex_words() {
        let cell = grid![[0,0,0,0,0,0,0,0,0][0,0,0,0,0,0,0,0,0][0,0,0,0,0,0,0,0,0][0,0,0,0,0,0,0,0,0][0,0,0,0,0,0,0,0,0][0,0,0,0,0,0,0,0,0][0,0,0,0,0,0,0,0,0][0,0,0,0,0,0,0,0,0][0,0,0,0,0,0,0,0,0]];
        let test = dynamic(cell, "corangeti", "borangeye");
        let expected = "orange";
        assert_eq!(test, expected);
    }
}
