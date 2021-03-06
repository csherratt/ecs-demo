use specs::prelude::*;

pub mod baseline_benches;
pub mod legion_benches;
pub mod legion_parallel;
pub mod specs_benches;
pub mod specs_parallel;

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct A(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct B(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct C(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct D(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct E(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct F(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct G(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct H(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct I(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct J(u32);

#[derive(Component, Copy, Clone, Debug, Default)]
#[storage(VecStorage)]
pub struct K(u32);

