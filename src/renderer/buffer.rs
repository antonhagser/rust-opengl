pub trait Buffer {
    /// Bind buffer
    fn bind(&self);

    /// Unbind buffer
    fn unbind(&self);
}