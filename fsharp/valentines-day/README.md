# Valentine's Day

Welcome to Valentine's Day on Exercism's F# Track.
If you need help running the tests or submitting your code, check out `HELP.md`.
If you get stuck on the exercise, check out `HINTS.md`, but try and solve it without using those first :)

## Introduction

## Discriminated Unions

The discriminated union type represents a fixed number of named cases. Each value of a discriminated union corresponds to exactly one of the named cases.

A discriminated union is defined using the `type` keyword, with cases separated by pipe (`|`) characters:

```fsharp
type Season =
    | Spring
    | Summer
    | Autumn
    | Winter
```

Each case of a discriminated union can optionally have data associated with it, and different cases can have different types of data. If none of the cases have data associated with them, the discriminated union is similar to what other languages usually refer to as an _enumeration_ (or _enum_).

```fsharp
type Number =
    | Integer of int
    | Float of float
    | Invalid
```

Creating a value for a specific case can be done by referring to its name (e.g, `Success`). As case names are just constructor functions, associated data can be passed as a regular function argument. If another discriminated union has defined a case with the same name, you'll need to use its full name (e.g. `Result.Success`).

```fsharp
let byName = Integer 2
let byFullName = Number.Invalid
```

Discriminated unions have _structural equality_, which means that two values for the same case and with the same (optional) data are equivalent.

While one can use `if/elif/else` expressions to work with discriminated unions, the recommended way to work with them is through pattern matching using the _identifier pattern_:

```fsharp
let describe number =
    match number with
    | Integer i -> sprintf "Integer: %d" i
    | Float d  -> sprintf "Float: %f" d
    | Invalid   -> "Invalid"
```

## Instructions

In this exercise, it's Valentine's day and you and your partner are planning on doing something nice together. Your partner has lots of ideas, and is now asking you to rate the ideas, in order to find the activity to engage in.

The following ideas are proposed by your partner:

- Playing a board game
- Chill out
- Watch a movie
- Go to a restaurant
- Take a walk

You have six tasks to help choose your Valentine's day activity.

## 1. Define the approval

For each idea your partner proposes, you respond with one of three options: yes, no or maybe.

Define the `Approval` discriminated union to represent these options as the following three cases: `Yes`, `No` and `Maybe`.

## 2. Define the cuisines

Your partner has selected two possible restaurants: one based on the Korean cuisine and the other based on the Turkish cuisine.

Define the `Cuisine` discriminated union to represent these cuisines as the following two cases: `Korean` and `Turkish`.

## 3. Define the movie genres

There are tons of movies to choose from, so to narrow things down, your partner also lists their genre.

Define the `Genre` discriminated union to represent the following genres as cases: `Crime`, `Horror`, `Romance` and `Thriller`.

## 4. Define the activity

As said, your partner has come up with five different activities: playing a board game, chill out, watch a movie, go to a restaurant and taking a walk.

Define the `Activity` discriminated union to represent these activity types:

- `BoardGame`: no associated data.
- `Chill`: no associated data.
- `Movie`: has its `Genre` as associated data.
- `Restaurant`: has its `Cuisine` as associated data.
- `Walk`: has an `int` representing the number of kilometers to walk as associated data.

## 5. Rate the activity

Finally, you're ready to rate your partner's ideas. This is how you feel about your partner's idea:

- Playing a board game: no.
- Chill out: no.
- Watch a movie: yes if is is a romantic movie; otherwise, no.
- Go to a restaurant: yes if the cuisine is Korean, maybe if it is Turkish.
- Take a walk: yes if the walk is less than three kilometers; maybe if it is less than five kilometers; otherwise, no.

Implement a function named `rateActivity` that takes an `Activity` value and returns the `Approval` based on the above sentiments:

```fsharp
rateActivity (Restaurant Turkish)
// => Maybe
```

## Source

### Created by

- @ErikSchierboom