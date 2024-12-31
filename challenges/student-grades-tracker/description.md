Now that we know about vectors and hashmaps, let’s tackle something more challenging. In this exercise, we’ll build a system to manage and analyze student grades. This challenge will help you practice working with structs, hashmaps, and methods while managing a collection of data.

The focus here is on functionality. **Ignore error handling for this challenge**; assume the input is always valid.

## Your Task

You need to define the `StudentGrades` struct, which contains a `HashMap` of student names (`String`) as keys and `Student` structs as values. Each `Student` struct should have the following fields:

- `name`: The name of the student (`String`).
- `grades`: A `Vec<u8>` to store the student's grades.

Implement the following methods for the `StudentGrades` struct:

1. **`add_student(name: &str)`:** Add a new student to the `HashMap`. If the student already exists, do nothing.
2. **`add_grade(name: &str, grade: u8)`:** Add a grade to an existing student.
3. **`get_grades(name: &str) -> &[u8]`:** Retrieve the grades of a student as an immutable reference.
4. **`calculate_average() -> f64`:** Calculate and return the average of all grades for all students. Return `0.0` if there are no grades.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- Use the `HashMap` methods like `entry`, `insert`, and `get` to manage student data.
- To calculate the average, you can iterate over the `grades` field of all students in the `HashMap`.
- Since there’s no error handling, assume all inputs are valid, and keys will exist when accessed.

</details>
