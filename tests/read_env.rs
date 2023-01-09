use derenv::dotenv;

#[dotenv(path = ".env.test", public = true)]
pub struct ProjectEnv;

#[dotenv(path = "tests/.env")]
pub struct LocalEnv;
