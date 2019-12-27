use rustc::ty::{self, TyCtxt};
use rustc::mir::*;

/// Returns `true` if this place is allowed to be less aligned
/// than its containing struct (because it is within a packed
/// struct).
pub fn is_disaligned<'tcx, L>(
    tcx: TyCtxt<'tcx>,
    local_decls: &L,
    param_env: ty::ParamEnv<'tcx>,
    place: &Place<'tcx>,
) -> bool
where
    L: HasLocalDecls<'tcx>,
{
    debug!("is_disaligned({:?})", place);
    if !is_within_packed(tcx, local_decls, place) {
        debug!("is_disaligned({:?}) - not within packed", place);
        return false
    }

    let ty = place.ty(local_decls, tcx).ty;
    match tcx.layout_raw(param_env.and(ty)) {
        Ok(layout) if layout.align.abi.bytes() == 1 => {
            // if the alignment is 1, the type can't be further
            // disaligned.
            debug!("is_disaligned({:?}) - align = 1", place);
            false
        }
        _ => {
            debug!("is_disaligned({:?}) - true", place);
            true
        }
    }
}

fn is_within_packed<'tcx, L>(tcx: TyCtxt<'tcx>, local_decls: &L, place: &Place<'tcx>) -> bool
where
    L: HasLocalDecls<'tcx>,
{
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    let mut place_projection = &place.projection;

    while let Some(proj) = place_projection {
        match proj.elem {
=======
    let mut cursor = &*place.projection;
    while let [proj_base @ .., elem] = cursor {
        cursor = proj_base;

        match elem {
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
            // encountered a Deref, which is ABI-aligned
            ProjectionElem::Deref => break,
            ProjectionElem::Field(..) => {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
                let ty = Place::ty_from(&place.base, &proj.base, local_decls, tcx).ty;
=======
                let ty = Place::ty_from(&place.base, proj_base, local_decls, tcx).ty;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
                match ty.sty {
                    ty::Adt(def, _) if def.repr.packed() => {
                        return true
                    }
                    _ => {}
                }
            }
            _ => {}
        }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
        place_projection = &proj.base;
=======
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    }

    false
}
