use foo_core;
use savvy::{NotAvailableValue, OwnedStringSexp, StringSexp, savvy};

#[savvy]
pub fn to_upper(x: StringSexp) -> savvy::Result<savvy::Sexp> {
    let mut out = OwnedStringSexp::new(x.len())?;

    for (i, e) in x.iter().enumerate() {
        if e.is_na() {
            out.set_na(i)?;
            continue;
        }

        let e_upper = foo_core::to_upper(e);
        out.set_elt(i, &e_upper)?;
    }

    Ok(out.into())
}
