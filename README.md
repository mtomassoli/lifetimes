# Lifetime annotations: why doesn't Rust?

After having played a little with [*GATs* and *HKTs*](https://github.com/mtomassoli/HKTs), I had a look at some of the exercises on [https://practice.rs/](https://practice.rs/). In particular, I experimented a little with the [Hard exercise](https://practice.rs/lifetime/advance.html#a-difficult-exercise) at the end of the section about *lifetimes*.

Now I have a question and no time to try to answer it on my own, as my 7-day adventure with Rust is sadly coming to an end :(

My initial intention was just to ask my question, but I ended up writing a full-fledged article. To make it more self-contained, I even decided to put to words my understanding of lifetimes since it might help other beginners.

I think annotating *lifetimes* is the hardest part for a newcomer to Rust.

My purpose with this article is thus threefold:

* explain the concept of *lifetimes*, *lifetime annotations* and *lifetime bounds*, the way I understand them;
* show how one could solve, mechanically, the [Hard exercise](https://practice.rs/lifetime/advance.html#a-difficult-exercise);
* wonder why Rust doesn't do that automatically for us. This is a genuine question of mine for the experts of [r/rust](https://www.reddit.com/r/rust).

## Lifetime and lifetime annotations

* Lifetimes are associated with references.
  * The *lifetime* of a reference is the *interval of time* during which the reference is valid.
* In Rust, a *generic lifetime* is indicated with an apostrophe followed by a (usually) short name: `'a`, `'b`, `'c1`.
  * These symbols are called *lifetime annotations*.
* `&'a T` and `&'a mut T` are references, with generic lifetime `'a`, referencing a generic type `T`.
  * Technically, `&'a T` and `&'a mut T` are *type constructors*:
    * we can construct a concrete type by replacing the generic type parameters with concrete types.
    * Note, though, that
      * although we can indicate concrete types such `f64`,
      * we can't indicate concrete lifetimes,
        * except for `'static`,
          * which is basically the lifetime of the whole program.
          * The name comes (probably) from the *static memory*, which:
            * is created and initialized (it's read-only) when the program starts, and
            * is destroyed when the program ends.

Lifetime annotations are used to keep track of lifetimes with the ultimate goal of avoiding *dangling references*, i.e. references that reference invalid memory.

Lifetime annotations are only used to *make sure* that good code is actually good. They will ***never*** turn bad code into good code. In particular, a lifetime annotation will ***never*** make a reference live longer. A lifetime annotation makes the compiler *realize* that a reference lives long enough.

## Lifetime bounds

Note that I write

* "an `X`" as short for "an object of type `X`",
* "`X`s" as short for "objects of type `X`",
* and so on...

I'll also say "`X` replaces `Y`" instead of "`Y` is replaced with `X`", because the former doesn't swap the order of the terms in `X : Y`. Hopefully, that's correct English.

In general:

* `A` is a subtype of `B` when `A`s can always replace `B`s.
* This is written as `A : B`, in Rust.

More precisely, `A : B` *if and only if*, for all `b` of type `B`, any surrounding code that works with `b` will also work with any `A`.

Of course, in this context, the verb "work" only indicates the absence of type errors.

In what follows I'll use "an `'a`" as short for "a reference which is valid (at least) during `'a`".

* Being a lifetime a *set* of points in time,
  * there's a natural *inclusion relation* between lifetimes,
  * which is used to express a *subtyping relation* between lifetimes.
* In Rust, we write `'a : 'b` to indicate that:
  * `'a` is a *superset* of `'b`:
    * if a time point is in `'b` than it's also in `'a`
  * `'a` is a *subtype* of `'b`:
    * the replacement requirement for subtyping is satisfied:
      * the *surrounding code* of a `'b` only accesses the reference during `'b`;
      * an `'a` is valid during `'a`, which includes `'b`, so it's also valid during `'b`,
        * meaning the *surrounding code* only accesses the `'a` when it's valid.

## Variance

It's also important to know about *variance*.

Assuming the `...` parts don't change, we can say that `TC(..., T, ...)`:

* is *covariant* in `T` if `A : B` implies `TC(..., A, ...) : TC(..., B, ...)`
* is *contravariant* in `T` if `A : B` implies `TC(..., B, ...) : TC(..., A, ...)`
* is *invariant* in `T` otherwise

Here are a few (abstract) examples, where I'll assume `Dog : Animal` and `'long : 'short`:

* A function `(T1, T2, T3) -> R` is
  * *contravariant* in `T1`, `T2`, and `T3` because, for instance,
    * the surrounding code of a `(Dog, T2, T3) -> R` only passes `Dog`s in `T1` position
    * `(Animal, T2, T3) -> R` can accept `Dog`s in `T1` position,
    * so an `(Animal, T2, T3) -> R` can replace a `(Dog, T2, T3) -> R`.
  * *covariant* in `R`:
    * the surrounding code of a `(T1, T2, T3) -> Animal` expects `Animal`s from the function
    * `(T1, T2, T3) -> Dog` returns an `Animal`,
    * so a `(T1, T2, T3) -> Dog` can replace a `(T1, T2, T3) -> Animal`.
* A container `Container<T>` is
  * *covariant* in `T` ***if read-only***:
    * the surrounding code of a `Container<Animal>` reads `Animal`s from it
    * and a `Container<Dog>` only contains `Animal`s,
    * so a `Container<Dog>` can replace a `Container<Animal>`.
    * (Notice that the opposite direction doesn't work.)
  * *contravariant* in `T` ***if write-only***:
    * the surrounding code of a `Container<Dog>` puts `Dog`s in it
    * and a `Container<Animal>` can contain `Dog`s,
    * so a `Container<Animal>` can replace a `Container<Dog>`.
    * (Notice that the opposite direction doesn't work.)
  * *invariant* in `T` ***if read-write***:
    * it would need to be both *covariant* and *contravariant*, which is impossible.
* References `&'a T` and `&'a mut T` are both
  * *covariant* in `'a`, *by definition*:
    * a `&'a (mut) T` is what we called "an `'a`"!
* A reference `&'a T` is
  * *covariant* in `T` for the same reasons a ***read-only*** container is.
* A reference `&'a mut T` is
  * *invariant* in `T` for the same reasons a ***read-write*** container is.

Note that `T` can very well contain lifetime annotations. For instance:

* `&'a &'b (mut) U` is *covariant* in both `'a` and `'b`
* `&'a mut &'b (mut) U` is *covariant* in `'a`, but *invariant* in `'b`,
  * since `&'a mut T` is *invariant* in `T`.

## Solving the hard exercise

Here it is:

```rust
/* Make it work */
struct Interface<'a> {
    manager: &'a mut Manager<'a>
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface(&'a mut self) -> Interface {
        Interface {
            manager: &mut self.manager
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
```

We're asked to fix the lifetime annotations so that the code compiles without any errors.

First of all, why is this exercise considered *hard*?

I'm sure experienced Rust programmers can solve it right away, but do they do that in a systematic way or somewhat intuitively?

I'm wondering whether there's an infallible systematic way of adding lifetime annotations.

I don't have the luxury of going deep into this rabbit hole, but maybe some of you live on the other side of that rabbit hole and can shed some light on the matter.

Let's get started! First of all, let's ignore the two functions at the bottom as we don't need to touch those.

**Step 0:** Get rid of all the preexisting lifetime annotations.

```rust
struct Interface {
    manager: &mut Manager
}

impl Interface {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager {
    text: &str
}

struct List {
    manager: Manager,
}

impl List {
    pub fn get_interface(&mut self) -> Interface {
        Interface {
            manager: &mut self.manager
        }
    }
}
```

**Step 1:** Add lifetime annotations for every single reference appearing in definitions and signatures.

```rust
struct Interface<'a1> {
    manager: &'a1 mut Manager
}

impl Interface {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'b1> {
    text: &'b1 str
}

struct List {
    manager: Manager,
}

impl List {
    pub fn get_interface<'c1>(&'c1 mut self) -> Interface {
        Interface {
            manager: &mut self.manager
        }
    }
}
```

**Step 2:** Propagate the lifetime annotations to fill in the holes.

```rust
struct Interface<'a1, 'a2> {
    manager: &'a1 mut Manager<'a2>
}

impl<'a1, 'a2> Interface<'a1, 'a2> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'b1> {
    text: &'b1 str
}

struct List<'d1> {
    manager: Manager<'d1>,
}

impl<'d1> List<'d1> {
    pub fn get_interface<'c1, 'c2, 'c3>(&'c1 mut self) -> Interface<'c2, 'c3> {
        Interface {
            manager: &mut self.manager
        }
    }
}
```

By "saturating" the code with lifetime annotations, we disable any kind of *lifetime elision* so that we start completely clean with no lifetime constraints at all.

The idea is to then add *all and only* the lifetime constraints strictly required by the code, one by one. While subsequent constraints may make previous ones *redundant*, they should never make them needlessly restrictive. In other words, we just need to *add* and never *remove* constraints. We *can* remove constraints but only at the end as a *final simplification step*.

**Step 3:** Let's let Rust guide us.

Rusts says:

```text
21 | /         Interface {
22 | |             manager: &mut self.manager
23 | |         }
   | |_________^ associated function was supposed to return data with lifetime `'c2` but it is returning data with lifetime `'d1`
   |
   = help: consider adding the following bound: `'d1: 'c2`
```

Let's do what it says:

```rust
impl<'d1> List<'d1> {
    pub fn get_interface<'c1, 'c2, 'c3>(&'c1 mut self) -> Interface<'c2, 'c3>
        where 'd1 : 'c2
    {
        Interface {
            manager: &mut self.manager
        }
    }
}
```

This makes sense:

* By `List`'s definition, `List<'d1>` implies that `self.manager` is `'d1`:
  * as a consequence, so is `&mut self.manager`.
* By `Interface`'s definition, it's clear we're returning an `Interface<'d1, >`.
* We need to return an `Interface<'c2, >`, but we're returning an `Interface<'d1, >`.
  * We can see from `Interface`'s definition that it's *covariant* in its first argument, so
    * `'d1 : 'c2` implies `Interface<'d1, > : Interface<'c2, >`, which means
    * by adding `'d1 : 'c2` we can safely return an `Interface<'d1, >`.

Let's proceed:

```text
23 | /         Interface {
24 | |             manager: &mut self.manager
25 | |         }
   | |_________^ associated function was supposed to return data with lifetime `'c3` but it is returning data with lifetime `'d1`
   |
   = help: consider adding the following bound: `'d1: 'c3`
```

Let's follow the suggestion:

```rust
impl<'d1> List<'d1> {
    pub fn get_interface<'c1, 'c2, 'c3>(&'c1 mut self) -> Interface<'c2, 'c3>
        where 'd1 : 'c2 + 'c3
    {
        Interface {
            manager: &mut self.manager
        }
    }
}
```

Note that `'d1 : 'c2 + 'c3` is just short for `'d1 : 'c2, 'd1 : 'c3`.

This also makes sense... sort of:

* `Interface` is *invariant* in its second argument (since its field is), so
  * `'d1 : 'c3` won't help this time.
  * The problem is that neither `'d1 : 'c3` nor `'c3 : 'd1` implies `Interface< ,'d1> : Interface< ,'c3>`.
  * This means the only possibility is that `'c3 = 'd1`.

Is Rust leading us astray? (For the suspense-averse: No!)

Let's continue:

```text
23 | /         Interface {
24 | |             manager: &mut self.manager
25 | |         }
   | |_________^ associated function was supposed to return data with lifetime `'d1` but it is returning data with lifetime `'c3`
   |
   = help: consider adding the following bound: `'c3: 'd1`
```

This results in:

```rust
impl<'d1> List<'d1> {
    pub fn get_interface<'c1, 'c2, 'c3>(&'c1 mut self) -> Interface<'c2, 'c3>
        where 'd1 : 'c2 + 'c3, 'c3 : 'd1
    {
        Interface {
            manager: &mut self.manager
        }
    }
}
```

Not surprisingly, we get both `'d1 : 'c3` and `'c3 : 'd1`, which implies `'d1 = 'c3`. This follows from the *antisymmetric* property of `:`, property everyone is very familiar with:

* `(num1 <= num2 and num2 <= num1) implies num1 = num2

Let's keep going:

```text
23 | /         Interface {
24 | |             manager: &mut self.manager
25 | |         }
   | |_________^ associated function was supposed to return data with lifetime `'c2` but it is returning data with lifetime `'c1`
   |
   = help: consider adding the following bound: `'c1: 'c2`
```

We get:

```rust
impl<'d1> List<'d1> {
    pub fn get_interface<'c1, 'c2, 'c3>(&'c1 mut self) -> Interface<'c2, 'c3>
        where 'd1 : 'c2 + 'c3, 'c3 : 'd1, 'c1 : 'c2
    {
        Interface {
            manager: &mut self.manager
        }
    }
}
```

And we're done: exercise solved!

The constraint `'c1 : 'c2` is needed because, in a nutshell, the returned `Interface` will reference, through a `c2`, the same `Manager` referenced by `self`, a `'c1`. We're not really interested in the fact that `self` itself is a `'c1`, but in the fact that the referenced `Manager` is thus also `'c1`. After all, {`Interface`'s `manager`} references {the `Manager` referenced by `self`} directly: `self` is only used for the assignment.

(Yep, I used grouping {} in a natural language.)

**Step 4:** Let's simplify the lifetime annotations.

This is not strictly needed.

```rust
impl<'d1> List<'d1> {
    pub fn get_interface<'c1, 'c2, 'c3>(&'c1 mut self) -> Interface<'c2, 'c3>
        where 'd1 : 'c2 + 'c3, 'c3 : 'd1, 'c1 : 'c2
    {
        Interface {
            manager: &mut self.manager
        }
    }
}
```

Let's carry out the `'d1 = 'c3` simplification we talked about before:

```rust
impl<'d1> List<'d1> {
    pub fn get_interface<'c1, 'c2>(&'c1 mut self) -> Interface<'c2, 'd1>
        where 'd1 : 'c2, 'c1 : 'c2
    {
        Interface {
            manager: &mut self.manager
        }
    }
}
```

If we wanted, we could try adding the constraint `'c2 : 'c1` so that `'c1 = 'c2`, and see whether everything still works. By keeping `'c1`, we'd get

```rust
impl<'d1> List<'d1> {
    pub fn get_interface<'c1>(&'c1 mut self) -> Interface<'c1, 'd1> {
        Interface {
            manager: &mut self.manager
        }
    }
}
```

This would still work!

I think the first thing a human being would do, when solving this exercise, is add the constraint `'a2 : 'a1` to `Interface`, like this:

```rust
struct Interface<'a1, 'a2 : 'a1> {
    manager: &'a1 mut Manager<'a2>
}
```

Yet, we got away with not doing it!

## Why doesn't Rust?

Why doesn't Rust do this automatically for us?

I think the approach above finds the *least constraining set of lifetime annotations* that makes the code compile without any errors.

Please note that you can't skip the "saturation" step of the method or it won't work.

I can't be sure the method above works in the general case, but, from the (very) little I've seen, the problem looks neither *undecidable* nor *intractable*.

I still think that the programmer will still want to add some lifetime bounds as part of a *contract* between the *writer* and the *user* of a piece of code, but shouldn't Rust then complete the annotations?

Please let me know what you think, since I'm curious about this.

---

This post concludes my (hopefully first but not last) adventure with Rust.
I think Rust is a wonderful language and I like how one can talk about the more technical aspects of the language on [r/rust](https://www.reddit.com/r/rust). That's not always the case with other languages. I suspect that when a language becomes very popular, the forums get flooded with people that just want to get things done and don't have time to "waste" talking about more theoretical stuff. It'll happen to [r/rust](https://www.reddit.com/r/rust) as well! :)

Happy coding!
