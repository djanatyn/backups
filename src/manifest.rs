use crate::bucket::Bucket;
use crate::repository::Repository;
use crate::systemd::SystemdCreds;

pub struct Manifest {
    pub repo: Repository,
    pub bucket: Bucket,
    pub creds: SystemdCreds,
}

impl Manifest {
    pub fn save(&self) {
        todo!();
    }
}
