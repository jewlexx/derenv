use derenv::dotenv;

#[dotenv(path = "../.env.test", public = true)]
pub struct ProjectEnv;

pub struct LocalEnv;
