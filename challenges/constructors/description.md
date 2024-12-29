Unlike other programming languages, Rust doesn't have a build-in `constructor` keyword to construct instances of a struct. Instead, you can directly create an instance of a struct simply by calling the struct's name and passing the required values.

```rust
struct Post {
    pub content: String,
    pub user: String,
    pub views: u32
}

fn main(){
    let new_blog_post = Post {
        content: "Some content".to_string(),
        user: "Jon Doe".to_string(),
        views: 0
    };
}
```

However, this pattern is discouraged, in real-life scenarios, we might not want to expose all fields with `pub` or some fields will have default values that it would be repetitive to provide the value for every new instance, instead.

In Rust we have a **convention**, constructors are implemented as associated functions named `new`. They encapsulate the initialization of the struct, ensuring that instances are always created in a valid state.

For example, for the `Post` struct above, we can have a default value for the `views` to always be `0` for new instances.

```rust
impl Post {
    pub fn new(content: String, user: String) -> Post {
        Post {
            content,
            user,
            views: 0
        }
    }
}

fn main(){
    let new_blog_post = Post::new(
        "Some content".to_string(),
        "Jon Doe".to_string()
    );
}
```

## Your Task

In this challenge, you will implement a constructor for a struct that represents a **Book**. A `Book` will have the following fields:

- `title`: A string that represents the book's title.
- `author`: A string that represents the book's author.
- `year`: An integer that represents the year the book was published.
- `likes`: An integer that represents the number of likes the book has received. Default value is `0`.

The constructor function, `Book::new`, should take three parameters (`title`, `author`, and `year`) and return a fully initialized `Book`.

## Your Task

Implement the `Book` struct and its constructor, ensuring that it correctly initializes all fields.

## Requirements

1. The struct must be named `Book`.
2. Implement a constructor function `Book::new` that:

   - Takes three arguments: `title` (string slice), `author` (string slice), and `year` (integer).
   - Returns a `Book` instance with the specified values and default `likes` value of `0`.

3. Remember to use `pub` for fields (required for testing).

## Example Test

```rust
let book = Book::new("The Rust Programming Language", "Steve Klabnik and Carol Nichols", 2019);

assert_eq!(book.title, "The Rust Programming Language");
assert_eq!(book.author, "Steve Klabnik and Carol Nichols");
assert_eq!(book.year, 2019);
assert_eq!(book.likes, 0);
```
