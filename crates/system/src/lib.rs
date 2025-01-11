/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/swamp/limnus
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use core::marker::PhantomData;
use limnus_system_state::State;

// Systems
pub trait System: 'static {
    fn run(&self, world: &mut State);
}

/// Convert to system (to create a trait object)
pub trait IntoSystem<Params> {
    type System: System;

    fn into_system(self) -> Self::System;
}

/// Convert any function with only system params into a system
impl<F, Params: SystemParam> IntoSystem<Params> for F
where
    F: SystemParamFunction<Params>,
{
    type System = FunctionSystem<F, Params>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            system: self,
            params: PhantomData,
        }
    }
}

/// Wraps a system with params
pub struct FunctionSystem<F: 'static, Params: SystemParam> {
    system: F,
    params: PhantomData<Params>,
}

/// Implement `System` for the `FunctionSystem` wrapper
impl<F, Params: SystemParam> System for FunctionSystem<F, Params>
where
    F: SystemParamFunction<Params>,
{
    fn run(&self, state: &mut State) {
        SystemParamFunction::run(&self.system, state);
    }
}

/// A function with only `SystemParam`, which is the only thing that is supported (and unit function)
trait SystemParamFunction<Params: SystemParam>: 'static {
    fn run(&self, state: &mut State);
}

/// Support for unit function, for convenience
impl<F> SystemParamFunction<()> for F
where
    F: Fn() + 'static,
{
    fn run(&self, _state: &mut State) {
        self(); // We don't need state, since we are just calling without any parameter
    }
}

/// implement `SystemParamFunction` for function with one parameter
impl<F, P1: SystemParam<Item = P1>> SystemParamFunction<(P1,)> for F
where
    F: Fn(P1) + 'static,
{
    fn run(&self, world: &mut State) {
        let Some(v1) = P1::get(world) else { return };

        self(v1);
    }
}

/// implement `SystemParamFunction` for function with two parameters
impl<F, P1: SystemParam<Item = P1>, P2: SystemParam<Item = P2>> SystemParamFunction<(P1, P2)> for F
where
    F: Fn(P1, P2) + 'static,
{
    fn run(&self, world: &mut State) {
        let Some(v1) = P1::get(world) else { return };
        let Some(v2) = P2::get(world) else { return };

        self(v1, v2);
    }
}

/// implement `SystemParamFunction` for function with three parameters
impl<F, P1: SystemParam<Item = P1>, P2: SystemParam<Item = P2>, P3: SystemParam<Item = P3>>
    SystemParamFunction<(P1, P2, P3)> for F
where
    F: Fn(P1, P2, P3) + 'static,
{
    fn run(&self, world: &mut State) {
        let Some(v1) = P1::get(world) else { return };
        let Some(v2) = P2::get(world) else { return };
        let Some(v3) = P3::get(world) else { return };

        self(v1, v2, v3);
    }
}

/// implement `SystemParamFunction` for function with four parameters
impl<
        F,
        P1: SystemParam<Item = P1>,
        P2: SystemParam<Item = P2>,
        P3: SystemParam<Item = P3>,
        P4: SystemParam<Item = P4>,
    > SystemParamFunction<(P1, P2, P3, P4)> for F
where
    F: Fn(P1, P2, P3, P4) + 'static,
{
    fn run(&self, world: &mut State) {
        let Some(v1) = P1::get(world) else { return };
        let Some(v2) = P2::get(world) else { return };
        let Some(v3) = P3::get(world) else { return };
        let Some(v4) = P4::get(world) else { return };

        self(v1, v2, v3, v4);
    }
}

/// implement `SystemParamFunction` for function with five parameters
impl<
        F,
        P1: SystemParam<Item = P1>,
        P2: SystemParam<Item = P2>,
        P3: SystemParam<Item = P3>,
        P4: SystemParam<Item = P4>,
        P5: SystemParam<Item = P5>,
    > SystemParamFunction<(P1, P2, P3, P4, P5)> for F
where
    F: Fn(P1, P2, P3, P4, P5) + 'static,
{
    fn run(&self, world: &mut State) {
        let Some(v1) = P1::get(world) else { return };
        let Some(v2) = P2::get(world) else { return };
        let Some(v3) = P3::get(world) else { return };
        let Some(v4) = P4::get(world) else { return };
        let Some(v5) = P5::get(world) else { return };

        self(v1, v2, v3, v4, v5);
    }
}

