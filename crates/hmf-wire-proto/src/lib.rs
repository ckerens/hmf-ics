pub mod proto {
    pub mod v1 {
        include!(concat!(env!("OUT_DIR"), "/hmf.v1.rs"));
    }
}

pub mod convert;
pub mod error;
pub mod wire;
