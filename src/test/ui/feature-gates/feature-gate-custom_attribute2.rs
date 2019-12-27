// This test ensures that attributes on formals in generic parameter
// lists are included when we are checking for unstable attributes.

// gate-test-custom_attribute

struct StLt<#[lt_struct] 'a>(&'a u32);
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `lt_struct` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `lt_struct` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
struct StTy<#[ty_struct] I>(I);
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `ty_struct` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `ty_struct` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

enum EnLt<#[lt_enum] 'b> { A(&'b u32), B }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `lt_enum` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `lt_enum` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
enum EnTy<#[ty_enum] J> { A(J), B }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `ty_enum` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `ty_enum` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

trait TrLt<#[lt_trait] 'c> { fn foo(&self, _: &'c [u32]) -> &'c u32; }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `lt_trait` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `lt_trait` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
trait TrTy<#[ty_trait] K> { fn foo(&self, _: K); }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `ty_trait` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `ty_trait` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

type TyLt<#[lt_type] 'd> = &'d u32;
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `lt_type` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `lt_type` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
type TyTy<#[ty_type] L> = (L, );
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `ty_type` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `ty_type` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

impl<#[lt_inherent] 'e> StLt<'e> { }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `lt_inherent` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `lt_inherent` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
impl<#[ty_inherent] M> StTy<M> { }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `ty_inherent` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `ty_inherent` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

impl<#[lt_impl_for] 'f> TrLt<'f> for StLt<'f> {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR the attribute `lt_impl_for` is currently unknown to the compiler
=======
    //~^ ERROR cannot find attribute `lt_impl_for` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    fn foo(&self, _: &'f [u32]) -> &'f u32 { loop { } }
}
impl<#[ty_impl_for] N> TrTy<N> for StTy<N> {
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR the attribute `ty_impl_for` is currently unknown to the compiler
=======
    //~^ ERROR cannot find attribute `ty_impl_for` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    fn foo(&self, _: N) { }
}

fn f_lt<#[lt_fn] 'g>(_: &'g [u32]) -> &'g u32 { loop { } }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `lt_fn` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `lt_fn` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
fn f_ty<#[ty_fn] O>(_: O) { }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
//~^ ERROR the attribute `ty_fn` is currently unknown to the compiler
=======
//~^ ERROR cannot find attribute `ty_fn` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)

impl<I> StTy<I> {
    fn m_lt<#[lt_meth] 'h>(_: &'h [u32]) -> &'h u32 { loop { } }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR the attribute `lt_meth` is currently unknown to the compiler
=======
    //~^ ERROR cannot find attribute `lt_meth` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    fn m_ty<#[ty_meth] P>(_: P) { }
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR the attribute `ty_meth` is currently unknown to the compiler
=======
    //~^ ERROR cannot find attribute `ty_meth` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
}

fn hof_lt<Q>(_: Q)
    where Q: for <#[lt_hof] 'i> Fn(&'i [u32]) -> &'i u32
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    //~^ ERROR the attribute `lt_hof` is currently unknown to the compiler
=======
    //~^ ERROR cannot find attribute `lt_hof` in this scope
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
{
}

fn main() {

}
