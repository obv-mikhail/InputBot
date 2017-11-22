use ::*;

pub type BindHandler = Arc<Fn() + Send + Sync + 'static>;