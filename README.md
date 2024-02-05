What I've learned from the nice people at gdext discord chat:
- The book sample assumed that the `impl ExtensionLibrary` will have the `#[gdextension]` macro
- The entry point is for the ExtensionLibrary custom-class, not for Singleton custom-class
- Currently, there seems to be a bug in which the Godot text-editor intellisense (ctags?) will show MyEditorSingleton::new() in which, should not be used (it seems to just reset the singleton back to default (GodotDefault impl) values?)
- Currently, gdext singletons are not thread-safe (most likely, it isn't threadsafe on upstream as well), but the architects are working on making it thread-safe
  
