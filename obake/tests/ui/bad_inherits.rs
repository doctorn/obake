#[obake::versioned]
#[obake(version("0.1.0"))]
struct Foo {}

#[obake::versioned]
#[obake(version("0.1.0"))]
struct Bar {
    #[obake(inherit)]
    field_0: [Foo; 3],
}

#[obake::versioned]
#[obake(version("0.1.0"))]
enum Baz {
    Variant {
        #[obake(inherit)]
        field_0: [Foo; 3]
    }
}

fn main() {}
