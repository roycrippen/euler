//! Project Euler solutions runner crate. (https://projecteuler.net)
//!
//! Run one solution or all concurrently.

use std::env;

extern crate euler_solutions;
use euler_solutions as sol;

use euler_solutions::eu001_010;
use euler_solutions::eu011_020;
use euler_solutions::eu021_030;
use euler_solutions::eu031_040;
use euler_solutions::eu041_050;
use euler_solutions::eu051_060;
use euler_solutions::eu061_070;
use euler_solutions::eu071_080;
use euler_solutions::eu081_090;
use euler_solutions::eu091_100;
use euler_solutions::eu101_110;
use euler_solutions::eu111_120;
use euler_solutions::eu121_130;

/// Run 1 solution given in arg or all solutions concurrently.
pub fn main() {
    let mut fns = eu001_010::get_functions();
    fns.append(&mut eu011_020::get_functions());
    fns.append(&mut eu021_030::get_functions());
    fns.append(&mut eu031_040::get_functions());
    fns.append(&mut eu041_050::get_functions());
    fns.append(&mut eu051_060::get_functions());
    fns.append(&mut eu061_070::get_functions());
    fns.append(&mut eu071_080::get_functions());
    fns.append(&mut eu081_090::get_functions());
    fns.append(&mut eu091_100::get_functions());
    fns.append(&mut eu101_110::get_functions());
    fns.append(&mut eu111_120::get_functions());
    fns.append(&mut eu121_130::get_functions());

    sol::run(fns.clone(), env::args().nth(1), 1);
}
