# Learning Rust!

I found an old repository with [homeworks](https://github.com/cis198-2016s)
and it looks like a great way to learn! I'll also add some random tasks.

To create a new project:

```bash
$ cargo new --bin <project>
```

Then to run/build it:

```bash
$ cd <project>
$ cargo run
$ cargo build
```

That's pretty much where I am :)

## Exercism

[Exercism.io](https://exercism.io)  You'll first want to update
your workspace to be the repository;

```bash
$ exercism configure --workspace $PWD 
```

And then confirm it is correct!

```bash
$ exercism workspace
/home/vanessa/Desktop/Code/learning-elm
```

For each exercise, we will want to download code like:

```bash
exercism download --exercise=hello-world --track=elm
/home/vanessa/Desktop/Code/learning-elm/elm/hello-world
```

The exercism clones are in subfolders of [rust](rust).