/// implement `SystemParamFunction` for function with six parameters
impl<
        F,
        P1: SystemParam<Item = P1>,
        P2: SystemParam<Item = P2>,
        P3: SystemParam<Item = P3>,
        P4: SystemParam<Item = P4>,
        P5: SystemParam<Item = P5>,
        P6: SystemParam<Item = P6>,
    > SystemParamFunction<(P1, P2, P3, P4, P5, P6)> for F
where
    F: Fn(P1, P2, P3, P4, P5, P6) + 'static,
{
    fn run(&self, world: &mut State) {
        let Some(v1) = P1::get(world) else { return };
        let Some(v2) = P2::get(world) else { return };
        let Some(v3) = P3::get(world) else { return };
        let Some(v4) = P4::get(world) else { return };
        let Some(v5) = P5::get(world) else { return };
        let Some(v6) = P6::get(world) else { return };

        self(v1, v2, v3, v4, v5, v6);
    }
}

/// implement `SystemParamFunction` for function with seven parameters
impl<
        F,
        P1: SystemParam<Item = P1>,
        P2: SystemParam<Item = P2>,
        P3: SystemParam<Item = P3>,
        P4: SystemParam<Item = P4>,
        P5: SystemParam<Item = P5>,
        P6: SystemParam<Item = P6>,
        P7: SystemParam<Item = P7>,
    > SystemParamFunction<(P1, P2, P3, P4, P5, P6, P7)> for F
where
    F: Fn(P1, P2, P3, P4, P5, P6, P7) + 'static,
{
    fn run(&self, world: &mut State) {
        let Some(v1) = P1::get(world) else { return };
        let Some(v2) = P2::get(world) else { return };
        let Some(v3) = P3::get(world) else { return };
        let Some(v4) = P4::get(world) else { return };
        let Some(v5) = P5::get(world) else { return };
        let Some(v6) = P6::get(world) else { return };
        let Some(v7) = P7::get(world) else { return };

        self(v1, v2, v3, v4, v5, v6, v7);
    }
}

/// implement `SystemParamFunction` for function with eight parameters
impl<
        F,
        P1: SystemParam<Item = P1>,
        P2: SystemParam<Item = P2>,
        P3: SystemParam<Item = P3>,
        P4: SystemParam<Item = P4>,
        P5: SystemParam<Item = P5>,
        P6: SystemParam<Item = P6>,
        P7: SystemParam<Item = P7>,
        P8: SystemParam<Item = P8>,
    > SystemParamFunction<(P1, P2, P3, P4, P5, P6, P7, P8)> for F
where
    F: Fn(P1, P2, P3, P4, P5, P6, P7, P8) + 'static,
{
    fn run(&self, world: &mut State) {
        let Some(v1) = P1::get(world) else { return };
        let Some(v2) = P2::get(world) else { return };
        let Some(v3) = P3::get(world) else { return };
        let Some(v4) = P4::get(world) else { return };
        let Some(v5) = P5::get(world) else { return };
        let Some(v6) = P6::get(world) else { return };
        let Some(v7) = P7::get(world) else { return };
        let Some(v8) = P8::get(world) else { return };

        self(v1, v2, v3, v4, v5, v6, v7, v8);
    }
}

/// Abstraction of a parameter for a system function
pub trait SystemParam: 'static {
    type Item;

    fn get(world: &mut State) -> Option<Self::Item>;
}

impl SystemParam for () {
    type Item = ();

    fn get(_world: &mut State) -> Option<Self::Item> {
        // it is easy to fetch, it is nothing `()`
        None
    }
}

/// implement one parameter for a `SystemParam`
impl<T1> SystemParam for (T1,)
where
    T1: SystemParam,
{
    type Item = (T1::Item,);

    fn get(world: &mut State) -> Option<Self::Item> {
        T1::get(world).map(|t1| (t1,))
    }
}

/// implement two parameters for a `SystemParam`
impl<T1, T2> SystemParam for (T1, T2)
where
    T1: SystemParam,
    T2: SystemParam,
{
    type Item = (T1::Item, T2::Item);

    fn get(world: &mut State) -> Option<Self::Item> {
        T1::get(world).zip(T2::get(world))
    }
}

