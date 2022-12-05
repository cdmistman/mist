let it_works () = print_endline "it works!";;

let () = Callback.register "it_works" it_works;;
