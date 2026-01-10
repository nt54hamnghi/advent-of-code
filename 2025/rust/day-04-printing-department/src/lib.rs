use std::fmt::Display;

#[derive(Debug)]
pub struct Grid {
    pub height: usize,
    pub width: usize,
    pub data: Vec<char>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for ck in self.data.chunks(self.height) {
            let l = ck.iter().collect::<String>();
            s.push_str(&l);
            s.push_str("\n");
        }
        write!(f, "{s}")
    }
}

impl Grid {
    pub fn adjecent_of(&self, pos: usize) -> impl Iterator<Item = usize> {
        [
            self.top_left_of(pos),
            self.top_of(pos),
            self.top_right_of(pos),
            self.left_of(pos),
            self.right_of(pos),
            self.bottom_right_of(pos),
            self.bottom_of(pos),
            self.bottom_left_of(pos),
        ]
        .into_iter()
        .flatten()
    }

    /// Returns the row index for a given position in the grid.
    /// The zero-indexed row number where the position is located
    pub fn row_of(&self, pos: usize) -> usize {
        pos / self.width
    }

    pub fn left_of(&self, pos: usize) -> Option<usize> {
        let row = self.row_of(pos);
        let new = pos.checked_sub(1)?;

        (new >= self.width * row).then_some(new)
    }

    pub fn right_of(&self, pos: usize) -> Option<usize> {
        let row = self.row_of(pos);
        let new = pos + 1;

        (new < self.width * (row + 1)).then_some(new)
    }

    pub fn top_of(&self, pos: usize) -> Option<usize> {
        pos.checked_sub(self.width)
    }

    pub fn bottom_of(&self, pos: usize) -> Option<usize> {
        let new = pos + self.width;
        (new < self.data.len()).then_some(new)
    }

    pub fn top_left_of(&self, pos: usize) -> Option<usize> {
        let new = self.top_of(pos)?;
        self.left_of(new)
    }

    pub fn top_right_of(&self, pos: usize) -> Option<usize> {
        let new = self.top_of(pos)?;
        self.right_of(new)
    }

    pub fn bottom_left_of(&self, pos: usize) -> Option<usize> {
        let new = self.bottom_of(pos)?;
        self.left_of(new)
    }

    pub fn bottom_right_of(&self, pos: usize) -> Option<usize> {
        let new = self.bottom_of(pos)?;
        self.right_of(new)
    }
}
impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut height = 0;
        let mut data = Vec::new();

        for l in value.lines() {
            for c in l.chars() {
                data.push(c);
            }
            height += 1;
        }

        Self {
            height,
            width: value.len() / height,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::*;

    #[fixture]
    fn grid_3x3() -> Grid {
        Grid {
            height: 3,
            width: 3,
            data: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
        }
    }
    #[rstest]
    #[case(0, None)]
    #[case(1, Some(0))]
    #[case(2, Some(1))]
    #[case(3, None)]
    #[case(4, Some(3))]
    #[case(5, Some(4))]
    #[case(6, None)]
    #[case(7, Some(6))]
    #[case(8, Some(7))]
    fn test_left_of(grid_3x3: Grid, #[case] pos: usize, #[case] expected: Option<usize>) {
        assert_eq!(grid_3x3.left_of(pos), expected);
    }
    #[rstest]
    #[case(0, Some(1))]
    #[case(1, Some(2))]
    #[case(2, None)]
    #[case(3, Some(4))]
    #[case(4, Some(5))]
    #[case(5, None)]
    #[case(6, Some(7))]
    #[case(7, Some(8))]
    #[case(8, None)]
    fn test_right_of(grid_3x3: Grid, #[case] pos: usize, #[case] expected: Option<usize>) {
        assert_eq!(grid_3x3.right_of(pos), expected);
    }

    #[rstest]
    #[case(0, None)]
    #[case(1, None)]
    #[case(2, None)]
    #[case(3, Some(0))]
    #[case(4, Some(1))]
    #[case(5, Some(2))]
    #[case(6, Some(3))]
    #[case(7, Some(4))]
    #[case(8, Some(5))]
    fn test_top_of(grid_3x3: Grid, #[case] pos: usize, #[case] expected: Option<usize>) {
        assert_eq!(grid_3x3.top_of(pos), expected);
    }

    #[rstest]
    #[case(0, Some(3))]
    #[case(1, Some(4))]
    #[case(2, Some(5))]
    #[case(3, Some(6))]
    #[case(4, Some(7))]
    #[case(5, Some(8))]
    #[case(6, None)]
    #[case(7, None)]
    #[case(8, None)]
    fn test_bottom_of(grid_3x3: Grid, #[case] pos: usize, #[case] expected: Option<usize>) {
        assert_eq!(grid_3x3.bottom_of(pos), expected);
    }

    #[rstest]
    #[case(0, vec![1, 3, 4])]
    #[case(1, vec![0, 2, 3, 4, 5])]
    #[case(2, vec![1, 4, 5])]
    #[case(3, vec![0, 1, 4, 6, 7])]
    #[case(4, vec![0, 1, 2, 3, 5, 6, 7, 8])]
    #[case(5, vec![1, 2, 4, 7, 8])]
    #[case(6, vec![3, 4, 7])]
    #[case(7, vec![3, 4, 5, 6, 8])]
    #[case(8, vec![4, 5, 7])]
    fn test_adjecent_of(grid_3x3: Grid, #[case] pos: usize, #[case] expected: Vec<usize>) {
        let mut result = grid_3x3.adjecent_of(pos).collect::<Vec<_>>();
        result.sort();
        assert_eq!(result, expected);
    }
}
