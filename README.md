# reproducing an S3 object store timeout issue

To run, you'll need:

- An s3 bucket
- s3 credentials to access the bucket
- A file about ~100MB in size in that bucket

Create a .env file in this directory with your bucket credentials, path, and desired number of
concurrent tasks:

```
AWS_ACCESS_KEY_ID=[insert access key here]
AWS_SECRET_ACCESS_KEY=[insert secret access key here]
AWS_DEFAULT_REGION=[insert region here]
AWS_BUCKET=[insert bucket name here]
OBJECT_STORE_PATH=[insert path within bucket here]
NUM_CONCURRENT_TASKS=1
```

Then `cargo run`. You have reproduced the issue if you see output containing errors:

```
starting 0
result for task 0: Err(Generic { store: "S3", source: reqwest::Error { kind: Decode, source: reqwest::Error { kind: Body, source: TimedOut } } })
...
```

Conversely, you have not reproduced the issue if all you see is Oks, which is what I see if I set
`OBJECT_STORE_PATH` to the path of a smaller file:

```
starting 0
result for task 0: Ok(1327644)
starting 1
result for task 1: Ok(1327644)
```