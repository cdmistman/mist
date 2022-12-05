(lang dune 3.6)
(using menhir 2.1)

; See the complete stanza docs at https://dune.readthedocs.io/en/stable/dune-files.html#dune-project

(name mist-compiler)

(generate_opam_files true)

(source
 (github cdmistman/mist))

(authors "Colton Donnelly")

(maintainers "Colton Donnelly")

(package
 (name mist-compiler)
 (synopsis "Compiler for the Mist Programming Language.")
 (description
  "This is the original compiler implementation for Mist. The Mist CLI interfaces with it to compile Mist programs into Mist bytecode.")
 (depends ocaml dune)
 (tags ("programming languages")))
