use std::rc::Rc; // Rc is a reference-counted smart pointer
use std::sync::Arc; // Arc is an atomically reference-counted smart pointer (thread-safe)

fn main() {
    // Rust does referencing and dereferencing automatically, so you don't have to worry about
    // managing memory manually. This also means you don't need to use * to dereference a pointer.

    println!("--- Raw Pointers ---");

    let x = 42;
    // Create a raw pointer to x. this is similar to pointers in C/C++.
    // Raw pointers are not managed by Rust's ownership system.
    // They can be created by casting references.
    let raw_ptr: *const i32 = &x as *const i32;

    println!("Value of x: {}", x);
    // Printing the memory address held by the raw pointer.
    println!("Raw pointer address: {:?}", raw_ptr);

    // Dereferencing a raw pointer is unsafe because it can lead to undefined behavior if the
    // pointer is invalid. As such it must be done in an unsafe block.
    unsafe {
        println!("Dereferenced raw pointer value: {}", *raw_ptr);
        println!(
            "Memory address of dereferenced raw pointer: {:?}",
            raw_ptr as *const i32
        );
    }

    // Raw pointers can be mutable as well.
    // However, you cannot create a mutable raw pointer from an immutable reference.
    // You can only create a mutable raw pointer from a mutable reference.
    // This is similar to C/C++.
    let mut value = 500;
    // Create a mutable raw pointer to mut_raw_ptr.
    let mut_raw_ptr: *mut i32 = &mut value as *mut i32;

    println!("Value of value: {}", value);
    // Printing the memory address held by the mutable raw pointer.
    println!("Mutable raw pointer address: {:p}", mut_raw_ptr);

    unsafe {
        // Dereferencing and modifying the value via the pointer.
        *mut_raw_ptr = 600;
        println!("Dereferenced mutable raw pointer value: {}", *mut_raw_ptr);
    }
    println!("Value of value after modification: {}", value);

    println!("\n--- References ---");

    let y = 100;
    println!("Value of y: {}", y);
    let y_ref: &i32 = &y; // Create an immutable reference to y.
    println!("Value of y_ref: {}", y_ref);
    println!("Memory address of y_ref: {:?}", y_ref as *const i32);

    let mut y_mut = 200;
    println!("Value of y_mut: {}", y_mut);
    let y_immut_ref_from_mut_value: &i32 = &y_mut; // Create an immutable reference from a mutable
                                                   // value.
                                                   // This is safe because the reference is
                                                   // immutable. It does mean that y_mut cannot be
                                                   // modified through this reference, but y_mut
                                                   // can still be modified directly.
                                                   // y_mut cannot be modified until
                                                   // y_immut_ref_from_mut_value goes out of scope.
    println!(
        "Value of y_immut_ref_from_mut_value: {}",
        y_immut_ref_from_mut_value
    );
    println!(
        "Memory address of y_immut_ref_from_mut_value: {:?}",
        y_immut_ref_from_mut_value as *const i32
    );

    // y_mut += 1; // This line would cause a compile-time error if uncommented.

    // If we did this instead like this:
    let mut z_mut = 300;
    println!("Value of z_mut: {}", z_mut);
    {
        let z_ref: &i32 = &z_mut; // Create an immutable reference to z_mut.
        println!("Value of z_ref: {}", z_ref);
        println!("Memory address of z_ref: {:?}", z_ref as *const i32);
    } // z_ref goes out of scope here, so we can modify z_mut again.
      // This is a block scope, so z_ref is dropped here.
      // Now we can modify z_mut again.
    z_mut += 1;
    println!("Value of z_mut after modification: {}", z_mut);

    // Print the memory addresses and the values via references.
    println!("Reference to y: {:p}, value: {}", y_ref, y_ref);
    println!(
        "Reference to y_mut: {:p}, value: {}",
        y_immut_ref_from_mut_value, y_immut_ref_from_mut_value
    );

    println!("\n--- Smart Pointers ---");
    println!("\n--- Box Smart Pointer ---");

    // Box<T> is a smart pointer that allocates memory on the heap and returns a smart pointer to
    // it.
    let boxed = Box::new(7);

    // Dereferencing a Box is safe and can be done without an unsafe block.
    // It works just like a reference, but the data lives on the heap.
    println!("Box pointer address: {:p}", boxed);
    println!("Value of boxed: {}", boxed);

    // You can also create a mutable Box.
    let mut boxed_mut = Box::new(8);
    println!("Boxed mutable pointer address: {:p}", boxed_mut);
    println!("Value of boxed_mut: {}", boxed_mut);

    // You can dereference a mutable Box to modify the value it points to.
    *boxed_mut += 1; // This is safe because boxed_mut is mutable.
    println!("Value of boxed_mut after modification: {}", boxed_mut);

    // You can also create a mutable reference to a Box.
    // This is similar to creating a mutable reference to a regular variable.
    let boxed_mut_ref: &mut Box<i32> = &mut boxed_mut; // Create a mutable reference to boxed_mut.
    println!("Boxed mutable reference address: {:p}", boxed_mut_ref);
    println!("Value of boxed_mut_ref: {}", boxed_mut_ref);

    println!("\n--- Rc Smart Pointer ---");
    // Rc<T> is a reference-counted smart pointer. It allows multiple ownership of the same data in
    // a single threaded context.
    let rc_value = Rc::new(55);

    // Rc doesn't implement Deref to a pointer, so we use Rc::as_ptr() to get the memory address.
    println!("Rc pointer address: {:p}", Rc::as_ptr(&rc_value));
    println!("Value of rc_value: {}", rc_value);

    println!("\n--- Arc Smart Pointer ---");
    // Arc<T> is an atomically reference-counted smart pointer. It allows multiple ownership of the
    // same data in a multi-threaded context.
    // Arc is thread-safe and can be shared across threads.
    let arc_value = Arc::new(88);

    // Similarly, use Arc::as_ptr() to get the memory address.
    println!("Arc pointer address: {:p}", Arc::as_ptr(&arc_value));
    println!("Value of arc_value: {}", arc_value);
}
