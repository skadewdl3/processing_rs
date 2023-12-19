/*
    This function maps one value belonging to an input range to an output range
*/

#[no_mangle]
pub extern "C" fn map (
    value: f64,
    input_range_start: f64,
    input_range_end: f64,
    output_range_start: f64,
    output_range_end: f64
) -> f64 {

    
    // Input rang cannot be 0
    if input_range_start == input_range_end {
        panic!("Input range start and end cannot be the same");
    }

    // Output range cannot be 0
    if output_range_start == output_range_end {
        panic!("Output range start and end cannot be the same");
    }

    // Example: start = -1, end = -5. Value = -4 is valid. Value = 0 or -6 is invalid.
    if input_range_start > input_range_end {
        if value > input_range_start {
            panic!("Value cannot be greater than input range start");
        }
        if value < input_range_end {
            panic!("Value cannot be less than input range end");
        }
    }

    // Example: start = 1, end = 5. Value = 4 is valid. Value = 0 or 6 is invalid.
    else if input_range_start < input_range_end {
        if value < input_range_start {
            panic!("Value cannot be less than input range start");
        }
        if value > input_range_end {
            panic!("Value cannot be greater than input range end");
        }
    }

    // get what percentage of the value is from input range
    let percentage = (value - input_range_start) / (input_range_end - input_range_start);
    // get the value in the output range
    let output = percentage * (output_range_end - output_range_start) + output_range_start;
    // return the output
    output
}