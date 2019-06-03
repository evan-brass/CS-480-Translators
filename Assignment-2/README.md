# Assignment 2
**Due by 11:59pm on Monday, 5/13/2019**

**Demo due by 11:59pm on Monday, 5/27/2019**

In this assignment we'll work on a parser for a small subset of the language Python.  In particular, we'll use the widely-used parser generator Bison to implement a parser that, when combined with the scanner we wrote in assignment 1, will perform syntax-directed translation from Python to C/C++.

There are a few major parts to this assignment, described below.  To get you started, you are provided with a Flex scanner specification in `scanner.l` that solves the problem defined in assignment 1.  There is also a makefile that specifies compilation for the scanner.  A simple `main()` function for the scanner is written in `main.cpp`.

## 1. Modify the scanner to work with Bison

Flex and Bison are designed to easily integrate with each other, but you'll still need to make some modifications to the scanner specification to make it and the parser work together.  These modifications will be easiest to do in stages:

1. Set up a basic Bison parser definition (say in `parser.y`) with no nonterminals.  The main thing you'll need to do is write `%token` directives to specify all of the terminals you'll use in your grammar.  These terminals will correspond directly to the syntactic categories we recognized with the scanner in assignment 1, e.g. `IDENTIFIER`, `FLOAT`, `WHILE`, `PLUS`, etc.  To write these `%token` directives, you'll need to figure out what data type(s) you'll use to represent the different program constructs in the representation output by the parser.  Remember, our end goal with this project is to output C/C++ code corresponding to the Python code being parsed.

2. Once you have your parser definition started, add a compilation step for it in the makefile.  The goal at this point is to generate a header file containing integer values for all of the nonterminals/syntactic categories, so you can include that header file in the scanner and start returning these integer values instead of just printing out syntactic categories.  To generate this header file, add the `-d` option to your `bison` command, e.g.:
    ```
    bison -d -o parser.cpp parser.y
    ```
    This will generate two files, `parser.cpp` and `parser.hpp`, the later of which is the header file to include in the scanner definition.

3. Now, make the scanner return syntactic categories instead of printing them.  For Python, this sounds easier than it actually is.  In particular, under the default setup, a Bison-generated parser exists as a function `yyparse()`.  This function repeatedly calls the scanning function `yylex()` that's generated by our Flex specification, and, on each call, it expects `yylex()` to return the integer code for the syntactic category of the next word in the source program.

    Thus, you might be tempted to simply replace all of the `cout` statements in the scanner that print syntactic categories into `return` statements that just return those syntactic categories instead.  This would work for all but a small few cases.  In particular, there are a few situations where a single call to the scanner could generate *multiple* tokens.  Specifically, when a program is dedented by multiple levels at once, we need our scanner to be able to return multiple `DEDENT` tokens from a single Flex rule.  This cannot be done with a simple return statement.  There are at least two ways to solve this problem:

      1. **Use a queue to store tokens to return.**  Every time a token is generated in the scanner, place it into a queue instead of returning it.  Then, at the beginning of each call to `yylex()`, first check the queue to see if there are any tokens waiting to be returned.  If there are, simply dequeue the first token and return it.  Note that under this approach, you may need to do some extra work to be able to return the *lexeme* along with each syntactic category that's returned, since this is needed for some syntactic categories like `IDENTIFIER`.  One possibility would be to store lexeme/syntactic category pairs in your queue.

      2. **Implement a push parser.**  The default model implemented by a Bison-generated parser is to "pull" tokens from the scanner by calling `yylex()` each time a new token is needed.  A push parser reverses these roles so that `yylex()` is called only once and now "pushes" a token to the parser each time a new token becomes available.  It does this by calling the function `yypush_parse()` with the new token passed as an argument.  Under the push-parsing paradigm, it doesn't matter if the scanner generates multiple tokens at a time, since each one can be pushed to the parser in turn.  You can read more about how push parsers work in Bison here:

          https://www.gnu.org/software/bison/manual/bison.html#Push-Decl

## 2. Implement grammar rules to recognize Python constructs

Once your scanner is able to generate one token at a time, either via a token queue or via push parsing, you are ready to write some grammar rules to recognize constructs in the Python language.  At this point, you don't need to worry about attaching actions to these rules.  You can just get your grammar in place.

The grammar you write will need to recognize a simplified subset of Python.  In particular, your grammar should be able to recognize a program comprised of the following kinds of statements:

* **Assignment statements.**  These are statements where the value of an expression is assigned to a specific variable, e.g.:
    ```python
    circumference = pi * 2 * r
    ```
    In the subset of Python we'll implement, no assignment statement will span more than a single line of code, and each statement will be terminated by a newline (i.e. lines won't be broken with a `/` character, as they can be in actual Python syntax).  The expression on the right-hand side of the assignment can be any valid expression involving identifiers, floats, integers, or booleans and the following operators: `+`, `-`, `*`, `/`, `==`, `!=`, `>`, `>=`, `<`, `<=`, `not`.  Expressions may also contain parentheses `()`.

