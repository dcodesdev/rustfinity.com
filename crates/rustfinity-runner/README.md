# Rustfinity code runner

The official code runner used in [Rustfinity.com](https://www.rustfinity.com), a platform to learn Rust programming language.

## How to use

It's best to run this in a docker container to be completely isolated from the outside world.

1. Build the image locally

   ```sh
   make build TAG=latest
   ```

2. Convert your code to **base64** format.

   In JavaScript it would be something like this

   ```js
   const code = `fn main() {
       println!("Hello, world!");
   }`;

   const base64Code = Buffer.from(code).toString("base64");
   ```

3. Run the code using the CLI

```bash
docker run -i \
    --rm \
    --network=none \
    --cpus=1 \
    -m=500m \
    rustfinity-runner \
    /bin/bash -c "/app/rustfinity-runner run --code 'cHViIGZuIGhlbGxvX3dvcmxkKCkgewogICAgcHJpbnRsbiEoIkdvb2Qgam9iLCB5b3UgZGVjb2RlZCBpdCA6RCIpCn0K' --challenge 'printing-hello-world'"
```

### Arguments

- `--code`: The base64 encoded code to run.
- `--challenge`: The challenge name. This is used to identify the challenge in the logs.
- `--n-tests` or `n`: The number of tests to run (takes the minimum amount of time in `ms` and prints it).
