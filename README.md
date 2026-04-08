# ⚙️ TorcScript

> A human-readable, general-purpose interpreted programming language written in Rust.

---

## 🧠 Overview

**TorcScript** is a programming language designed with one core principle:

> **Code should read like thought.**

It prioritizes **clarity**, **simplicity**, and **expressiveness**, making it easy to understand even for those who are not deeply familiar with programming.

Built in **Rust**, TorcScript combines performance with safety, while maintaining a clean and approachable syntax.

---

## ✨ Features

- 🧾 **Highly readable syntax** — close to natural language
- ⚡ **Interpreted** — fast iteration, no compilation step
- 🧩 **General-purpose** — suitable for scripting, automation, and beyond
- 🧱 **Structured primitives** — built-in support for Lists and Objects
- 🔤 **Explicit constructs** — no hidden magic, everything is clear
- 🦀 **Powered by Rust** — safe and efficient under the hood

---

## 🚀 Example

### Code

```torc
state my_name = "Juan";
state favourite_fruits = List("apple", "banana", "grapes");
state juan = Object(name: my_name, age: 15, favourite_fruits: favourite_fruits);

action print_person(person) {
    print person.name + " is " + String(person.age) + " years old";
}

print_person(juan);

for (fruit) in juan.favourite_fruits {
    print "Favourite fruit: " + fruit;
}
````

### Output

```
> 'Juan is 15 years old'
> 'Favourite fruit: apple'
> 'Favourite fruit: banana'
> 'Favourite fruit: grapes'
```

---

## 🧩 Core Concepts

### 📌 Print keywords

Print a value using the `print` keyword

```torc
print "Hello, how are you?"
```

Use `print_named` to print an state (variable) or a fact (constant) with its name as a prefix

```torc
state is_happy = True;

print_named is_happy;

// > is_happy: True
```

---

### 📌 State (Variables)

Variables are declared using `state`, emphasizing explicit data definition.

```torc
state name = "Zeki";
```

---

### 📌 Facts (Constants)

Constants are declared using `fact`. They are immutable.

```torc
fact PI = 314;

PI = "another thing" ❌
```

---

### ⚙️ Conditionals 

Use the `if` keyword, followed with a condition and a block of code, to define an action that will be executed if the condition is true

```torc
state im_happy = True;

if im_happy {
    print "Yes, I'm happy";
}
```

Optionally, you can use `else` and `else if` to execute code if the main condition is not true

```torc
state im_happy = False;
state im_sad = False;

if im_happy {
    print "Yes, I'm happy";
} else if im_sad {
    print "I'm sad...";
} else {
    print "I'm not even happy or sad!";
}
```

---

### ⚙️ Actions (Functions)

Functions are defined using `action`, keeping behavior clearly separated.

```torc
action greet(person) {
    print "Hello " + person;
}
```

---

### 📦 Objects

Simple structured data with named properties.

```torc
state user = Object(name: "Juan", age: 15);
```

---

### 📚 Lists

Ordered collections of values.

```torc
state fruits = List("apple", "banana");
```

---

### 🔁 Loops

```torc
for (item, index) in fruits {
    print index + ": " + item;
}
```

---

## 🤝 Contributing

Contributions are welcome.

If you have ideas, improvements, or bug fixes:

* Open an issue
* Submit a pull request

---

## 📄 License

MIT License © Ezetorc