* **If-elif-else statements.**  In Python these look like the following:
    ```python
    if a:
      x = 2 * y
    elif b <= 7:
      x = 3 * y
    else:
      x = 4 * y
    ```
    Of course, the `elif` and `else` parts are both optional.  The statement could also include more `elif` blocks.  Importantly, all of the statements to be executed for each of the `if`, `elif`, and `else` conditions are indented to the same level.  In other words, each block is contained within a matching `INDENT`/`DEDENT` pair.  Also, for this assignment, every one of these blocks will be preceded by a newline.  In other words, another statement cannot be included on the same line as the `if`, the `elif`, or the `else`.  For this assignment the conditions for `if` and `elif` statements can be any valid expression or any boolean combination of expressions using the `and` and `or` operators.

* **While statements.**  These are similar to `if` statements, e.g.:
    ```python
    while i < 10:
      i = i + 1
    ```
    Again, the block of statements to be executed in each iteration of the while loop will be contained within a matching `INDENT`/`DEDENT` pair and will be separated from the `while` statement with a newline.  Again, the termination conditions for `while` statements can be any valid expression or any boolean combination of expressions using the `and` and `or` operators.

* **Break statements.** These simply consist of the keyword `break` followed by a newline, i.e.:
    ```python
    break
    ```

For this assignment, some things you specifically *do not* need to worry about are:
  * For loops.
  * Function definitions and function calls.
  * Arrays and dictionaries.

## 3. Assign actions to your grammar rules to perform syntax-directed translation into C/C++

Syntax-directed translation is essentially compilation by the parser.  In other words, in a syntax-directed translation scheme, the parser directly outputs the target program.  Our goal for this assignment is to perform syntax-directed translation from Python into C or C++.  In other words, your parser must output a working C/C++ program that performs the same computation as the input Python program.

Once you have your grammar defined, you can begin to attach actions to your rules to perform the syntax directed translation.  The easiest approach here will be to use the information you gain from the rules of your grammar about constructs recognized in the source program to generate corresponding C/C++ language strings for those constructs.  In this way, at the end of the parse, your grammar's goal symbol will refer to a string containing the entire translated target program.

A few things to consider while you're performing the syntax-directed translation:

* Your parser should generate a *working* C/C++ program, so it will need to contain boilerplate things like `#include` statements and a `main()` function.  It will probably be easiest if you don't worry about adding things like `#include <iostream>` or wrapping your target program within a `main()` function until the parse is complete.  If your parse simply translates a sequence of Python statements into a corresponding sequence of C/C++ statements, you can wrap this translated sequence in a `main()` function at the very end.

* In order to generate a working C/C++ program, you'll also need a variable declaration for each variable used in the program.  To do this, you can maintain a simple symbol table, where each variable identifier is stored when it's first encountered.  When your parse is finished, you can simply iterate through the identifiers stored in the symbol table and, for each identifier, generate a variable declaration at the top of your `main()` function.

    There are a couple simplifying assumptions you can make for the purposes of this assignment to make this a little easier:

    * Every variable will appear as the left-hand side of an assignment statement before it is used anywhere else.

    * All variables can be scoped to the `main()` function.  You don't need to worry about scoping variables within blocks (e.g. inside of an `if` block).

    * All variables can have the same type, e.g. `double` or `float`.

* So you can tell what's happening with your translated code, you should also generate one `printf()`/`cout` statement at the end of your `main()` function for each variable to print the value of that variable at the end of the execution of the translated program.  For example, say you have the following simple Python program:
    ```python
    five = 2 + 2
    ```
    If you are translating to C++, your parser should output a program that looks like this (though you don't need to match the indentation of this program; it's included only for clarity):
    ```c++
    #include <iostream>
    int main() {
      double five;
      five = 2 + 2;
      std::cout << "five: " << five << std::endl;
    }
    ```

* If the source program contains one or more syntax errors, you should not output a target program.  Instead, you should report at least the first encountered syntax error.

* Don't worry about indentation in your generated target program.  Everything can be unindented.

Once you get your translation fully working, you should be able to use `gcc`/`g++` to compile and run the generated target program, provided the source program contains no syntax errors.

## 4. Make sure your makefile fully generates your parser

You should be able to type `make` to generate an executable parser from your scanner and parser specifications.

## Testing your parser

There are some simple Python programs you may use for testing your parser included in the `testing_code/` directory.  Some of these programs (i.e. `p*.py`) are syntactically valid, and your parser should be able to translate them successfully.  There are example translations for these programs included in the `example_output/` directory.  Some of the programs in `testing_code/` (i.e. `error*.py`) contain various syntax errors.  Your parser should fail to translate these programs.

## Submission

We'll be using GitHub Classroom for this assignment, and you will submit your assignment via GitHub.  Make sure your completed files are committed and pushed by the assignment's deadline to the master branch of the GitHub repo that was created for you by GitHub Classroom.  A good way to check whether your files are safely submitted is to look at the master branch your assignment repo on the github.com website (i.e. https://github.com/osu-cs480-sp19/assignment-2-YourGitHubUsername/). If your changes show up there, you can consider your files submitted.

## Grading criteria

The TAs will grade your assignment by compiling and running it on one of the ENGR servers, e.g. `flip.engr.oregonstate.edu`, so you should make sure your code works as expected there.  `bison` and `flex` are installed on the ENGR servers.  If your code does not compile and run on the ENGR servers, the TAs will deduct at least 25 points from your score.

This assignment is worth 100 points total, broken down as follows:
  * 30 points: scanner is modified to correctly return tokens to the parser
  * 30 points: grammar rules are correctly set up for the subset of Python described above
  * 35 points: parser successfully performs syntax-directed translation, as described above
  * 5 points: makefile is specified to fully generate an executable parser