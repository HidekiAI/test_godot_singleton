extends Node

func _ready() -> void:
	# Singleton seems to be broken or I am using this wrong...  Documentation says to do:
	# MyEditorSingleton.foo() without new()
	
	# Following will new 3 times, hence each call to X_foo() will be a new instance...
	MyEditorSingleton.new().get_foo()
	MyEditorSingleton.new().set_foo(42)	# this does NOT seem right...
	MyEditorSingleton.new().get_foo()

func _process(delta):
	var v = MyEditorSingleton.new().get_foo();
	print("static value = " + str(v) );
