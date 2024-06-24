---
# You can also start simply with 'default'
theme: dracula
# random image from a curated Unsplash collection by Anthony
# like them? see https://unsplash.com/collections/94734566/slidev
background: https://cover.sli.dev
# some information about your slides (markdown enabled)
title: Introduction to Rust
info: |
    My experience learning and building something with rust for the first time.
# apply unocss classes to the current slide
class: text-center
# https://sli.dev/custom/highlighters.html
highlighter: shiki
# https://sli.dev/guide/drawing
drawings:
  persist: false
# slide transition: https://sli.dev/guide/animations#slide-transitions
transition: slide-left
# enable MDC Syntax: https://sli.dev/guide/syntax#mdc-syntax
mdc: true
# show line numbers in code blocks
lineNumbers: true
---

# Introduction to Rust

By: Trevor Arnold

<div class="abs-br m-6 flex gap-2">
  <a href="https:ithub.com/tlarnold10/rustPresentation" target="_blank" alt="GitHub" title="Open in GitHub"
    class="text-xl slidev-icon-btn opacity-50 !border-none !hover:text-white">
    <carbon-logo-github />
  </a>
</div>

---
transition: slide-left
layout: image-right
image: ./assets/rust-logo.png
---

# What is Rust?

- Owned and managed by the Rust Foundation
    - Currently backed by AWS, Google, Microsoft, Meta, etc.
- Low(er) level language that allows the developer to manage memory.
- Mostly used for systems level programming.
- Statically and strongly typed language (reliability)
- Secure and modern replacement for c and c++

---
transition: slide-left
layout: image-right
image: ./assets/rust.png
---

# Why rust?

- Extremely fast and reliable
- Most admirred language from the 2023 Stack Overflow survey, and pretty high on the desired rank. https://survey.stackoverflow.co/2023/#section-admired-and-desired-programming-scripting-and-markup-languages
- Gaining traction and an active community
- Great cross platform compatability

---
transition: slide-left
layout: image-right
image: ./assets/rust2.png
---

# Why not rust?

- Some difficult concepts to learn
    - Memory management
- Lower developer velocity (takes longer to actually build and ship things)
- Not for UI related application development (native and web)
- Very few job openings as many organizations are interested in the language, but have not yet found a place for it.
- While it is very fast, that is not really needed for a majority of application development. Especially when you look at the lower developer velocity trade-off

---
transition: slide-left
layout: image-right
image: ./assets/rust-logo.png
---

# What I built
- Mad libs
    - Working with files, strings, and hashMap
    - Difficult enough to do in Rust without taking up a significant amount of time or stress
    - Funny

---
transition: fade-out
---

# What I learned
- A little background, I first programmed professionally in Python and first learned coding in college (Java). Been programming in Python, then TypeScript and C#, and then Java.
    - So this is my first experience with lower level memory management.
- Memory management is difficult and is another layer to keep in mind as you are developing
    - Rust's approach: https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html
- Pretty good resources out there, but they vary in quality. Seems like so many different ways to do a certain thing

## Good 
- Compiler catching many issues and (usually) provides great recommendations to fix.
- Learning was actually much easier than I thought it would be
    - The book I read really helped here: The Rust Programming Language

---
transition: slide-left
---

## Bad
- Extremely long compile times and always had some kind of code syle suggestion
    - For example it suggested that I remove the parenthesis in my if statements
- Variable not using snake_case will result in an error...
    - I really prefer camelCase

## Ugly
- Differece between str and String data types
    - Really struggled with all things strings because of the rules with the borrow checker and differences between str and String

```rust {lines:true,startLine:1}
// Creating a String from a string literal. my_string owns the data "hello" on the heap
let mut my_string = String::from("hello"); 

// Appending to a String. my_string can grow and change its content
my_string.push_str(", world!"); 

// Creating a &str from a string literal. hello_str is a &str that points to the data "hello" in static memory
let hello_str = "hello"; 

// Slicing a &str. world_str is a &str that points to a part of hello_str's data
let world_str = &hello_str[3..8]; 
```

---
transition: slide-left
layout: image-right
image: ./assets/rust-logo.png
---

# What next?
- What would I have learned next?
    - lifetimes are a big part of rust and get much more difficult to understand
    - dig deeper into how the borrow checker works
- Would I recommend Rust?
    - not a ton of openings and many companies are trying to understand the actual use case for the language...
    - tons of promise, so worth picking up if you have not used a lower level language

---
transition: slide-up
---

# Thank You!

## Connect: 
[LinkedIn](https://www.linkedin.com/in/trevorarnold/) · [GitHub](https://github.com/tlarnold10) · [Personal Website](https://tlarnold10.github.io/arnold-website/)

