use blake2b_rs::{Blake2b, Blake2bBuilder};
use ckb_testtool::{
    ckb_error::Error as CKBError,
    ckb_jsonrpc_types,
    ckb_types::{bytes::Bytes, core::TransactionView},
    context::Context,
};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

const MAX_CYCLES: u64 = 10_000_000;

#[cfg(test)]
mod component_definition_type_tests;
#[cfg(test)]
mod component_lock_tests;

const TEST_ENV_VAR: &str = "CAPSULE_TEST_ENV";

pub enum TestEnv {
    Debug,
    Release,
}

impl FromStr for TestEnv {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(TestEnv::Debug),
            "release" => Ok(TestEnv::Release),
            _ => Err("no match"),
        }
    }
}

pub struct Loader(PathBuf);

impl Default for Loader {
    fn default() -> Self {
        let test_env = match env::var(TEST_ENV_VAR) {
            Ok(val) => val.parse().expect("test env"),
            Err(_) => TestEnv::Debug,
        };
        Self::with_test_env(test_env)
    }
}

impl Loader {
    fn with_test_env(env: TestEnv) -> Self {
        let load_prefix = match env {
            TestEnv::Debug => "debug",
            TestEnv::Release => "release",
        };
        env::current_dir().unwrap();
        let mut base_path = PathBuf::new();
        // cargo may use a different cwd when running tests, for example:
        // when running debug in vscode, it will use workspace root as cwd by default,
        // when running test by `cargo test`, it will use tests directory as cwd,
        // so we need a fallback path
        base_path.push("build");
        if !base_path.exists() {
            base_path.pop();
            base_path.push("..");
            base_path.push("build");
        }
        base_path.push(load_prefix);
        Loader(base_path)
    }

    pub fn load_binary(&self, name: &str) -> Bytes {
        let mut path = self.0.clone();
        path.push(name);
        fs::read(path).expect("binary").into()
    }
}

pub const CKB_PERSONALIZATION: &[u8] = b"ckb-default-hash";
pub fn new_blake2b() -> Blake2b {
    Blake2bBuilder::new(32)
        .personal(CKB_PERSONALIZATION)
        .build()
}

pub fn dump_tx(tx: &TransactionView) {
    let json: ckb_jsonrpc_types::TransactionView = tx.clone().into();
    println!("{}", serde_json::to_string(&json).unwrap());
}

pub fn verify_tx(context: &mut Context, tx: TransactionView) -> Result<u64, CKBError> {
    let tx = context.complete_tx(tx);
    context.verify_tx(&tx, MAX_CYCLES)
}

pub fn assert_tx_ok(context: &mut Context, tx: TransactionView, msg: &str) {
    if let Err(err) = verify_tx(context, tx) {
        panic!("expect {} ok but got err: {}", msg, err);
    }
}

pub fn assert_tx_err(context: &mut Context, tx: TransactionView, msg: &str, err_code: i8) {
    match verify_tx(context, tx) {
        Ok(_) => panic!("expect {} with err code {} but got ok", msg, err_code),
        Err(err) => {
            assert!(
                err.to_string()
                    .contains(format!("error code {} ", err_code).as_str()),
                "expect {} with err code {} but got: {}",
                msg,
                err_code,
                err
            )
        }
    }
}
