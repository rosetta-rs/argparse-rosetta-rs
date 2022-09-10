# Design Trade-offs

This will be looking at CLI parser design trade offs from the lens of comparing
[bpaf](https://docs.rs/bpaf) and [clap](https://docs.rs/clap).

For anyone asking the question "which should I use?", the short answer would be
- `bpaf` would work well for simple cases like example bins for a library
  while giving the best build times and a reasonable to understand code with
  the `derive` API
- All other cases the answer is "it depends" as the two designs offer different
  trade offs.   If you want to just choose one with little research that will
  cover the most use cases, that will most likely be `clap`.

## Static vs Dynamic Typing

`bpaf`'s combinator API uses generics and macros to build a statically-typed
parser, pushing development errors to compile time.  This minimizes binary
size and runtime, only paying for what you use.

The combinator approach tends towards yoda-speak though with [careful
structuring by breaking down arguments into
functions](https://github.com/pacak/bpaf/blob/aa6992931bbfbdca6390c87f4a76898f8db0ae47/examples/top_to_bottom.rs),
a more straightforward ordering can be accomplished.

`clap`'s builder API stores parsed values in a Map-like container where the
keys are the argument IDs and the values are effectively `Box<dyn Any>`.
- This allows dynamically created CLIs, like with
  [clap_serde](https://docs.rs/clap_serde) or [conditionally-present
  flags](https://github.com/sharkdp/bat/blob/6680f65e4b25b0f18c455f7a4639a96e97519dc5/src/bin/bat/clap_app.rs#L556)
- Callers can dynamically iterate over the arguments, like for layered configs.
- `clap` can have ready-to-use, arbitrarily deifned defaults and validation
  conditioned on argument values (e.g. `default_value_if`).  See the section on
  Validation for more nuance on the "arbitrarily" part
- `clap` tries to help push as many errors to early test time with minimal
  effort with the `cmd.debug_assert()` function.  Value lookups still requires
  full coverage of that code to catch errors.

Which design approach is faster to build will be dependent on the exact
implementation and the compiler.

## Context-sensitive parsing

Parsing arguments is context sensitive.  Consider:
```console
$ prog --one two --three --four -abcdef
```
- When is an argument a subcommand, a positional value, a flag, or a flag's value?
- When is the trailing parts of a short flag an attached value or more shorts flags?

For example, some possible interpretations of the above could be:
```console
$ prog --one=two --three=--four -a=bcdef
$ prog two -a -b -c -d -e -f --one --three --four
```

`clap` parses left-to-right using [`clap_lex`](https://docs.rs/clap_lex), with
the output from the previous token hinting how to parse the next one to
prevent ambiguity.  This leads to a fairly complicated parser to handle all of
the different cases upfront, including some usability features to help catch
developer mistakes.

`bpaf` instead does context-free tokenization, storing all flags and values in
a list.
- Tokens that look like flags can only be considered flags, unless escaped with
  `--`, not allowing cases like `fd --exec rm -rf ;`
- Short flags can only have attached values in the `-a=b` case (not even `-ab=c`)
- An argument is resolved as either a subcommand, positional value, or a flag's
  value based on the order that the they are processed (as determined by their
  definition order)

While `bpaf` supports fewer use case / CLI styles, it has an overall simpler
parser that then scales up in runtime, build time, and code size based on the
arguments themselves, only paying for what you use.  In addition, some
context-sensitive cases can be reproduced using the argument combinators (e.g.
`--color [WHEN]`).

## Validation

Specifically when arguments conflict, override, or require each other.

`bpaf` uses function/macro combinators to declare argument / group of argument relationships.  This
offers a lot of flexibility that, again, you only pay for what you use.

The downside to this approach is it couples together:
- Parsing disambiguation with positionals
- Validation (in a strict tree structure)
- Help ordering
- Help sections headers
- Code structure building up the combinators
- Data structures as each level is its own `struct`
- Organizing some of the validation rules aren't always as intuitive for people
  not steeped in a functional way of thinking

`clap` provides `ArgGroup` to compose arguments and other groups with
relationships defined in terms of either group or argument IDs.  `ArgGroup`s
(and the separate help section header feature) are tags on arguments.  This
allows a very flexible directed acyclic graph of relationships.

The downside to this approach
- Everyone pays the runtime, compile time, and code size cost, no matter which subset they are using
- Developers are limited by what relationships `clap` has predefined
- Even once `clap` opens up to user-provided relationships, the ergonomics for
  defining them won't be as nice as it will require implementing a trait that
  uses the lower levels of clap's API and then passing it in to the parser

## Derive APIs

Both libraries provide derive APIs that mask over the static and dynamic typing differences.

In `bpaf`s case, the combinators still show through in terms of requiring the
user to organize their data structures around their validation.  Some times
this is good (pushing errors to compile time like if mutually exclusive
arguments are represented in an `enum`) while at other times it has the potential to convolute the code.

In `clap`s case, it has the challenge of hand-implemented support to express
each case of argument relationships in the type system (which hasn't been done
yet).

In both cases, some errors are still pushed off from compile time to early test
time through asserts.

## Maturity

While this document is focused on design trade-offs, we understand some users
will look to this to help understand which would work better for them.

`clap` has been around for many more years and has a lot more users from
varying backgrounds solving different problems and `clap` has taken input and
adapted to help meet a variety of needs.

`bpaf` is a younger project but it is able to move a lot more quickly because
of the fewer requirements it is trying to fulfill.  `bpaf` already covers
common cases for creating a polished CLI.

An exact feature-by-feature comparison is out of the scope as `clap` and `bpaf`
are both constantly evolving.
