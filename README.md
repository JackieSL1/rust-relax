## What is rust-relax?
rust-relax is a small command line relational algebra calculator written in Rust based off of (RelaX)[https://dbis-uibk.github.io/relax/landing]. Supported operations include:
* Table creation
* Selection (select)
* Projection (project)
* Inner join (join)
* Left outer join (leftJoin)
* Right outer join (rightJoin)
* Full outer join (fullJoin)
* Set Union (union)
* Set Compliement (-)
* Set Division (/)
* Cartesian Product (*)

## How do I use it?
First, clone the repo onto your machine
```
$ git clone https://github.com/JackieSL1/rust-relax
```

Make sure you have Rust & Cargo installed. From inside rust-relax, run the project with cargo
```
$ cd rust-relax
$ cargo run
```
