cfg_if::cfg_if! {
    if #[cfg(feature = "rayon")] {
        type MaybeScope<'a> = rayon::Scope<'a>;
    }else {
        type MaybeScope<'a> = ();
    }
}

pub fn maybe_scope<'scope, OP, R>(op: OP) -> R
where
    OP: FnOnce(&MaybeScope<'scope>) -> R + Send,
    R: Send,
{
    cfg_if::cfg_if! {
        if #[cfg(feature = "rayon")] {
            rayon::scope(op)
        }else {
            op(&())
        }
    }
}
