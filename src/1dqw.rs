extern crate gnuplot;
extern crate nalgebra as na;
extern crate num;

use gnuplot::*;
use na::base::Matrix1;
use na::base::Matrix2;
use na::base::Matrix2x1;
use std::thread::sleep;
use std::time::Duration;
use std::f64::consts::PI;
use std::f64::consts::FRAC_1_SQRT_2;
use num::complex::Complex;

pub type State = Matrix2x1<Complex<f64>>;

fn main()
{
	// setting arguments
	let theta: f64 = 12.*PI/12.;
	const L: usize = 200;
	fn to_complex(real: f64) -> Complex<f64> {
		Complex{
			re: real,
			im: 0.
		}
	}

	// preparing time development matrix
	let P: Matrix2<Complex<f64>> = Matrix2::new(
		to_complex(theta.cos()), to_complex(theta.sin()),
		to_complex(0.) , to_complex(0.)
	);
	let Q: Matrix2<Complex<f64>> = Matrix2::new(
		to_complex(0.) , to_complex(0.) ,
		to_complex(theta.sin()), to_complex(-theta.cos())
	);
	println!("P = {}, Q = {}", P, Q);

	// prepareing State type and state map
	let mut prob: Vec<f64> = vec![];
	let mut state_map: Vec<State> = vec![];
	// intitialize
	let alpha: Complex<f64> = Complex::new(FRAC_1_SQRT_2, 0.);
	let beta: Complex<f64> = Complex::new(0., FRAC_1_SQRT_2);
	for j in 0..2*L+1 {
		if j == L {
			state_map.push(State::new(alpha, beta));
			prob.push(1.);
		} else {
			state_map.push(State::new(Complex::new(0.,0.), Complex::new(0., 0.)));
			prob.push(0.);
		}
	}

	fn develop(state_map: &mut Vec<State>, prob: &mut Vec<f64>, P: &Matrix2<Complex<f64>>, Q: &Matrix2<Complex<f64>>) {
		for i in 0..2*L+1 {
			if i == 0 {
				state_map[i] = P*state_map[i+1];
			} else if i == 2*L {
				state_map[i] = Q*state_map[i-1];
			} else {
				state_map[i] = P*state_map[i+1] + Q*state_map[i-1];
			}
			prob[i] = (state_map[i].conjugate_transpose()*state_map[i]).trace().re;
		}
	}

	println!("This is an animation on quantum walk... Ctrl-C to quit.");
	let mut fg = Figure::new();
	loop
	{
		develop(&mut state_map, &mut prob, &P, &Q);
		fg.clear_axes();
		fg.axes2d()
			.lines_points(0..2*L+1, &prob, &[
                PointSymbol('r'),
            ]);
		fg.show();
		sleep(Duration::from_millis(100));
	}
}
