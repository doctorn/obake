pub use ver_derive::{ver, versioned};

#[versioned]
#[version("0.1.0")]
#[version("0.2.0", migrate_to_0_2_0)]
#[version("0.3.0", migrate_to_0_3_0)]
#[version("1.9.0", migrate_to_1_9_0)]
struct Foo {
    field_0: String,
    #[requires(">=0.2")]
    field_1: i32,
    #[requires("0.2")]
    field_2: u64,
    #[requires(">0.2, <1.8.0")]
    field_3: String,
}

fn migrate_to_0_2_0(foo: ver![Foo@"0.1.0"]) -> ver![Foo@"0.2.0"] {
    ver!(Foo@"0.2.0" {
        field_0: foo.field_0,
        field_1: 0,
        field_2: 0,
    })
}

fn migrate_to_0_3_0(foo: ver![Foo@"0.2.0"]) -> ver![Foo@"0.3.0"] {
    ver!(Foo@"0.3.0" {
        field_0: foo.field_0,
        field_1: foo.field_1,
        field_3: "default here".to_owned(),
    })
}

fn migrate_to_1_9_0(foo: ver![Foo@"0.3.0"]) -> ver![Foo@"1.9.0"] {
    ver!(Foo@"1.9.0" {
        field_0: foo.field_0,
        field_1: foo.field_1,
    })
}

impl Foo {
    fn hello(&self) {
        println!("Hello, World!");
    }
}
