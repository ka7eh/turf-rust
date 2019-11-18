use helpers::point;
use projection::{to_mercator, to_wgs84};

pub fn buffer() {
    let center = vec![
        6.83471679687499,
        85.022614510813854
    ];
    let radius = 100000.0;
    let steps = 128;
    let steps_querter = steps as f64 / 4.0;
    let mut output = vec![(0.0, 0.0); steps];
    let (c_x, c_y) = to_mercator(&center);
    for i in 0..steps/4 {
        let d_x = radius * i as f64 / steps_querter;
        let d_y = (radius.powi(2) - d_x.powi(2)).sqrt();
        output[i] = to_wgs84(&vec![c_x + d_x, c_y + d_y]);
        output[i + steps_querter as usize] = to_wgs84(&vec![c_x + d_y, c_y - d_x]);
        output[i + 2 * steps_querter as usize] = to_wgs84(&vec![c_x - d_x, c_y - d_y]);
        output[i + 3 * steps_querter as usize] = to_wgs84(&vec![c_x - d_y, c_y + d_x]);
        println!(
            "{:.1}\t{:.1}\t{}\t{}\t{}\t{}",
            d_x, d_y,
            i,
            i + steps_querter as usize,
            i + 2 * steps_querter as usize,
            i + 3 * steps_querter as usize
        );
    }
    output.push(output[0]);
    println!("\n{:?}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        buffer();
    }
}