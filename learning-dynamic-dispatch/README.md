# Dynamic Dispatch

The purpose of this example is to showcase how to use `Trait`s and `Generics` for allowing _dynamic dispatch_.

_Dynamic dispatch_ is the runtime behavior of using the `vtable` to perform a pointer lookup in order to find the appropriate instance method for the target object.

_Static dispatch_ is addressed [here](../learning-static-dispatch).

# Explanation

We want to be able to create an object known only at runtime that can be passed around with a known ability; it `CanDoThings`. So we create a `Trait` that declares a behavior `do_it` (the _thing_, obviously).

This trait must be implemented by the object that will _do the thing_, so we implement the `Trait` (`CanDoThings`) for a `Thing` `struct`.

At this point we could obviously just pass around an instance of `Thing`, but what if we want to hold on to something that `CanDoThings` as part of a `Container` (perhaps as a matter of application state, application configuration, etc.), without the container caring about the specific implementation of the `CanDoThings` `Trait`?

`Trait`s are not `Rust` `Type`s. Thus we can't apply `CanDoThings` as the `Type` for a member of the `Container` `struct`.

Instead, what we must do is use a `Generic` parameter for the `Container` `struct` and set the `CanDoThings` `Trait` as a specifier for that `Generic` parameter (`C`). Then we can define a member `a_thing` that has `Type` `C` that the compiler will know `CanDoThings`.

For consuming this `Container`, we initialize it like any `struct` and for the `a_thing` member we can statically (at-compile-time) assign the `Thing` `struct` to that member. This is valid because `CanDoThings` is implemented for `Thing`.

Now we can call on our `container` instance and its `a_thing` member to `do_it()` (i.e. to _do the thing_).

# Extra Resources

https://blog.rust-lang.org/2015/05/11/traits.html

https://joshleeb.com/posts/rust-traits-and-trait-objects/
