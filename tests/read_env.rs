use derenv::dotenv;

#[dotenv(".env.test")]
pub struct ProjectEnv {}

#[dotenv("tests/.env")]
pub struct LocalEnv {}

#[test]
fn tt() {
    LocalEnv::grab();
}
