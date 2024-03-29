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
const SECTODAY: f64 = 1.0 / (24.0 * 3600.0);
const TODAYSEC: f64 = (24.0 * 3600.0) * (METERTOAU * METERTOAU);
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
    let z = mars.eccentricity_vec().dot(&mars.angular_momentum()) / mars.angular_momentum().norm();
    let f_frame_mars = Body {
        position: f_frame * mars.position,
        velocity: f_frame * mars.velocity,
    };
    let (r0, v0) = f_frame_mars.position_and_velocity(0.0);
    let (r90, v90) = f_frame_mars.position_and_velocity(90.0);
    let (r180, v180) = f_frame_mars.position_and_velocity(180.0);

    printer!("Mars Position", v => mars.position);
    printer!("Mars Velocity", v => mars.velocity);
    printer!("e_xi", v => f_frame.row(0));
    printer!("e_eta", v => f_frame.row(1));
    printer!("e_zeta", v => f_frame.row(2));
    printer!("Mars Angular Momentum", v => mars.angular_momentum());
    printer!("F Frame Angular Momentum", v => f_frame * mars.angular_momentum());
    printer!("Mars Orbital Energy", s => mars.total_energy());
    printer!("Mars Eccentricty Vector", v => mars.eccentricity_vec());
    printer!("Z", s => z);
    printer!("AA", s => mars.eccentricity_vec().dot(&mars.angular_momentum()));
    printer!("F Frame Eccentricity Vec", s => (f_frame * mars.eccentricity_vec()).norm());
    printer!("Mars' Semimajor Axis", s => mars.semi_major_axis() * METERTOAU);
    printer!("Mars' Orbital Parameter", s => mars.orbital_parameter() * METERTOAU);
    printer!("Mars' True Anomaly", s => f_frame_mars.true_anomaly());
    printer!("Position at 0 Degrees", v => r0 * METERTOAU);
    printer!("Velocity at 0 Degrees", v => v0 * TODAYSEC);
    printer!("Position at 90 Degrees", v => r90 * METERTOAU);
    printer!("Velocity at 90 Degrees", v => v90 * TODAYSEC);
    printer!("Position at 180 Degrees", v => r180 * METERTOAU);
    printer!("Velocity at 180 Degrees", v => v180 * TODAYSEC);
}
