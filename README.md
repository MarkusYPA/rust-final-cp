# Rust Final CP

Practice solving exercises listed to appear in the exam.

## How to Use

1. Clone the repository to your computer
2. Choose an exercise and navigate to its folder
3. Read the README
4. Start editing src/lib.rs directly, or paste template.txt over everything above #[cfg(test)] (see Notes).
5. Run the code with ```cargo run```
6. Test the code with ```cargo test```
7. Look at the solution text files for examples

## Excercise Folder Contents

- README.md: The exercise as it appears on the platform

src/ subfolder:  
- lib.rs: Write your answer here (exception: brackets_matching, where you write it in main.rs)
- main.rs:  Runs your code with modifiable inputs
- tests.rs:  Actual tests the code needs to pass
- solution01edu.txt: Example solution from the system
- solutionm.txt: My solution
- template.txt: The empty base for the exercise


## Notes

Copy contents of template.txt over lib.rs (keep the tests bit at the bottom!) to be sure to have the exercise as it will appear on the platform. lib.rs has often been modified to avoid VSCode's rust-analyzer giving red compiler error warnings.

## Exercises by Level

**Level 1**  
matrix-multiplication  
previousprime  
smallest  

**Level 2**  
counting-words  
modify-letter  
partial-sums  
reverse-it  

**Level 3**  
count-factorial-steps  
format-me  
get-document-id  
lucas-number  

**Level 4**  
display-table  
dress-code  
matrix-determinant  
matrix-display  
profanity-filter  
rot21  

**Level 5**  
blood-types-s  
brackets-matching  
car-rental  
filter-table  
lunch-queue  
moving-targets  
negative-spelling  
office-worker  

## Sources
- [Exercises](https://github.com/01-edu/public/tree/master/subjects)
- [Solutions and tests](https://github.com/01-edu/rust-tests)
