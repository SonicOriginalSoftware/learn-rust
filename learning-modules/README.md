I don't see any benefit of using the following method for organizing `modules`:

```
- src
  - main.rs
  - a_module.rs
  - a_module_library.rs
  - another_module.rs
  - an_internal_library
    - a_library.rs
  - another_module
    - another_module.rs
    - even_another_module.rs
```

The primary reason to embrace this organization is to reduce the number of `mod.rs` files sitting around leading to confusion if multiple of them are sitting around open in your code editor.

However, there's still a large chance you're duplicating a number of other file names, most notably by having a file with the same name as a folder. But you're also likely to duplicate the folder name as the name of the module's `Rust` files, as in the case of `another_module` above.

Which means you now have 2 files of the same name as well as a folder.

That all said, if removing `mod.rs` makes sense to you, go for it!
