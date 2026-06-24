use std::{any::TypeId, fmt::Debug, marker::PhantomData};

use crate::world::World;

pub trait System: 'static {
    fn id(&self) -> TypeId;
    fn run(&mut self, world: *mut World);
}

pub trait SystemParam: Sized + 'static {
    unsafe fn fetch<'w>(world: *mut World) -> Option<Self>;
}

pub trait IntoSystem<Params> {
    fn into_system(self) -> Box<dyn System>;
}

struct FunctionSystem<F, Params> {
    func: F,
    _marker: PhantomData<fn() -> Params>,
}

macro_rules! impl_into_system {
    ($($param:ident),*) => {
        impl<Func, $($param: SystemParam),*> IntoSystem<($($param,)*)> for Func
        where
            Func: FnMut($($param,)*) + 'static,
        {
            fn into_system(self) -> Box<dyn System> {
                Box::new(FunctionSystem {
                    func: self,
                    _marker: std::marker::PhantomData::<fn() -> ($($param,)*)>,
                })
            }
        }

        #[allow(non_snake_case, unused_variables)]
        impl<Func, $($param: SystemParam),*> System for FunctionSystem<Func, ($($param,)*)>
        where
            Func: FnMut($($param,)*) + 'static,
        {
            fn id(&self) -> TypeId {
                TypeId::of::<Self>()
            }

            fn run(&mut self, world: *mut World) {
                unsafe {
                    if let ($(Some($param),)*) = ($($param::fetch(world),)*) {
                        // 3. If they are all Some, execute the system function
                        (self.func)($($param,)*);
                    }

                    // $(let $param = $param::fetch(world_ptr);)*
                    // (self.func)($($param,)*);
                }
            }
        }
    };
}

impl_into_system!();
impl_into_system!(P0);
impl_into_system!(P0, P1);
impl_into_system!(P0, P1, P2);
impl_into_system!(P0, P1, P2, P3);
impl_into_system!(P0, P1, P2, P3, P4);
impl_into_system!(P0, P1, P2, P3, P4, P5);
