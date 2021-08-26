#[obake::versioned]
#[obake(version("0.1.0"))]
#[derive(PartialEq, Eq, Debug)]
struct Foo {}

impl Foo {
    fn foo(&self) -> u32 {
        42
    }
}

#[test]
fn foo_alias_created() {
    let x: Foo = Foo {};
    assert_eq!(x, Foo {});
}

#[test]
fn foo_method_visible() {
    let x: Foo = Foo {};
    assert_eq!(x.foo(), 42);
}
