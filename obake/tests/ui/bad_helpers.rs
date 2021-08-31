#![allow(unused)]

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

mod derives {
    #[obake::versioned]
    #[obake(version("0.1.0"))]
    struct Foo {
        #[obake(derive(Clone))]
        field_0: u32,
    }

    #[obake::versioned]
    #[obake(version("0.1.0"))]
    enum Bar {
        #[obake(derive(Clone))]
        X,
    }

    #[obake::versioned]
    #[obake(version("0.1.0"))]
    enum Baz {
        X {
            #[obake(derive(Clone))]
            field_0: u32,
        },
    }
}

mod serdes {
    #[obake::versioned]
    #[obake(version("0.1.0"))]
    struct Foo {
        #[obake(serde(skip_serializing))]
        field_0: u32,
    }

    #[obake::versioned]
    #[obake(version("0.1.0"))]
    enum Bar {
        #[obake(serde(skip_serializing))]
        X,
    }

    #[obake::versioned]
    #[obake(version("0.1.0"))]
    enum Baz {
        X {
            #[obake(serde(skip_serializing))]
            field_0: u32,
        },
    }
}

fn main() {}
