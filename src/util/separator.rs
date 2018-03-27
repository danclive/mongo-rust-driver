macro_rules! separated_uint_with_output {
    ($string:expr, $output:expr) => {{
        let mut place = $string.len();
        let mut later_loop = false;

        for ch in $string.chars() {
            if later_loop && place % 3 == 0 {
                $output.push(',');
            }

            $output.push(ch);
            later_loop = true;
            place -= 1;
        };

        $output
    }};
}

macro_rules! separated_uint {
    ($string:expr) => {{
        let mut output = String::new();
        separated_uint_with_output!($string, output)
    }}
}

macro_rules! separated_int {
    ($string:expr) => {{
        let mut output = String::new();
        let magnitude = if $string.starts_with('-') {
            output.push('-');
            (&$string)[1..].to_string()
        } else {
            $string
        };

        separated_uint_with_output!(magnitude, output)
    }};
}

macro_rules! separated_float {
    ($string:expr) => {{
        let idx = match $string.find('.') {
            Some(i) => i,
            None => $string.len()
        };

        let int_part = (&$string[..idx]).to_string();
        let fract_part = &$string[idx..];
        let output = separated_int!(int_part);
        output + fract_part
    }};
}

macro_rules! separated_float2 {
    ($string:expr) => {{
        let idx = match $string.find('.') {
            Some(i) => i,
            None => $string.len()
        };

        let int_part = &$string[..idx];
        let fract_part = &$string[idx..];

        let mut output = String::new();
        let magnitude = if int_part.starts_with('-') {
            output.push('-');
            int_part[1..].to_string()
        } else {
            int_part.to_string()
        };

        let mut place = magnitude.len();
        let mut later_loop = false;

        for ch in magnitude.chars() {
            if later_loop && place % 3 == 0 {
                output.push(',');
            }

            output.push(ch);
            later_loop = true;
            place -= 1;
        };

        output.push_str(fract_part);
        output
    }};
}

pub trait Separatable {

    /// Converts the number to a string with thousands separator.
    fn separated_string(&self) -> String;
}

impl Separatable for i16 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_int!(string)
    }
}

impl Separatable for i32 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_int!(string)
    }
}

impl Separatable for i64 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_int!(string)
    }
}

impl Separatable for isize {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_int!(string)
    }
}

impl Separatable for u16 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_uint!(string)
    }
}

impl Separatable for u32 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_uint!(string)
    }
}

impl Separatable for u64 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_uint!(string)
    }
}

impl Separatable for usize {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_uint!(string)
    }
}

impl Separatable for f32 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_float!(string)
    }
}

impl Separatable for f64 {
    fn separated_string(&self) -> String {
        let string = format!("{}", self);
        separated_float!(string)
    }
}

pub trait FixedPlaceSeparatable {
    fn separated_string_with_fixed_place(&self, places: usize) -> String;
}

impl FixedPlaceSeparatable for f32 {
    fn separated_string_with_fixed_place(&self, places: usize) -> String {
        let string = format!("{:.*}", places, self);
        separated_float2!(string)
    }
}

impl FixedPlaceSeparatable for f64 {
    fn separated_string_with_fixed_place(&self, places: usize) -> String {
        let string = format!("{:.*}", places, self);
        separated_float2!(string)
    }
}
