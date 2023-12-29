use super::*;

#[test]
fn import_succeeds() {
  Test::new()
    .tree(tree! {
      "import.justfile": "
        b:
          @echo B
      ",
    })
    .justfile(
      "
        import './import.justfile'

        a: b
          @echo A
      ",
    )
    .test_round_trip(false)
    .arg("a")
    .stdout("B\nA\n")
    .run();
}

#[test]
fn trailing_spaces_after_import_are_ignored() {
  Test::new()
    .tree(tree! {
      "import.justfile": "",
    })
    .justfile(
      "
      import './import.justfile'\x20
      a:
        @echo A
    ",
    )
    .test_round_trip(false)
    .stdout("A\n")
    .run();
}

#[test]
fn import_after_recipe() {
  Test::new()
    .tree(tree! {
      "import.justfile": "
        a:
          @echo A
      ",
    })
    .justfile(
      "
      b: a
      import './import.justfile'
      ",
    )
    .test_round_trip(false)
    .stdout("A\n")
    .run();
}

#[test]
fn circular_import() {
  Test::new()
    .justfile("import 'a'")
    .tree(tree! {
      a: "import 'b'",
      b: "import 'a'",
    })
    .status(EXIT_FAILURE)
    .stderr_regex(path_for_regex(
      "error: Import `.*/a` in `.*/b` is circular\n",
    ))
    .run();
}

#[test]
fn import_recipes_are_not_default() {
  Test::new()
    .tree(tree! {
      "import.justfile": "bar:",
    })
    .justfile("import './import.justfile'")
    .test_round_trip(false)
    .status(EXIT_FAILURE)
    .stderr("error: Justfile contains no default recipe.\n")
    .run();
}

#[test]
fn listed_recipes_in_imports_are_in_load_order() {
  Test::new()
    .justfile(
      "
      import './import.justfile'
      foo:
    ",
    )
    .write("import.justfile", "bar:")
    .args(["--list", "--unsorted"])
    .test_round_trip(false)
    .stdout(
      "
      Available recipes:
          foo
          bar
    ",
    )
    .run();
}

#[test]
fn include_error() {
  Test::new()
    .justfile("!include foo")
    .status(EXIT_FAILURE)
    .stderr(
      "
      error: The `!include` directive has been stabilized as `import`
       --> justfile:1:1
        |
      1 | !include foo
        | ^
      ",
    )
    .run();
}

#[test]
fn recipes_in_import_are_overridden_by_recipes_in_parent() {
  Test::new()
    .tree(tree! {
      "import.justfile": "
        a:
          @echo IMPORT
      ",
    })
    .justfile(
      "
        import './import.justfile'

        set allow-duplicate-recipes

        a:
          @echo ROOT
      ",
    )
    .test_round_trip(false)
    .arg("a")
    .stdout("ROOT\n")
    .run();
}

#[cfg(not(windows))]
#[test]
fn import_paths_beginning_with_tilde_are_expanded_to_homdir() {
  Test::new()
    .write("foobar/mod.just", "foo:\n @echo FOOBAR")
    .justfile(
      "
        import '~/mod.just'
      ",
    )
    .test_round_trip(false)
    .arg("foo")
    .stdout("FOOBAR\n")
    .env("HOME", "foobar")
    .run();
}
