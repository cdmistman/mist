module Main

use std.io
use std.io.Write
use std.system
{- is the same as -}
in std use
	io
	io.Write
	system

effect Console
	stdout: Write

main () =
	let stdout = system.stdout in

	let result = handle
		hello_world ()
	with
		Console -> Console { stdout }
	in

	match result
		Ok() -> (),
		Err(err) -> panic ("uh oh"),

main' () =
	let stdout = system.stdout in

	let result = handle hello_world ()
		with Console -> Console { stdout }
	in
	
	match result
		Ok () -> ()
		Err err -> panic "uh oh"

hello_world: () -> ^Console io.Result
hello_world =
	my_print "hello, world!"

my_print: AsRef str -> ^Console io.Result
my_print s =
	println s
