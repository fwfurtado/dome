#[derive(Clone, Debug)]
pub struct Config<T> {
    inner: T,
}

impl<T> Default for Config<T>
where
    T: Default,
{
    fn default() -> Self {
        Config::new(T::default())
    }
}

impl<T> Config<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn get(&self) -> &T {
        &self.inner
    }
}
