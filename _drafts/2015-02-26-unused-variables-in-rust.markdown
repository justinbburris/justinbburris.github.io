---
layout: post
title:  "Unused variables in Rust"
date:   2015-02-26 21:53:49
categories: rust thoughts
rust_ver: "rustc 1.0.0-nightly (522d09dfe 2015-02-19) (built 2015-02-20)"
---

Ever had code that looks like this?

<figure>
  <figcaption><a href='/examples/2015-02-26/unused.rs'>unused.rs</a></figcaption>
  {% highlight rust linenos %}
  fn main() {
    let not_so_great_value = 1u32;
    let great_value = 1u32;

    println!("This is a great value: {:?}", great_value);
  }
  {% endhighlight %}
</figure>

Here we've initialized two variables.

`great_value` & `not_so_great_value`

Yet when we try to complie this code in rust, the compiler provides us with this helpful warning:

{% highlight rust %}
// $ rustc unused.rs
unused.rs:2:9: 2:27 warning: unused variable: `not_so_great_value`, #[warn(unused_variables)] on by default
unused.rs:2     let not_so_great_value = 1u32;
                    ^~~~~~~~~~~~~~~~~~
{% endhighlight %}

The rust compiler comes with built in checking of dead code. When the compiler sees that there's a
variable initialized, which is never used, it warns us of this. Your code will still compile and run
but I recommend you clean out those bits of dead code.

## Why care about dead code?

Since this [dead code](http://www.aivosto.com/vbtips/deadcode.html) is never executed, why worry about it? When working with larger software projects,
this extra baggage can cause you extra headache when trying to understand what a function or a class does.

Additionally when you have code which does nothing in your system, it becomes hard to recognize what is working
code and what needs to be removed. This means when you want to add in a new feature or fix a bug, it can
be difficult to figure out what exactly your program is doing.

## What if that line is really needed?

Sometimes we just want those extra lines. At times when you want to debug an issue, keep some code around in
a refactor or trying to wrap your head around something new, it can be really frustrating to have this error
appear when you're trying to compile your program.

Fortunatelly rust has some ways of disabling this feature, so you can experiment without feeling bad about
dangaling references.

### Prefix your variable with an underscore

<figure>
  <figcaption><a href='/examples/2015-02-26/underscore.rs'>underscore.rs</a></figcaption>
  {% highlight rust linenos %}
  fn main() {
    // notice the `_` in _not_so_great_value
    let _not_so_great_value = 1u32;
    let great_value = 1u32;

    println!("This is a great value: {:?}", great_value);
  }
  {% endhighlight %}
</figure>

In addition to being a pretty quick and dirty way to silence warnings, it's apparently useful when a function
requires an argument, which isn't used. [catamorphism](https://github.com/rust-lang/rust/issues/832) provides a good example
of when you would need to do this.


### Use an attribute to silence your warnings

<figure>
  <figcaption><a href='/examples/2015-02-26/attribute.rs'>attribute.rs</a></figcaption>
  {% highlight rust linenos %}
  #[allow(dead_code)]
  fn unused_function() {}

  fn main() {
    let great_value = 1u32;

    println!("This is a great value: {:?}", great_value);
  }
  {% endhighlight %}
</figure>

Rust attributes can only be used to silence warnings for unused functions. Since your variables aren't units
rust will error if you try to silence a variable with an attribute.
