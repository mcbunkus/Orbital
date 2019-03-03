#![allow(dead_code)]
#![allow(unused_doc_comments)]

/**
 * Austen LeBeau
 * ENGR 3310-002
 */
extern crate nalgebra;
use nalgebra::Vector3;

extern crate colored;
use colored::*;

#[macro_use]
mod util;
use self::util::*;

const METERTOAU: f64 = 1.0 / 1.49597870700e11;
const KMTOM: f64 = 1000_f64;

fn main() {
    let mars = Body {
        position: Vector3::new(
            8.535066315950862E+07,
            2.109100603746325E+08,
            2.290206680751368E+06,
        ) * KMTOM,
        velocity: Vector3::new(
            -2.153469826777515E+01,
            1.121061951236782E+01,
            7.632686135580742E-01,
        ) * KMTOM,
    };

    /* Constructing F frame from mars' orbital parameters */
    let f_frame = mars.make_frame();

    let Z = mars.eccentricity_vec().dot(&mars.angular_momentum()) / mars.angular_momentum().norm();

    printer!("Mars Position", v => mars.position);
    printer!("Mars Velocity", v => mars.velocity);
    printer!("e_xi", v => f_frame.row(0));
    printer!("e_eta", v => f_frame.row(1));
    printer!("e_zeta", v => f_frame.row(2));
    printer!("Mars Angular Momentum", v => mars.angular_momentum());
    printer!("Mars Orbital Energy", s => mars.total_energy());
    printer!("Mars Eccentricty Vector", v => mars.eccentricity_vec());
    printer!("Z", s => Z);
}