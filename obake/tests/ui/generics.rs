#[obake::versioned]
struct Foo<T> {}

#[obake::versioned]
struct Bar<'a> {}

#[obake::versioned]
enum Baz<T> {}

#[obake::versioned]
enum Flim<'a> {}

fn main() {}
