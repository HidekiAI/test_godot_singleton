What I've learned (mostly from the nice people at gdext discord chat, but some discovered thanks to rust-analyzer):
- The book sample assumed that the `impl ExtensionLibrary` will have the `#[gdextension]` macro
- The entry point is for the ExtensionLibrary custom-class, not for Singleton custom-class
- Currently, there seems to be a bug in which the Godot text-editor intellisense (ctags?) will show MyEditorSingleton::new() in which, should not be used (it seems to just reset the singleton back to default (GodotDefault impl) values?)
- I had to add GodotDefault on my registering/deregistering side struct impl (maybe because I've derived it off Object?), in which I am (as always) thankful for rust-analyzer to catch it (how many other system-level languages do you know, that will catch these kind of head-scratching "I didn't know that" bugs during compiling/static-analysis level rather than during runtime?!?!?  All hail rust!)
- Currently, gdext singletons are not thread-safe (most likely, it isn't threadsafe on upstream as well), but the architects are working on making it thread-safe
