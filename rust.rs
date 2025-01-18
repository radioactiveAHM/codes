/// Provides a &'static T from a T for sending a &'static T to threads or tasks.
/// Cheap to clone and move around.
/// Panics if the &T source lifetime is shorter than &'static T.

pub fn unsafe_staticref<'a, T: ?Sized>(r: &'a T) -> &'static T { unsafe { std::mem::transmute::<&'a T, &'static T>(r) } }
