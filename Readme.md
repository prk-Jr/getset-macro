# GetSet Procedural Macro

The `GetSet` procedural macro simplifies the creation of getter and setter methods for fields in your Rust structs. With this macro, you can generate these methods automatically, reducing boilerplate code and enhancing code readability.

## Table of Contents
- [Usage](#usage)
- [Example](#example)
- [How It Works](#how-it-works)
- [Contributing](#contributing)
- [License](#license)

## Usage

To use the `GetSet` procedural macro, follow these steps:

0. Add the `GetSet` crate to your `Cargo.toml`:

   ```toml
   [dependencies]
   getset-macro = "0.1"
   ```

1. Import the GetSet procedural macro into your Rust code:
    ```rust
    use getset_macro::GetSet;
    ```

2. Apply the #[derive(GetSet)] attribute to your struct. This will automatically    generate constructor, getter and setter methods for all the struct's fields.
    ```rust
    #[derive(GetSet)]
        struct MyStruct {
            field1: FieldType1,
            field2: FieldType2,
            // ... more fields
        }
    ```

3. Use the generated constrotor, getter and setter methods as follows:

    ```rust
        let mut instance = MyStruct {
                field1: initial_value1,
                field2: initial_value2,
                };

        // Or Using derived constructor
        let mut new_instance = MyStruct::new(field1, field2);

        // Get the value of field1
        let value1 = instance.get_field1();

        // Set the value of field2
        instance.set_field2(new_value2);

    
    ```

## Example
Here's an example of how to use the GetSet procedural macro:
```rust
use getset_macro::GetSet;

#[derive(GetSet)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    // Using constructor API
    let mut person = Person::new("Alice".to_string(), 30);

    // Get the name and age
    let name = person.get_name();
    let age = person.get_age();

    // Modify the age
    person.set_age(25);
}

```
In this example, the GetSet macro generates new(), get_name(), get_age(), set_name(), and set_age() methods for the Person struct's fields.

How It Works
The GetSet macro takes care of the following tasks:

It generates getter methods (get_fieldname()) for all fields in the struct.
It generates setter methods (set_fieldname(value)) for all fields in the struct.
The generated getter methods return a reference to the field's value.
The generated setter methods allow you to set the field's value.
The macro analyzes the struct's fields and automatically generates code for these methods, reducing manual code writing.