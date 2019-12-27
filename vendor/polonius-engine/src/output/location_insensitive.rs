// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::collections::BTreeSet;
use std::time::Instant;

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
=======
use crate::output::initialization;
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
use crate::output::liveness;
use crate::output::Output;

use datafrog::{Iteration, Relation, RelationLeaper};
use facts::{AllFacts, Atom};

<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
pub(super) fn compute<Region: Atom, Loan: Atom, Point: Atom, Variable: Atom>(
=======
pub(super) fn compute<Region: Atom, Loan: Atom, Point: Atom, Variable: Atom, MovePath: Atom>(
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    dump_enabled: bool,
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    all_facts: &AllFacts<Region, Loan, Point, Variable>,
) -> Output<Region, Loan, Point, Variable> {
=======
    all_facts: &AllFacts<Region, Loan, Point, Variable, MovePath>,
) -> Output<Region, Loan, Point, Variable, MovePath> {
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
    let mut result = Output::new(dump_enabled);
<<<<<<< HEAD   (086005 Importing rustc-1.38.0)
    let region_live_at = liveness::init_region_live_at(
        all_facts.var_used.clone(),
        all_facts.var_drop_used.clone(),
        all_facts.var_defined.clone(),
        all_facts.var_uses_region.clone(),
        all_facts.var_drops_region.clone(),
        all_facts.var_initialized_on_exit.clone(),
        &all_facts.cfg_edge,
        all_facts.region_live_at.clone(),
=======
    let var_maybe_initialized_on_exit = initialization::init_var_maybe_initialized_on_exit(
        all_facts.child.clone(),
        all_facts.path_belongs_to_var.clone(),
        all_facts.initialized_at.clone(),
        all_facts.moved_out_at.clone(),
        all_facts.path_accessed_at.clone(),
        &all_facts.cfg_edge,
        &mut result,
    );
    let region_live_at = liveness::init_region_live_at(
        all_facts.var_used.clone(),
        all_facts.var_drop_used.clone(),
        all_facts.var_defined.clone(),
        all_facts.var_uses_region.clone(),
        all_facts.var_drops_region.clone(),
        var_maybe_initialized_on_exit.clone(),
        &all_facts.cfg_edge,
>>>>>>> BRANCH (8cd2c9 Importing rustc-1.39.0)
        all_facts.universal_region.clone(),
        &mut result,
    );

    let potential_errors_start = Instant::now();

    let potential_errors = {
        // Create a new iteration context, ...
        let mut iteration = Iteration::new();

        // static inputs
        let region_live_at: Relation<(Region, Point)> = region_live_at.into();
        let invalidates = Relation::from_iter(all_facts.invalidates.iter().map(|&(b, p)| (p, b)));

        // .. some variables, ..
        let subset = iteration.variable::<(Region, Region)>("subset");
        let requires = iteration.variable::<(Region, Loan)>("requires");

        let potential_errors = iteration.variable::<(Loan, Point)>("potential_errors");

        // load initial facts.

        // subset(R1, R2) :- outlives(R1, R2, _P)
        subset.extend(all_facts.outlives.iter().map(|&(r1, r2, _p)| (r1, r2)));

        // requires(R, B) :- borrow_region(R, B, _P).
        requires.extend(all_facts.borrow_region.iter().map(|&(r, b, _p)| (r, b)));

        // .. and then start iterating rules!
        while iteration.changed() {
            // requires(R2, B) :- requires(R1, B), subset(R1, R2).
            //
            // Note: Since `subset` is effectively a static input, this join can be ported to
            // a leapjoin. Doing so, however, was 7% slower on `clap`.
            requires.from_join(&requires, &subset, |&_r1, &b, &r2| (r2, b));

            // borrow_live_at(B, P) :- requires(R, B), region_live_at(R, P)
            // potential_errors(B, P) :- invalidates(B, P), borrow_live_at(B, P).
            //
            // Note: we don't need to materialize `borrow_live_at` here
            // so we can inline it in the `potential_errors` relation.
            //
            potential_errors.from_leapjoin(
                &requires,
                (
                    region_live_at.extend_with(|&(r, _b)| r),
                    invalidates.extend_with(|&(_r, b)| b),
                ),
                |&(_r, b), &p| (b, p),
            );
        }

        if dump_enabled {
            let subset = subset.complete();
            for (r1, r2) in &subset.elements {
                result
                    .subset_anywhere
                    .entry(*r1)
                    .or_insert(BTreeSet::new())
                    .insert(*r2);
            }

            let requires = requires.complete();
            for (region, borrow) in &requires.elements {
                result
                    .restricts_anywhere
                    .entry(*region)
                    .or_insert(BTreeSet::new())
                    .insert(*borrow);
            }

            for (region, location) in &region_live_at.elements {
                result
                    .region_live_at
                    .entry(*location)
                    .or_insert(vec![])
                    .push(*region);
            }
        }

        potential_errors.complete()
    };

    if dump_enabled {
        info!(
            "potential_errors is complete: {} tuples, {:?}",
            potential_errors.len(),
            potential_errors_start.elapsed()
        );
    }

    for (borrow, location) in &potential_errors.elements {
        result
            .errors
            .entry(*location)
            .or_insert(Vec::new())
            .push(*borrow);
    }

    result
}
