# Maze Solver

In this challenge, you will implement a maze solver in Rust. A maze can be represented as a grid where each cell is either an open path or a wall. The goal is to navigate from a starting point to an ending point using the shortest path possible. This problem will test your understanding of control flow in Rust, including loops, conditionals, and possibly recursion.

## Your Task

Your task is to write a function `solve_maze` that takes a 2D vector of characters representing the maze and two tuples representing the start and end points. The maze will be given as a grid of characters where 'S' represents the start, 'E' represents the end, '.' represents an open path, and '#' represents a wall.

The function should return a vector of tuples representing the path from start to end if a path exists, or an empty vector if no path exists.

## Requirements

- The maze will always contain exactly one 'S' and one 'E'.
- You can only move up, down, left, or right.
- Implement the function using appropriate control flow constructs (loops, conditionals, and/or recursion).
- Ensure your solution handles edge cases, such as no available path or mazes with various sizes.

## Example

```rust
let maze = vec![
    vec!['S', '.', '#', '#', '#'],
    vec!['#', '.', '#', '.', '.'],
    vec!['#', '.', '.', '.', '#'],
    vec!['#', '#', '#', '.', '#'],
    vec!['#', '#', '#', 'E', '#']
];
let start = (0, 0);
let end = (4, 3);

let path = solve_maze(maze, start, end);
assert_eq!(path, vec![(0, 0), (1, 0), (2, 1), (2, 2), (3, 2), (4, 2), (4, 3)]);
```

> Did You Know?
>
> Maze solving algorithms are not just for games. They are used in robotics for **pathfinding**, in computer networks for routing, and in various optimization problems. Algorithms such as Depth-First Search (DFS), Breadth-First Search (BFS), and A\* are commonly used to solve these problems efficiently.

## Hints

- Start by implementing a simple depth-first search (DFS) or breadth-first search (BFS) to explore the maze.
- Consider using a queue for BFS or a stack for DFS to manage the cells to be explored.
- Keep track of visited cells to avoid infinite loops.
- Think about how you can reconstruct the path once you find the end.
