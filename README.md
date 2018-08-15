# cargo2junit

Converts Rust's "cargo test" output into JUnit XML.

## License
This project is licensed under "THE BEER-WARE LICENSE" (Revision 42).

<rchabowski@gmail.com> wrote this project. As long as you retain this notice you
can do whatever you want with this stuff. If we meet some day, and you think
this stuff is worth it, you can buy me a beer in return.

Yours,
mgr inż. Rafał


## Example input

```
   Compiling quote v0.3.15
   Compiling num-traits v0.1.41
   Compiling dtoa v0.4.2
   Compiling unicode-xid v0.0.4
   Compiling itoa v0.3.4
   Compiling serde v1.0.27
   Compiling synom v0.11.3
   Compiling syn v0.11.11
   Compiling serde_derive_internals v0.19.0
   Compiling serde_derive v1.0.27
   Compiling serde_json v1.0.9
   Compiling pathfinder v0.1.0 (file:///D:/Git/pathfinder)
    Finished dev [unoptimized + debuginfo] target(s) in 24.41 secs
     Running target\debug\deps\pathfinder-30a7743be09a8279.exe

running 14 tests
test tests::falling_over_right_edge ... ok
test tests::falling_over_left_edge ... ok
test tests::falling_over_top_edge ... ok
test tests::falling_over_bottom_edge ... ok
test tests::happy_path ... ok
test tests::map_validation_destination_out_of_bounds_negative ... ok
test tests::map_validation_destination_out_of_bounds_positive ... ok
test tests::map_validation_destination_too_big_height ... ok
test tests::map_validation_destination_too_big_height_width ... ok
test tests::map_validation_destination_too_big_width ... ok
test tests::map_validation_size_mismatch ... ok
test tests::map_validation_start_eq_destination ... ok
test tests::map_validation_start_out_of_bounds_negative ... ok
test tests::map_validation_start_out_of_bounds_positive ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests pathfinder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Example output

```xml
<?xml version="1.0" ?>
<testsuites>
    <testsuite errors="0" failures="1" id="0" name="my test suite" tests="2">
        <testcase classname="some.class.name" name="Test1" time="123.345000"/>
        <testcase classname="some.class.name" name="Test2" time="678.123000"/>
            <failure message="test failure">Assertion failed</failure>
    </testsuite>
</testsuites>
```