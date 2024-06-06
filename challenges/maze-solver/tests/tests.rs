#[cfg(test)]
mod tests {
    use maze_solver::*;

    #[test]
    fn test_solve_maze() {
        let maze = vec![
            vec!['S', '.', '#', '#', '#'],
            vec!['#', '.', '#', '.', '.'],
            vec!['#', '.', '.', '.', '#'],
            vec!['#', '#', '#', '.', '#'],
            vec!['#', '#', '#', 'E', '#'],
        ];
        let start = (0, 0);
        let end = (4, 3);

        let path = solve_maze(maze, start, end);
        assert_eq!(
            path,
            vec![
                (0, 0),
                (0, 1),
                (1, 1),
                (2, 1),
                (2, 2),
                (2, 3),
                (3, 3),
                (4, 3)
            ]
        );
    }

    #[test]
    fn test_no_path() {
        let maze = vec![
            vec!['S', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '#', '#'],
            vec!['#', '#', '#', 'E', '#'],
        ];
        let start = (0, 0);
        let end = (4, 3);

        let path = solve_maze(maze, start, end);
        assert_eq!(path, vec![]);
    }

    #[test]
    fn test_edge_case() {
        let maze = vec![vec!['S', '.', '.', '.', 'E']];
        let start = (0, 0);
        let end = (0, 4);

        let path = solve_maze(maze, start, end);
        assert_eq!(path, vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]);
    }
}
