mod structs {
    #[obake::versioned]
    #[obake(version("0.1.0"))]
    #[obake(inherit)]
    struct Foo {}

    #[obake::versioned]
    #[obake(version("0.1.0"))]
    #[obake(cfg("0.1.0"))]
    struct Bar {}
}

mod enums {
    #[obake::versioned]
    #[obake(version("0.1.0"))]
    #[obake(inherit)]
    enum Foo {}

    #[obake::versioned]
    #[obake(version("0.1.0"))]
    #[obake(cfg("0.1.0"))]
    enum Bar {}

    #[obake::versioned]
    #[obake(version("0.1.0"))]
    enum Baz {
        #[obake(inherit)]
        Variant,
    }
}

fn main() {}
