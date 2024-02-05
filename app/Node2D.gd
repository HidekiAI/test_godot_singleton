extends Node

func _ready() -> void:
	MyEditorSingleton.foo(); # without new()
	var v = MyEditorSingleton.foo_value;
	print("foo_value = " + str(v))
	
	# Following will new 3 times, hence each call to X_foo() will be a new instance...
	MyEditorSingleton.get_foo();
	MyEditorSingleton.set_foo(42);	# this does NOT seem right...
	MyEditorSingleton.get_foo();
