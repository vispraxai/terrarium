use serde::{Deserialize, Serialize};

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord,
        )]
        pub struct $name(pub u64);
    };
}

id_type!(PersonId);
id_type!(LocationId);
id_type!(ObjectId);
id_type!(EventId);
id_type!(SnapshotId);
