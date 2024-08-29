# cis1905 Project 0

## Read Before You Start

1. This assignment is due on **Friday, September 6th at 10pm**.

2. When you are finished, submit this repository on Gradescope. You can
   resubmit as many times as you like until the deadline.

3. Submissions should not have compilation warnings or errors.

4. See the course syllabus for the collaboration policy. All code
   submitted should be your own.

5. For an easy and feature-complete Rust editing experience, consider using
   VSCode with the
   [rust_analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
   extension. You are, of course, welcome to use any editor of your choice, but
   we can only guarantee the ability to help you debug editor configuration
   issues for VSCode.

6. As always, questions about the assignment of any form are welcome on EdStem.
   If you are posting a question with code you've written for an assignment,
   make sure to make it private.

## Introduction

This assignment consists of a number of programming exercises to get you familiar with writing Rust code and using the Rust compiler.

## Part 0: Rust Installation

If you haven't already, [install
Rust](https://www.rust-lang.org/tools/install). Once you've followed these
directions, you'll have the Rust compiler `rustc` and the Rust build tool
`cargo` available on the command line. Open a new terminal and run the
following command to verify you've installed Rust successfully.

```shell
$ rustc --version
rustc 1.80.0 (051478957 2024-07-21)
```

(Your version number may be different. If it is lower than 1.80.0, run `rustup update`)

Once you've verified your Rust install, you're ready to download the
assignment. We'll distrbute assignments using Github template repositories. To
get started, click the green "Use this template" button at the top of this
page and then "Create a new repository".

![copying a template repository](https://docs.github.com/assets/cb-76823/mw-1440/images/help/repository/use-this-template-button.webp)

Once you've created your repository, clone it to your local machine and open a
terminal in the newly cloned directory. Change to the `part1` directory and run `cargo test` to verify that everything is
working correctly (the tests will be failing for now).

> ⚠️ **If you have issues with anything in this section, post on EdStem or come to office hours.** Course staff can help debug issues with your Rust installation.

## Part 1:

We'll start by implementing some small functions to get you familiar with Rust
syntax. between the first lecture and its textbook reading we've covered everything you
need, but if you get stuck feel free to use Google to supplement your
understanding.

> Note: _If you usually use Github Copilot or a similar editor-integrated tool, I recommend disabling it for this part of the assignment. The goal is to get practice writing Rust code and working through compiler errors that you might not be used to from other languages, and Copilot can shortcut that process and make you miss out on learning opportunities. Of course, ultimately the choice is up to you so long as you follow the [usage of AI policy](https://www.cis.upenn.edu/~cis1905/2024fall/syllabus/#usage-of-ai)_.

### sqrt

Implement the square root function using the stencil in `lib.rs`. You may
implement this function however (in)efficiently you like, so long as you don't
use any built-in square root functions.

Some constructs you might find helpful:

1. Using a for loop to iterate through a range of numbers.
```rust
let n = 10;
for i in 0..n {
    println!("{}", i);
}
```
2. Using `loop` to create an infinite loop.
```rust
let mut i = 0;
loop {
    println!("{}", i);
    i += 1;
    if i == 10 {
        break;
    }
}
```

Tests for this function can be run with

```
cargo test sqrt
```

### binary_search

**Slices:**

In Rust, we can write a fixed size array type like this:

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5]; // an array of 5 signed 32-bit integers
```

We can access elements of the array using the index operator `[]`.

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first_element: i32 = arr[0]; // first_element is 1
```

> Recall that the type annotations are optional. We include them in READMEs to make the code easier to understand.

Additionally, we can take a subset of an array by using a range of indices.

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4]; // slice is [2, 3, 4]
```

This new type is called a slice and is written as `&[T]` where `T` is the type
of the elements in the slice. Slices are just a view of the original array, so
they can only hang around as long as the original array does. We will make this
idea more concrete in future lectures. For now, just know that slices have the
standard operations you would expect:

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let slice: &[i32] = &arr[1..4];
let first_element: i32 = slice[0]; // access an element
let new_slice: &[i32] = &slice[1..2]; // re-slice a slice
let length: usize = slice.len(); // get the length of a slice
```

**Options:**

We often want to express the idea that a value is either present or missing.
For example, trying to access an index of a list can either succeed and return
a value, or fail if the index is out of bounds. This is handled in a variety of
ways. In C, we might have:

```C
float *get(float_list l, int index);
```

Where the function returns a null pointer if the index is out of bounds and
a pointer to a value if the index is in bounds. In
languages without explicit pointers like Python, we can return a special value
`None`.

The downside to both of these approaches is that it's impossible to opt-out of
values potentially being null. There's no way to say in the type of a function
that it returns a non-null pointer, so the caller of that function has to check
for null every time they use it.

In Rust, there is no language-wide null value. Instead, we can use the
`Option<T>` type to express that a value might be null. For example, the `get`
function from earlier could be implemented as follows:

```rust
fn get(list: &[f32], index: usize) -> Option<f32> {
    if index < list.len() {
        let f = list[index];
        Some(f)
    } else {
        None
    }
}
```

Note that instead of returning the float value directly, we must wrap it in a
`Some(...)` to make it of type `Option<f32>`. Conversely, we can use `None`,
another value of type `Option<f32>`, to represent the absence of a value. To
use the result, we can use pattern matching.

```rust
let list = [1.0, 2.0, 3.0];

match get(&list, 1) {
    Some(f) => {
        println!("The value at index 1 is {}", f);
    },
    None => {
        println!("The index is out of bounds");
    },
}
```

**Binary Search:**

Using your new found knowledge of slices and options, implement binary search
using the provided stencil in `lib.rs`. The function signature is as follows:

```rust
pub fn binary_search(arr: &[i32], query: i32) -> Option<u32>
```

The function should return `Some(index)` if and only if `arr[index]==query`. If
the query is not in the array, the function should return `None`.

Tests for this function can be run with

```
cargo test binary_search
```

### Rainfall

The rainfall problem is a famous problem in CS education research used to test
students' understanding of certain foundational skills. The original formulation is as follows:

> Design a program called rainfall that consumes a list of numbers representing daily rainfall amounts as entered by a user. The list may contain the number -999 indicating the end of the data of interest. Produce the average of the non-negative values in the list up to the first -999 (if it shows up). There may be negative numbers other than -999 in the list.

Implement the rainfall function using the provided stencil in `lib.rs`. You might find using a for loop to iterate through a slice helpful:

```rust
fn print_arr(arr: &[i32]) {
   for n in arr {
       println!("{}", n);
   }
}
```

Tests for this function can be run with

```
cargo test rainfall
```

## Part 2:

Your goal is to implement a command-line hangman game. The following is an example of a possible run of the game:

```
Welcome to cis1905 Hangman!
The word so far is -------
You have 5 guesses left
Please guess a letter: r

The word so far is ------r
You have 5 guesses left
Please guess a letter: s

The word so far is ---s--r
You have 5 guesses left
Please guess a letter: t

The word so far is ---st-r
You have 5 guesses left
Please guess a letter: l

The word so far is l--st-r
You have 5 guesses left
Please guess a letter: a
Sorry, that letter is not in the word

The word so far is l--st-r
You have 4 guesses left
Please guess a letter: b

The word so far is l-bst-r
You have 4 guesses left
Please guess a letter: c
Sorry, that letter is not in the word

The word so far is l-bst-r
You have 3 guesses left
Please guess a letter: o

The word so far is lobst-r
You have 3 guesses left
Please guess a letter: e

Congratulations you guessed the secret word: lobster!
```

Alternatively, you may not have gotten the word:

```
Welcome to cis1905 Hangman!
The word so far is --------
You have 5 guesses left
Please guess a letter: a

The word so far is --a-----
You have 5 guesses left
Please guess a letter: b
Sorry, that letter is not in the word

The word so far is --a-----
You have 4 guesses left
Please guess a letter: c

The word so far is c-a-----
You have 4 guesses left
Please guess a letter: d
Sorry, that letter is not in the word

The word so far is c-a-----
You have 3 guesses left
Please guess a letter: e
Sorry, that letter is not in the word

The word so far is c-a-----
You have 2 guesses left
Please guess a letter: f

The word so far is c-a-f---
You have 2 guesses left
Please guess a letter: g
Sorry, that letter is not in the word

The word so far is c-a-f---
You have 1 guesses left
Please guess a letter: h

The word so far is c-a-f--h
You have 1 guesses left
Please guess a letter: i

The word so far is c-a-fi-h
You have 1 guesses left
Please guess a letter: j
Sorry, that letter is not in the word

Sorry, you ran out of guesses!
```

The program exits either when you correctly complete the word or when you run out of guesses.

Notes:

- To print text without a newline, use `print!` instead of `println!`.
-
- To compare values for equality, use `==`.

- To read input from the user, you can write something like this:

```
print!("Please guess a letter: ");
// Make sure the prompt from the previous line gets displayed:
io::stdout()
    .flush()
    .expect("Error flushing stdout.");
let mut guess = String::new();
io::stdin()
    .read_line(&mut guess)
    .expect("Error reading line.");
```

- To access a character at a specific index in a string, you can use the
  `chars` method. This only works for ASCII characters, we'll talk about more
  complicated strings in the future.

```rust
let s: String = ...;
match s.chars().nth(0) {
   Some(c) => {
       println!("The first character is {}", c);
   },
   None => {
       println!("The string is empty");
   },
}
```

- You don't need to match the above output exactly. Just make a game that works!
  The requirements are that your game is playable to completion in both the success
  and failure cases without crashing given reasonable input.
- We don’t expect you to worry about error-handling or re-prompting. However,
  doing so will give you better practice with the language.
- We haven’t talked in-depth about Options or Results yet, which are a big part of Rust error handling
  and will be the topic of lecture 2. If you Google how to do things,
  you may encounter suggestions that involve calling functions like `unwrap()`.
  You’re welcome to use such code, but there is probably a simpler way to do
  it. Feel free to ask on EdStem.
- Make a habit of compiling often and reading the compiler error messages closely.
  The errors often have suggested fixes that can be helpful.
- If you are working on this section before lecture 2, you may have a hard time
  writing helper functions that take strings as input or return strings as output.
  Consider writing everything in `main` or waiting until after lecture 2 if you
  feel like this limitation is halting your progress.

## Wrapping Up

When you are finished, submit this repository to Gradescope. Congratulations on completing the first assignment!

---

Credits:

_Portions of this assignment were adapted from CS110L at Stanford and cmsc388Z at UMD_
