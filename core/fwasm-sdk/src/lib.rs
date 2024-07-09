pub use fwasm_macros::*;
pub use fwasm_primitives::{AccountId, Balance, BlockNumber, Hash, Nonce, Signature};
#[allow(improper_ctypes)]
pub mod operation {

    extern "C" {
        fn encrypt(key: &[u8], data: &[u8]) -> anyhow::Result<()>;
    }

    pub fn decrypt(key: &[u8], data: &[u8]) -> anyhow::Result<()> {
        unsafe {
            //todo
        }
        Ok(())
    }
}
