extern crate gnuplot;
extern crate ndarray;
extern crate num;

use gnuplot::*;
use ndarray::*;

use std::thread::sleep;
use std::time::Duration;
use std::f32::consts::PI;
use num::complex::Complex;

fn main()
{
	println!("This is an animation on quantum walk... Ctrl-C to quit.");

	// setting arguments
	let theta = 3.0*PI/12.0;
	let L = 100;

	let P = arr2(&[
					[theta.cos(),theta.sin()],
					[0.0        ,0.0]
					]);
	let Q = arr2(&[
					[0.0        ,0.0],
					[theta.sin(),-theta.cos()]
					]);

	// preparing initial map
	let mut current_prob: Vec<f32>;
	let mut current_state: Vec<&[Complex<f32>; 2]>;
	let root2 = 2 as f32;
	let ini_state = [Complex::from(1.0/root2), Complex::from(1.0/root2)];
	for i in -L..L {
		if i == 0 {
			current_prob.push(1.0);
			current_state.push(ini_state);
		} else {
			current_prob.push(0.0);
			current_state.push(&[Complex::from(0.0),Complex::from(0.0)]);
		}
	}


	// plotting
	let mut fg = Figure::new();
	let mut t = 0;
	let mut x = vec![];
	for i in -L..L {
		x.push(i);
	};
	loop
	{
		fg.clear_axes();
		fg.axes2d()
			.points(x.iter(), current_prob.iter(), &[
                PointSymbol('r'),
            ]);
			t += 1;
		fg.show();
		sleep(Duration::from_millis(50));
	}
}
