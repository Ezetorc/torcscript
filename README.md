````markdown
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

### 📌 State (Variables)

Variables are declared using `state`, emphasizing explicit data definition.

```torc
state name = "Zeki";
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
for (item) in fruits {
    print item;
}
```

---

## 🎯 Philosophy

TorcScript is built around:

* **Readability over cleverness**
* **Explicitness over implicit behavior**
* **Simplicity over complexity**

It aims to reduce cognitive load and make code feel intuitive.

---

## 🛠️ Installation

> *(Add instructions here once available)*

```bash
git clone https://github.com/your-username/torcscript
cd torcscript
cargo build
```

---

## 📌 Roadmap

* [ ] Parser improvements
* [ ] Error handling system
* [ ] Standard library expansion
* [ ] REPL support
* [ ] File execution CLI
* [ ] Package/module system

---

## 🤝 Contributing

Contributions are welcome.

If you have ideas, improvements, or bug fixes:

* Open an issue
* Submit a pull request

---

## 📄 License

MIT License © Your Name

---

## 💡 Inspiration

TorcScript is inspired by the idea that:

> Programming languages should adapt to humans, not the other way around.

```

If you want, I can also generate:
- a **logo concept 🎨**
- a **GitHub repo structure 📁**
- or a **landing page for TorcScript 🌐**
```
