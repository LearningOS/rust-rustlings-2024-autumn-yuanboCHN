// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.

 

use std::f32::consts::PI;

fn main() {
    let radius: f32 = 5.0;  // 显式指定为 f32 类型

    let area = PI * radius.powi(2);  // 现在 powi 可以正确工作

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}