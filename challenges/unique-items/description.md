Managing collections of data efficiently is a crucial skill in software development. Iterators in Rust provide a flexible way to traverse, filter, and transform collections without consuming them. In this challenge, you will use iterators to filter out duplicate items from a collection of potentially redundant entries.

Imagine you're working in inventory management, and you receive a list of product identifiers that contains duplicates. You need to ensure that the system only processes unique items, as duplicates might cause errors or inefficiencies.

## Your Task

Define a function that takes a collection of items (as an iterator) and returns a collection of unique items in sorted order. The items are represented as strings. You are expected to handle duplicates and ignore entries that are empty or consist solely of whitespace.

Your function signature and return types must be determined by you, but the input must implement the `Iterator` trait, ensuring flexibility for different iterator sources.

### Requirements

- **Filter duplicates:** Ensure that the resulting collection contains only unique items.
- **Ignore invalid entries:** Exclude entries that are empty or only consist of whitespace.
- **Sort the output:** Return the items sorted in ascending lexicographical order.
- **Flexibility:** The function should accept an iterator, allowing compatibility with various input sources.

## Hints

<details>
<summary>Click here to reveal hints</summary>

- Use the `filter` method to remove invalid entries (e.g., empty or whitespace-only strings).
- Use a `HashSet` to efficiently track duplicates.
- The `Iterator` trait provides methods like `collect`, which can be combined with sorting logic.
- You can use the `trim` method on strings to handle whitespace effectively.
- To sort, you can convert to a `Vec` and call the `sort` method.

</details>