/// implement three parameters for a `SystemParam`
impl<T1, T2, T3> SystemParam for (T1, T2, T3)
where
    T1: SystemParam,
    T2: SystemParam,
    T3: SystemParam,
{
    type Item = (T1::Item, T2::Item, T3::Item);

    fn get(world: &mut State) -> Option<Self::Item> {
        Some((T1::get(world)?, T2::get(world)?, T3::get(world)?))
    }
}

/// implement four parameters for a `SystemParam`
impl<T1, T2, T3, T4> SystemParam for (T1, T2, T3, T4)
where
    T1: SystemParam,
    T2: SystemParam,
    T3: SystemParam,
    T4: SystemParam,
{
    type Item = (T1::Item, T2::Item, T3::Item, T4::Item);

    fn get(world: &mut State) -> Option<Self::Item> {
        Some((
            T1::get(world)?,
            T2::get(world)?,
            T3::get(world)?,
            T4::get(world)?,
        ))
    }
}

/// implement five parameters for a `SystemParam`
impl<T1, T2, T3, T4, T5> SystemParam for (T1, T2, T3, T4, T5)
where
    T1: SystemParam,
    T2: SystemParam,
    T3: SystemParam,
    T4: SystemParam,
    T5: SystemParam,
{
    type Item = (T1::Item, T2::Item, T3::Item, T4::Item, T5::Item);

    fn get(world: &mut State) -> Option<Self::Item> {
        Some((
            T1::get(world)?,
            T2::get(world)?,
            T3::get(world)?,
            T4::get(world)?,
            T5::get(world)?,
        ))
    }
}

/// implement six parameters for a `SystemParam`
impl<T1, T2, T3, T4, T5, T6> SystemParam for (T1, T2, T3, T4, T5, T6)
where
    T1: SystemParam,
    T2: SystemParam,
    T3: SystemParam,
    T4: SystemParam,
    T5: SystemParam,
    T6: SystemParam,
{
    type Item = (T1::Item, T2::Item, T3::Item, T4::Item, T5::Item, T6::Item);

    fn get(world: &mut State) -> Option<Self::Item> {
        Some((
            T1::get(world)?,
            T2::get(world)?,
            T3::get(world)?,
            T4::get(world)?,
            T5::get(world)?,
            T6::get(world)?,
        ))
    }
}

/// implement seven parameters for a `SystemParam`
impl<T1, T2, T3, T4, T5, T6, T7> SystemParam for (T1, T2, T3, T4, T5, T6, T7)
where
    T1: SystemParam,
    T2: SystemParam,
    T3: SystemParam,
    T4: SystemParam,
    T5: SystemParam,
    T6: SystemParam,
    T7: SystemParam,
{
    type Item = (
        T1::Item,
        T2::Item,
        T3::Item,
        T4::Item,
        T5::Item,
        T6::Item,
        T7::Item,
    );

    fn get(world: &mut State) -> Option<Self::Item> {
        Some((
            T1::get(world)?,
            T2::get(world)?,
            T3::get(world)?,
            T4::get(world)?,
            T5::get(world)?,
            T6::get(world)?,
            T7::get(world)?,
        ))
    }
}

/// implement seven parameters for a `SystemParam`
impl<T1, T2, T3, T4, T5, T6, T7, T8> SystemParam for (T1, T2, T3, T4, T5, T6, T7, T8)
where
    T1: SystemParam,
    T2: SystemParam,
    T3: SystemParam,
    T4: SystemParam,
    T5: SystemParam,
    T6: SystemParam,
    T7: SystemParam,
    T8: SystemParam,
{
    type Item = (
        T1::Item,
        T2::Item,
        T3::Item,
        T4::Item,
        T5::Item,
        T6::Item,
        T7::Item,
        T8::Item,
    );

    fn get(world: &mut State) -> Option<Self::Item> {
        Some((
            T1::get(world)?,
            T2::get(world)?,
            T3::get(world)?,
            T4::get(world)?,
            T5::get(world)?,
            T6::get(world)?,
            T7::get(world)?,
            T8::get(world)?,
        ))
    }
}
