pub fn solve_maze(
    maze: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    use std::collections::VecDeque;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut path = vec![vec![None; maze[0].len()]; maze.len()];

    queue.push_back(start);
    visited[start.0][start.1] = true;

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == end {
            let mut p = vec![];
            let mut current = Some((x, y));
            while let Some(c) = current {
                p.push(c);
                current = path[c.0][c.1];
            }
            p.reverse();
            return p;
        }

        for &(dx, dy) in &directions {
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx < maze.len() && ny < maze[0].len() && maze[nx][ny] != '#' && !visited[nx][ny] {
                queue.push_back((nx, ny));
                visited[nx][ny] = true;
                path[nx][ny] = Some((x, y));
            }
        }
    }

    vec![]
}
