In many real-world scenarios like library inventories, you need precise checks for validity.
Here’s the challenge with an instance method BookItem::check_validity(&self):

1. Book:
   - pages > 0
   - if there's a discount (Some(d)), then 0 <= d <= 50
2. EBook:
   - title must be non-empty
   - second field in the tuple must be > 0
3. Collection:
   - must contain at least one valid BookItem (recursively call .check_validity())
4. OutOfPrint:
   - always invalid

### Requirements

- Implement check_validity(&self) using a match on self.
- Return a boolean (true/false) based on the type of BookItem.
- Avoid panics for valid inputs.

### Hints

<details>
  <summary>Click here to reveal hints</summary>

- Use match and guards (if expressions).
- For data not needed, use \_.
- Recursively call i.check_validity() for Collections.

</details>
