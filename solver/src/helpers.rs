macro_rules! abort_if {
    ($x: expr) => {
        if $x {
            return false;
        }
    };
}

pub(crate) use abort_if;
