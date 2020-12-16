# This project doesn't work yet!

By running cargo build, I got these errors:

```
error[E0428]: the name `size_type` is defined multiple times
     --> /home/vee/Develop/free/tkrzw-sys/target/debug/build/tkrzw-sys-6bbb010a406e06b9/out/bindings.rs:24258:1
      |
24257 | pub type size_type = size_type;
      | ------------------------------- previous definition of the type `size_type` here
24258 | pub type size_type = std_size_t;
      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `size_type` redefined here
```

# tkrzw-sys
tkrzw binding
