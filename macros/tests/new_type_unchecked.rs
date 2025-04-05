#![allow(dead_code)]

use macros::NewUnchecked;
struct Foo;

#[derive(NewUnchecked)]
struct FooTuple(Foo);

#[derive(NewUnchecked)]
struct Wrapper<T>(T);

#[derive(NewUnchecked)]
struct WrapperList<T>(Vec<T>);

#[derive(NewUnchecked)]
struct WrapperRef<'a, T>(&'a T);

#[derive(NewUnchecked)]
struct WrapperRefMut<'a, T>(&'a mut T);
