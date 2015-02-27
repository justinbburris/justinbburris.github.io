---
layout: post
title:  "Dead Code and Unused Variables in Rust"
date:   2015-02-26 21:53:49
categories: rust thoughts
rust_ver: "rustc 1.0.0-nightly (522d09dfe 2015-02-19) (built 2015-02-20)"
langunage: rust
update:
  - attribute
---

Ever had code that looks like this?

<figure>
  <figcaption><a href='/examples/2015-02-26/unused.rs'>unused.rs</a></figcaption>
  {% highlight rust linenos %}
  fn main() {
    let unused_variable = 1u32;
    let great_variable = 1u32;

    println!("This is a great value: {:?}", great_variable);
  }
  {% endhighlight %}
</figure>

Here we've initialized two variables.

`great_variable` & `unused_variable`

Yet when we try to complie this code in rust, the compiler provides us with this helpful warning:

{% highlight rust %}
// $ rustc unused.rs
unused.rs:2:9: 2:27 warning: unused variable: `unused_variable`, #[warn(unused_variables)] on by default
unused.rs:2     let unused_variable = 1u32;
                    ^~~~~~~~~~~~~~~~~~
{% endhighlight %}

The rust compiler comes with built in checking of unused vairables and dead code. When the compiler sees that there's a
variable initialized, yet is never used, it warns us of this. Your code will still compile and run
but I recommend you clean out those bits of dead code.

## Why care about dead code?

Since [dead code](http://www.aivosto.com/vbtips/deadcode.html) is never executed, why worry about it? When working on larger software projects,
extra baggage can cause extra headache when trying to understand what a function or a class does.

> Avoid this at all costs

Additionally when you have code which does nothing in your system, it becomes hard to recognize what is working
code and what needs to be removed. This means when you want to add in new features or fix bugs, it can
be difficult to figure out what exactly your program is doing.

## What if that line is really needed?

Sometimes we just need that extra code, whether it's to debug an issue, keep some code around during
a refactor or when trying to wrap your head around something new. It can be really frustrating to have this error
appear when you're trying to compile your program. Additionally there are legitimate times when you might need
to keep a variable in your function for other purposes.

Fortunatelly rust has some ways of disabling this feature, so you can experiment without feeling bad about
dangaling references.

> I recommend not overring the compiler if you can avoid it

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

In addition to being a pretty quick and dirty way to silence warnings, it's useful when a function requires an argument,
which isn't used. [catamorphism](https://github.com/rust-lang/rust/issues/832) provides a good example
of when you would need to do this.

### Use an attribute to silence your warnings

<figure>
  <figcaption><a href='/examples/2015-02-26/attribute.rs'>attribute.rs</a></figcaption>
  {% highlight rust linenos %}
  #[allow(dead_code)] // Won't complain about the function never being called
  fn unused_function() { }

  #[allow(unused_variables)] // Won't complain about vairables never being used
  fn main() {
    let dont_mind_me = 1;
    let great_value = 1u32;

    println!("This is a great value: {:?}", great_value);
  }
  {% endhighlight %}
</figure>

By setting an attribute, you can can tell rust to allow the function to have nasty things like `dead_code` or `unused_variables`.

You can use mutliple attributes per function, allowing your dead code to flourish.

### Silence warnings via compiler options

The rust compiler has a lot of options but the one I want to focus on here is `-A`

{% highlight bash %}
$ rustc -h
    -A --allow OPT      Set lint allowed
{% endhighlight %}

With this we can let the linter know what it can ignore when compiling our beautiful rust program.

#### Dead Code

Silence warnings on any unused functions you may have by passing the `dead_code` option

{% highlight bash %}
$ rustc -A dead_code my_rust_program.rs
{% endhighlight %}

#### Unused Variables

Silence warnings on any unused variables with the `unused_variables` option

{% highlight bash %}
$ rustc -A unused_variables my_rust_program.rs
{% endhighlight %}

## That's it!

As far as I can tell, if you're looking to silence your unused variables and dead code, those are your options.

Get out there and write some great rust, and keep these things in mind if you need a quick way
to make those pesky warnings disappear.

