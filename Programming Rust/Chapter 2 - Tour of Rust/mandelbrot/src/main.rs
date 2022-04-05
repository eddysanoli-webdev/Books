// ============================
// Plot the Mandelbrot Set
// Often called an "embarrassingly parallel" algorithm, the mandelbrot set consists of a
// fractal produced by iterating a simple function on complex numbers. 
// 
// Mathematically, the mandelbrot set is defined as the set of numbers "c" that,
// after being added to a recursively squared number, doesn't cause said number to go into 
// infinity.
//                                  z = z^2 + c
// 
// For a real "c", values greater than 0.25 or less than -2 cause the squaring to diverge.
// However, for a complex "c", the non-divergent values are way harder to deduce analytically.
// Assuming that the imaginary plane corresponds to the X-Y plane, we would have each point
// in the plane checked to see if it blows up. Now, if we check a point and it diverges, we 
// can color it black, while coloring it in a lighter color if it does blow up. This in turn 
// generates a beautiful pattern known as a fractal. This is what we are trying to implement
// and visualize.
// ============================

use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn main() {
    println!("Hello, world!");
}

// ============================
// CREATE IMAGE

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    
    // Error Handling: If an error occurs is either because "File::create" or "encoder.encode"
    // failed. The return type of each method is:
    //      - File::create : Result<std::fs::File, std::io::Error>
    //      - encoder.encode : Result<(), std::io::Error>
    // 
    // Both share the same error type: "std::io::Error". It makes sense for our "write_image"
    // function to do the same. When either process fails, we return a "Result" where we
    // ignore the first argument and only utilize the common error type. All errors are
    // handled using the "?" operator.

    // Create a new file
    let output = File::create(filename)?;

    // Create a PNG encoder for the new file
    let encoder = PNGEncoder::new(output);

    // Give the encoder the pixel data, the width and height of the
    // picture, and a guide on how to interpret the bytes in the pixel data:
    // In this case, each value consists of an eight-bit grayscale value.
    encoder.encode(&pixels,
                   bounds.0 as u32, bounds.1 as u32,
                   ColorType::Gray(8))?;
    
    // If everything goes well, "write_image" has no useful value to return
    // (all useful data went to the image), so its success type is the unit type ().
    // Very similar to void in C and C++
    Ok(())
}


// ============================
// RENDER

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.

fn render(pixels: &mut [u8], 
          bounds: (usize, usize), 
          upper_left: Complex<f64>, 
          lower_right: Complex<f64>)
{   
    // The pixel array must be the same as the "width x height"
    assert!(pixels.len() == bounds.0 * bounds.1);

    // Iterate through the one dimensional pixel array like we are moving
    // through a grid
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {

            // Map from pixel coordinates to imaginary coordinates
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            // Get the number of iterations that it took for each point to converge or
            // diverge. If "None" is obtained (not part of the Mandelbrot set) we color
            // the pixel in the current position as black (0). If we get a number of iterations
            // (from 0 to 255), we color it the inverse of the number iterations (255 - Iterations)
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }

        }
    }
}

// ============================
// PIXELS TO COMPLEX NUMBERS MAP

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.

fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), 
                  upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64> 
{

    // Width  = X right-most coordinate - X left-most coordinate
    // Height = Y top coordinate - Y bottom coordinate
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex{
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,

        // Why a subtraction here? Pixel.1 increases as we go down, but the imaginary
        // component increases as we go up.
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }

}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75),
    Complex { re: -1.0, im: 1.0 },
    Complex { re: 1.0, im: -1.0 }),
    Complex { re: -0.5, im: -0.5 });
}


// ============================
// PARSE COMPLEX

/// Parse a pair of floating-point numbers separated by a comma as a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>> {

    // If the parse is successful, we use the tuple's elements to create and return 
    // a new complex number. Its important to notice that here we dont do: {re: re, im: im}, 
    // but {re, im}. This is shorthand for passing each element to the "Complex" struct.
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"),
               Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}


// ============================
// PARSE ARGUMENT PAIR

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are both
/// strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`. If it doesn't parse
/// correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {

    // "parse_pair<T: FromStr>" 
    // Reads as: 'for any type T that implements the "FromStr" trait'.
    // Defines a "parse_pair" method for all numeric types in Rust
    // 
    // "Option<(T, T)>"
    // Its either "None" or a value Some((v1, v2)), where (v1, v2) is a tuple of 
    // type T.


    // The "String" type's "find" method searches the string for a character that 
    // matches the variable "separator". If "find" returns None, no match was found causing
    // "parse_pair" to return None. Otherwise, we take "index" to be the separator's position
    //  in the string (eg. Separator = "x", s = "2x2" -> index = 1).
    match s.find(separator) {
        None => None,
        Some(index) => {

            // (T::from_str(&s[..index]), T::from_str(&s[index + 1..]))
            //  - &s[..index]: Slice of the string preceding the "separator"
            //  - &s[index + 1..]: Slice of the string following the "separator"
            //  - T::from_str(): We take the type used as a type parameter (argument after
            //    "parse_pair") and we use its "from_str" method to convert both string slices
            //    to a tuple of numbers of the form (num before separator, num after separator)
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {

                // Check if the result of the "from_str" method call, was successful for
                // both elements of the tuple. If it was successful, return the numeric tuple
                (Ok(l), Ok(r)) => Some((l, r)),

                // If one or both of the elements of the tuple failed the parsing step,
                // we return None, meaning that no valid arguments were passed 
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

// ============================
// ESCAPE TIME

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {

    // Its traditional to use "z" for complex numbers.
    // The crate's num "Complex" type is a struct defined as follows:
    // 
    //   struct Complex<T> {
    //     re: T, 
    //     im: T,
    //   }
    // 
    // This code defines a generic struct with two fields: re and im. It's generic
    // because the "<T>" after the type name is read as "for any type T". Here we
    // are intializing both the real and imaginary values of the complex value. The
    // "num" crate makes sure that any default operation between numbers (-, +, *)
    // is also valid for complex ones.
    let mut z = Complex { re: 0.0, im: 0.0 };

    // Iterate from 0 to (no including) the Limit 
    for i in 0..limit {

        // We get the squared norm (X^2 + Y^2) and check if that distance is higher
        // than 4 to check if "z" has left the circle of radius two.
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

// NOTE: The function's return value is an Option<usize>. Rust's standard library defines the
// "Option" type as follows:
// 
//      enum Option<T> {
//          None,
//          Some(T),
//      }
// 
// "Option" is an enum, because its definition enumerates several variants that a value of
// this type could be: "For any type T, a value of type Option is either Some(v), where v
// is a value of type T; or None, indicating no value is available". In this case, "escape_time"
// returns an "Option" to indicate whether "c" is in the Mandelbrot set, and if it's not, how
// long we had to iterate to find that out (returning "Some(i)").