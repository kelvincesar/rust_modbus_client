/// It takes a slice of u16 values and returns a vector of u32 values
/// 
/// Arguments:
/// 
/// * `data`: &[u16] - the data to convert
/// 
/// Returns:
/// 
/// A vector of u32 values.
fn convert_u16_to_u32(data: &[u16]) -> Vec<u32> {
    let mut result = Vec::with_capacity(data.len() / 2);
    let mut i = 0;
    while i < data.len() {
        let value = (data[i] as u32) << 16 | data[i + 1] as u32;
        result.push(value);
        i += 2;
    }
    result
}

/// It takes a slice of u16 values, and returns a Vec of f32 values
/// 
/// Arguments:
/// 
/// * `data`: &[u16] - the data to convert
/// 
/// Returns:
/// 
/// A vector of f32 values.
fn convert_u16_to_float(data: &[u16]) -> Vec<f32> {
    let mut result = Vec::with_capacity(data.len() / 2);
    let mut i = 0;
    while i < data.len() {
        let value = (data[i] as u32) << 16 | data[i + 1] as u32;
        result.push(f32::from_bits(value));
        i += 2;
    }
    result
}

/// "Convert a slice of u16 values to a Vec of i16 values."
/// 
/// Arguments:
/// 
/// * `data`: &[u16] - This is the data we want to convert.
/// 
/// Returns:
/// 
/// A vector of i16 values.
fn convert_u16_to_i16(data: &[u16]) -> Vec<i16> {
    let mut result = Vec::with_capacity(data.len());
    for value in data {
        result.push(*value as i16);
    }
    result
}