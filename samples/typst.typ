// Typst Document Example

#set document(
  title: "Sample Typst Document",
  author: "Author Name",
  date: datetime.today(),
)

#set page(
  paper: "a4",
  margin: (x: 2.5cm, y: 3cm),
  header: [
    _Typst Example_
    #h(1fr)
    #counter(page).display()
  ],
)

#set text(
  font: "New Computer Modern",
  size: 11pt,
  lang: "en",
)

#set heading(numbering: "1.1")

#show link: underline

// Title
#align(center)[
  #text(size: 24pt, weight: "bold")[
    Welcome to Typst
  ]
  #v(1em)
  #text(size: 14pt)[A modern typesetting system]
]

= Introduction

Typst is a new markup-based typesetting system designed to be as powerful as
LaTeX while being much easier to learn and use.

== Basic Formatting

You can make text *bold*, _italic_, or *_both_*. You can also use `inline code`
and #highlight[highlighted text].

Subscripts work like H#sub[2]O, and superscripts like E = mc#super[2].

== Lists

Unordered list:
- First item
- Second item
  - Nested item
  - Another nested item
- Third item

Numbered list:
+ First step
+ Second step
+ Third step

== Mathematics

Inline math: $x = (-b plus.minus sqrt(b^2 - 4 a c)) / (2 a)$

Display math:
$ integral_0^infinity e^(-x^2) dif x = sqrt(pi) / 2 $

Matrix:
$ mat(
  1, 2, 3;
  4, 5, 6;
  7, 8, 9;
) $

== Code Blocks

```rust
fn main() {
    println!("Hello, Typst!");
}
```

== Tables

#table(
  columns: (1fr, auto, auto),
  inset: 10pt,
  align: horizon,
  [*Name*], [*Age*], [*City*],
  [Alice], [28], [New York],
  [Bob], [34], [London],
  [Carol], [25], [Tokyo],
)

== Functions and Variables

#let greet(name) = [Hello, #name!]
#greet("World")

#let project-name = "My Project"
This is the #project-name documentation.

== Figures

#figure(
  rect(width: 100%, height: 3cm, fill: gradient.linear(blue, green)),
  caption: [A colorful gradient],
) <gradient-fig>

See @gradient-fig for an example.

== Bibliography

#bibliography("refs.bib", style: "ieee")
