use crate::{
    context::Context,
    wasm_code::{FunctionDescriptor, WasmCodeRef, WasmDescriptor, WasmInfo},
};
use fwasm_sdk::AccountId;
use wasmtime::{Engine, ExternRef, Instance, Module, Rooted, Store, Val, WasmResults};

pub struct Vm {
    space: Store<Context>,
    instance: Instance,
    descriptor: WasmDescriptor,
}

impl Vm {
    pub fn new_instance(wasm: &WasmInfo, context: Context) -> anyhow::Result<Self> {
        let engine = Engine::default();
        let module = match wasm.code {
            WasmCodeRef::Blob(ref blob) => Module::from_binary(&engine, blob)?,
            WasmCodeRef::File(ref path) => Module::from_file(&engine, path)?,
        };
        let mut store = Store::new(&engine, context);
        let injects = Context::inject_host_funcs(&mut store);
        let instance = Instance::new(&mut store, &module, &injects)?;
        let descriptor = wasm.descriptor.clone();
        Ok(Self {
            space: store,
            instance,
            descriptor,
        })
    }

    pub fn call_encrypt(&mut self, func: &str, args: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        // TODO handle endpoint not found
        let encrypt_fn = self
            .instance
            .get_func(&mut self.space, func)
            .ok_or(anyhow::anyhow!("endpoint not found"))?;
        let mut results = vec![Val::I32(0)];
        encrypt_fn.call(&mut self.space, &[Val::I32(1), Val::I32(2)], &mut results)?;
        Ok(vec![0u8])
    }

    // pub fn call_init(&mut self, func: &str, args: Vec<u8>)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn load_wasm_should_work() {
        env_logger::init();
        let wasm_path = "../../target/wasm32-unknown-unknown/debug/fwasm_examples.wasm";
        let wasm = WasmInfo {
            account: AccountId::new([0u8; 32]),
            name: "avs-dev-demo".to_string(),
            version: 0,
            code: WasmCodeRef::File(wasm_path.to_string()),
            descriptor: WasmDescriptor {
                init: FunctionDescriptor {
                    name: "init".to_string(),
                    signature: "i32".to_string(),
                },
                decrypt: vec![FunctionDescriptor {
                    name: "decrpyt".to_string(),
                    signature: "i32".to_string(),
                }],
                encrypt: vec![FunctionDescriptor {
                    name: "encrypt".to_string(),
                    signature: "i32".to_string(),
                }],
            },
        };

        let context = Context::init().unwrap();
        let mut vm = Vm::new_instance(&wasm, context).unwrap();
        let encoded_args = vec![0u8, 1u8];
        assert_eq!(
            vec![0u8],
            vm.call_encrypt("__fwasm_encrypt_encrypt", encoded_args)
                .unwrap()
        );
    }
}
