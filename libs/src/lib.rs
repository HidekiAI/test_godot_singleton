use godot::{engine::Engine, prelude::*};
//use lazy_static::lazy_static;
//use std::cell::Ref;
//use std::sync::Mutex;

// NOTE: The code is hybrid logic from  https://godot-rust.github.io/book/recipes/engine-singleton.html
// sample code godot-rust book, and have been modified to make it usable.

#[derive(GodotClass)]
//#[class(tool, init, base=Object)] // uncomment this (with default init) and remove below (impl init)
#[class(tool, base=Object)]
pub struct MyEditorSingleton {
    //base: Mutex<Base<Object>>,
    base: Base<Object>,

    #[export]
    foo_value: i64,
}

// Implementing init() to verify whether the entry point is called, remove/comment this when working
#[godot_api]
impl IObject for MyEditorSingleton {
    // For singleton, is this init() really safe?  Does it need MUTEX?
    fn init(base: Base<Object>) -> Self {
        godot_print!("MyEditorSingleton::init() - breadcrumb");
        MyEditorSingleton { base, foo_value: 0 }
    }
}

#[godot_api]
impl MyEditorSingleton {
    //#[func]
    //fn new() -> Self {
    //    godot_print!("MyEditorSingleton::new() - breadcrumb");
    //    MyEditorSingleton {
    //        base: Mutex::new(Base::new()),
    //        base: Base::new(),
    //        foo_value: 0,
    //    }
    //}

    #[func]
    fn foo(&mut self) {
        godot_print!("MyEditorSingleton::foo() - foo={:?}", self.foo_value);
    }

    #[func]
    fn set_foo(&mut self, v: i64) {
        godot_print!("MyEditorSingleton::set_foo({:?})", v);
        self.foo_value = v;
    }
    #[func]
    fn get_foo(&self) -> i64 {
        godot_print!("MyEditorSingleton::get_foo() - foo={:?}", self.foo_value);
        self.foo_value
    }
}
//lazy_static! {
//    static ref MY_SINGLETON: MyEditorSingleton = MyEditorSingleton::new();
//}

///////////////////////////////////////////////////////////////////// MyExtension
pub mod entry_point {
    use godot::{engine::Engine, prelude::*};
    use godot::obj::cap::GodotDefault;

    #[derive(GodotClass)]
    #[class(base=Object)]
    pub struct MyExtension {}

    impl godot::obj::cap::GodotDefault for MyExtension {}

    // define an entry point for which the singleton is registered
    #[gdextension]
    unsafe impl ExtensionLibrary for MyExtension {
        fn on_level_init(level: InitLevel) {
            let singleton_struct = StringName::from("MyEditorSingleton");
            if level == InitLevel::Scene {
                // The StringName identifies your singleton and can be
                // used later to access it.
                Engine::singleton()
                    .register_singleton(singleton_struct, super::MyEditorSingleton::new_alloc().upcast());
            }
        }

        fn on_level_deinit(level: InitLevel) {
            let singleton_struct = StringName::from("MyEditorSingleton");
            if level == InitLevel::Scene {
                // Unregistering is needed to avoid memory leaks and
                // warnings, especially for hot reloading.
                Engine::singleton().unregister_singleton(singleton_struct);
            }
        }
    }

    //////////////////////////////////////////////////////////////////////// tests
    #[cfg(test)]
    mod tests {
        // Import the function under test
        use super::*;

        #[test]
        fn test_get_ref() {
            // NOTE: This all asserts will ALWAYS fail/assert because Godot Engine is not going to be running, it's only here
            // as a means of completness to the usage of this extension-library
            let singleton_struct = StringName::from("MyEditorSingleton");
            let possible_singleton =
                godot::engine::Engine::singleton().get_singleton(singleton_struct.clone());
            assert!(
                possible_singleton.is_some(),
                "Singleton '{}' not found",
                singleton_struct.to_string()
            );
            match possible_singleton {
                Some(mut singleton) => {
                    let funcname = "foo";
                    let foo_arg1 = 42;
                    let found_func = singleton.has_method(StringName::from(funcname));
                    assert!(found_func, "Method '{}' not found", funcname);
                    let call_result =
                        singleton.call(StringName::from(funcname), &[Variant::from(foo_arg1)]);
                    assert_eq!(
                        call_result.is_nil(),
                        false,
                        "Call to '{}'({}) failed",
                        funcname,
                        foo_arg1
                    );
                }
                None => panic!("Singleton '{}' not found", singleton_struct.to_string()),
            }
        }
    }
}
