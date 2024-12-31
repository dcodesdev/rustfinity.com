If you’ve completed the first part of the Student Grades Tracker challenge, congratulations! Now it’s time to enhance the functionality by adding methods directly to the `Student` struct.

This will help you practice designing methods for individual struct instances while integrating them into a larger system.

For this challenge, we’ll build upon the `StudentGrades` system. In addition to the methods you’ve already implemented, you’ll now focus on adding two new methods to the `Student` struct itself.

## Your Task

You need to extend the `Student` struct by implementing two methods:

1. **`add_grade(&mut self, grade: u8)`:** Add a grade to the student’s `grades` vector.
2. **`average_grade(&self) -> f64`:** Calculate and return the average grade of the student. Return `0.0` if the student has no grades.

The `StudentGrades` struct remains the same as in the previous challenge. You’ll modify its existing methods to use the newly added methods of the `Student` struct.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- Use the `self` parameter in the `Student` struct methods for mutable or immutable references as required.
- Iterate over all students in the `StudentGrades` `HashMap` to calculate the total average using the `average_grade` method of each `Student`.
- Modify `StudentGrades::add_grade` to delegate the task of adding a grade to the relevant student’s method.

</details>
