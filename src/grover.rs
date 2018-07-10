extern crate gnuplot;
extern crate ndarray;

use gnuplot::*;
use std::thread::sleep;
use std::time::Duration;
use std::f64::consts::PI;

use ndarray::{Array, Ix1, Ix2};

fn main() {
	// setting arguments
    let n: usize = 5;// size
    let tmp: u32 = n as u32;
    let N: usize = 2_u32.pow(tmp) as usize;// size
    let temp = N as f64;

	// preparing time development matrix
    let mut vec: Vec<f64> = vec![];
    for i in 0..N {
        for j in 0..N {
            if i == j {
                if i == 2 {
                    vec.push(-1.);
                } else {
                    vec.push(1.);
                }
            } else {
                vec.push(0.);
            }
        }
    }
    let opCZ: Array<f64, Ix2> = Array::from_vec(vec).into_shape((N, N)).unwrap();
    let mut vec1: Vec<f64> = vec![];
    for i in 0..N {
        for j in 0..N {
            if i == j {
                vec1.push(2./temp - 1.);
            } else {
                vec1.push(2./temp);
            }
        }
    }
    let opD: Array<f64, Ix2> = Array::from_vec(vec1).into_shape((N, N)).unwrap();
    let op: Array<f64, Ix2> = opCZ.dot(&opD);

    // preparing state
    let mut vec2: Vec<f64> = vec![];
    for i in 0..N {
        vec2.push(1./temp.sqrt());
    }
    let mut state: Array<f64, Ix1> = Array::from_vec(vec2).into_shape(N).unwrap();

    fn develop(state: &Array<f64, Ix1> , op: &Array<f64, Ix2>) -> Array<f64, Ix1> {
        return state.dot(op);
    }

    // execute
	println!("This is an animation on quantum walk... Ctrl-C to quit.");
	let mut fg = Figure::new();
    loop
    {
        let next_state = develop(&state, &op);
        state = next_state;
		fg.clear_axes();
		fg.axes2d()
			.set_size(1.0, 1.0)
			.set_title("Grover", &[])
			.set_x_label("Position", &[])
			.set_y_label("Probability", &[])
			.set_y_range(Fix(0.), Fix(1.))
			.lines(0..N, &state.mapv(|a| a.powi(2)), &[]);
		fg.show();
		sleep(Duration::from_millis(500));
    }
}
