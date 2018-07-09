extern crate gnuplot;
use gnuplot::*;

use std::thread::sleep;
use std::time::Duration;

fn main()
{
	println!("This is a silly example of doing an animation... Ctrl-C to quit.");
	let mut fg = Figure::new();
	let mut x = vec![];
	let mut y = vec![];

	let mut t: f32 = 0.0;
	loop
	{
		x.push(t.sin() as f32);
		y.push(t.cos() as f32);
		fg.clear_axes();
		fg.axes2d()
			.points(x.iter(), y.iter(), &[
                PointSymbol('r'),
            ]);
			t += 0.1;
		fg.show();
		sleep(Duration::from_millis(50));
	}
}