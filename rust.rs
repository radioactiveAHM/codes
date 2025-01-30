{
/// Provides a &'static T from a T for sending a &'static T to threads or tasks.
/// Cheap to clone and move around.
/// Panics if the &T source lifetime is shorter than &'static T.

pub fn unsafe_staticref<'a, T: ?Sized>(r: &'a T) -> &'static T { unsafe { std::mem::transmute::<&'a T, &'static T>(r) } }
}

{
/// Direct proxy using tokio
    let (mut client_read, mut client_write) = split(stream);
    let (mut server_read, mut server_write) = split(target);

    // Forward packets between the client and the server
    tokio::try_join!(
        tokio::io::copy(&mut client_read, &mut server_write),
        tokio::io::copy(&mut server_read, &mut client_write),
    )?;
}
