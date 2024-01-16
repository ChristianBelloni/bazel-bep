pub mod server;

pub mod build_event_stream {
    include!(concat!(env!("OUT_DIR"), "/build_event_stream.rs"));
}

pub mod blaze {
    include!(concat!(env!("OUT_DIR"), "/blaze.rs"));
    pub mod invocation_policy {
        include!(concat!(env!("OUT_DIR"), "/blaze.invocation_policy.rs"));
    }
}

pub mod command_line {
    include!(concat!(env!("OUT_DIR"), "/command_line.rs"));
}

pub mod failure_details {
    include!(concat!(env!("OUT_DIR"), "/failure_details.rs"));
}

pub mod options {
    include!(concat!(env!("OUT_DIR"), "/options.rs"));
}

pub mod package_metrics {
    pub use crate::devtools::*;
}

pub mod google {
    pub mod devtools {
        pub mod build {
            pub mod v1 {
                include!(concat!(env!("OUT_DIR"), "/google.devtools.build.v1.rs"));
            }
        }
    }
    pub mod api {
        include!(concat!(env!("OUT_DIR"), "/google.api.rs"));
    }
}

mod devtools {
    pub use build::*;
    pub mod build {
        pub use lib::*;
        pub mod lib {
            pub use packages::*;
            pub mod packages {
                pub use metrics::*;
                pub mod metrics {
                    include!(concat!(
                        env!("OUT_DIR"),
                        "/devtools.build.lib.packages.metrics.rs"
                    ));
                }
            }
        }
    }
}