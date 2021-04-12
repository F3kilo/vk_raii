pub mod buffer;
pub mod command_buffer;
pub mod command_pool;
pub mod debug_report;
pub mod descr_pool;
pub mod descr_set;
pub mod device;
pub mod ds_layout;
pub mod fence;
pub mod instance;
pub mod memory;
pub mod pipeline;
pub mod pipeline_cache;
pub mod pipeline_layout;
pub mod queue;
pub mod render_pass;
pub mod sampler;
pub mod shader_module;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

pub trait RawHandle {
    type Dependencies;

    fn name() -> &'static str;

    fn destroy(&self, deps: &Self::Dependencies);
}

struct UniqueData<T, D> {
    handle: T,
    dependencies: D,
}

pub struct UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    data: Option<UniqueData<T, D>>,
}

impl<T: fmt::Debug, D> fmt::Debug for UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.handle(), f)
    }
}

impl<T: PartialEq, D> PartialEq for UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.handle(), &other.handle())
    }
}

impl<T: PartialEq, D> Eq for UniqueHandle<T, D> where T: RawHandle<Dependencies = D> {}

impl<T: PartialOrd, D> PartialOrd for UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.handle(), &other.handle())
    }
}

impl<T: Eq + PartialOrd + Ord, D> Ord for UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.handle(), &other.handle())
    }
}

impl<T: Hash, D> Hash for UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle().hash(state)
    }
}

impl<T, D> UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    /// # Safety
    /// * `handle` must be valud initialized handle;
    /// * `dependencies` must contain valid and initialized handles;
    pub unsafe fn new(handle: T, dependencies: D) -> Self {
        log::trace!("Unique {} initialized", T::name());
        Self {
            data: Some(UniqueData {
                handle,
                dependencies,
            }),
        }
    }

    pub fn handle(&self) -> &T {
        if let Some(d) = &self.data {
            return &d.handle;
        }
        unreachable!()
    }

    pub fn dependencies(&self) -> &D {
        if let Some(d) = &self.data {
            return &d.dependencies;
        }
        unreachable!()
    }

    pub fn into_inner(mut self) -> (T, D) {
        log::trace!("Unique {} unwrapped", T::name());
        let d = self
            .data
            .take()
            .expect("Try convert uninitialized unique handle into inner");
        (d.handle, d.dependencies)
    }
}

impl<T, D> Drop for UniqueHandle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn drop(&mut self) {
        if let Some(d) = &self.data {
            log::trace!("Unique {} destroyed", T::name());
            d.handle.destroy(&d.dependencies)
        }
    }
}

pub struct Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    handle: Arc<UniqueHandle<T, D>>,
}


impl<T: fmt::Debug, D> fmt::Debug for Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.handle(), f)
    }
}

impl<T: PartialEq, D> PartialEq for Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.handle(), &other.handle())
    }
}

impl<T: PartialEq, D> Eq for Handle<T, D> where T: RawHandle<Dependencies = D> {}

impl<T: PartialOrd, D> PartialOrd for Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.handle(), &other.handle())
    }
}

impl<T: Eq + PartialOrd + Ord, D> Ord for Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.handle(), &other.handle())
    }
}

impl<T: Hash, D> Hash for Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle().hash(state)
    }
}

impl<T, D> Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    /// # Safety
    /// * `handle` must be valud initialized handle;
    /// * `dependencies` must contain valid and initialized handles;
    pub unsafe fn new(handle: T, dependencies: D) -> Self {
        let unique = UniqueHandle::new(handle, dependencies);
        Self {
            handle: Arc::new(unique),
        }
    }

    pub fn handle(&self) -> &T {
        &self.handle.handle()
    }

    pub fn dependencies(&self) -> &D {
        &self.handle.dependencies()
    }

    pub fn try_unwrap(self) -> Result<(T, D), Self> {
        match Arc::try_unwrap(self.handle) {
            Ok(unique) => Ok(unique.into_inner()),
            Err(e) => Err(Self { handle: e }),
        }
    }

    /// Count of references to this handle.
    pub fn reference_count(&self) -> usize {
        Arc::strong_count(&self.handle)
    }
}

impl<T, D> Clone for Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

impl<T, D> Deref for Handle<T, D>
where
    T: RawHandle<Dependencies = D>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.handle()
    }
}