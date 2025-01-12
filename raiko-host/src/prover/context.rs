use std::path::PathBuf;

use anyhow::Result;

use super::{consts::RAIKO_GUEST_EXECUTABLE, request::ProofType};

#[derive(Debug, Default, Clone)]
pub struct Context {
    /// guest executable path
    pub guest_elf: PathBuf,
    /// cache for public input
    pub host_cache: PathBuf,

    pub max_caches: usize,

    pub l1_cache_file: Option<PathBuf>,

    pub l2_cache_file: Option<PathBuf>,
}

impl Context {
    pub fn new(
        guest_elf: PathBuf,
        host_cache: PathBuf,
        max_caches: usize,
        block_no: Option<u64>,
    ) -> Self {
        let mut ctx = Self {
            guest_elf,
            host_cache,
            max_caches,
            ..Default::default()
        };
        if let Some(block_no) = block_no {
            ctx.update_cache_path(block_no);
        }
        ctx
    }

    pub fn update_cache_path(&mut self, block_no: u64) {
        if self.l1_cache_file.is_none() {
            let file_name = format!("{}.l1.json.gz", block_no);
            self.l1_cache_file = Some(self.host_cache.join(file_name));
        }
        if self.l2_cache_file.is_some() {
            let file_name = format!("{}.l2.json.gz", block_no);
            self.l2_cache_file = Some(self.host_cache.join(file_name));
        }
    }

    pub fn guest_executable_path(&self, proof_instance: ProofType) -> PathBuf {
        match proof_instance {
            ProofType::Succinct => todo!(),
            ProofType::PseZk => todo!(),
            ProofType::Powdr => todo!(),
            ProofType::Sgx => self.guest_elf.join("sgx").join(RAIKO_GUEST_EXECUTABLE),
            ProofType::Risc0(_) => todo!(),
            ProofType::Native => todo!(),
        }
    }

    pub async fn remove_cache_file(&self) -> Result<()> {
        if let Some(file) = &self.l1_cache_file {
            tokio::fs::remove_file(file).await?;
        }
        if let Some(file) = &self.l2_cache_file {
            tokio::fs::remove_file(file).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_file_prefix() {
        let path = std::path::Path::new("/tmp/ethereum/1234.l1.json.gz");
        let prefix = path.file_prefix().unwrap();
        assert_eq!(prefix, "1234");
    }
}
