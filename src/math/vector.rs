//! Vector crate of math module in three.rs
//!

use std::cmp::Ord;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// TODO: switch to SIMD vector, reference (https://github.com/rust-lang/packed_simd)

#[derive(Debug, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

macro_rules! vector_default_impl {
    ($t:ident {$($field:ident), +}) => {

        impl<T> $t<T>
        where
            T: AddAssign
            + Add<Output = T>
            + MulAssign
            + Mul<Output = T>
            + DivAssign
            + Div<Output = T>
            + SubAssign
            + Sub<Output = T>
            + Ord
            + Clone
            + Copy
            + Default {

            #[inline]
            pub fn new($($field: T), +)-> $t<T> {
                $t { $($field), + }
            }

            pub fn set(&mut self, $($field: T), +) {
                $(self.$field = $field);+
            }

            pub fn set_scalar(&mut self, scalar: T) {
                $(self.$field = scalar);+
            }

            pub fn add(&mut self, v: &Self) {
                $(self.$field += v.$field);+
            }

            pub fn add_scalar(&mut self, scalar: T) {
                $(self.$field += scalar);+
            }

            pub fn add_vectors(&mut self, a: &Self, b: &Self) {
                $(self.$field = a.$field + b.$field);+
            }

            pub fn add_scaled_vectors(&mut self, v: &Self, s: T) {
                $(self.$field = v.$field * s);+
            }

            pub fn sub(&mut self, v: &Self) {
                $(self.$field -= v.$field);+
            }

            pub fn sub_scalar(&mut self, scalar: T) {
                $(self.$field -= scalar);+
            }

            pub fn sub_vectors(&mut self, a: &Self, b: &Self) {
                $(self.$field = a.$field - b.$field);+
            }

            pub fn multiply(&mut self, v: &Self) {
                $(self.$field *= v.$field);+
            }

            pub fn multiply_scalar(&mut self, scalar: T) {
                $(self.$field *= scalar);+
            }

            pub fn divide(&mut self, v: &Self) {
                $(self.$field /= v.$field);+
            }

            pub fn divide_scalar(&mut self, scalar: T) {
                $(self.$field /= scalar);+
            }

            pub fn min(&mut self, v: Self) {
                $(self.$field = T::min(self.$field, v.$field));+
            }

            pub fn max(&mut self, v: Self) {
                $(self.$field = T::max(self.$field, v.$field));+
            }

            pub fn clamp(&mut self, min: Self, max: Self) {
                $(self.$field = T::max(max.$field, T::min(self.$field, min.$field)));+
            }

            pub fn clamp_scalar(&mut self, min_val: T, max_val: T) {
                $(self.$field = T::max(max_val, T::min(self.$field, min_val)));+
            }

           // pub fn clamp_length(&mut self, min: T, max: T) {
           //     let length = self.length();
           //     self.divide_scalar(length);
           //     self.multiply_scalar(T::max(min, T::min(max, length)));
           // }

           // pub fn floor(&mut self) {
           //     $(self.$field = T::floor(self.$field));+
           // }

           // pub fn ceil(&mut self) {
           //     $(self.$field = T::ceil(self.$field));+
           // }

           // pub fn round(&mut self) {
           //     $(self.$field = T::round(self.$field));+
           // }

           // pub fn round_to_zero(&mut self) {
           //     $(self.$field = if self.$field < 0 { T::ceil(self.$field) } else { T::floor(self.$field)});+
           // }

           // pub fn length(&self) -> f32 {
           //     let mut square_length: T = T::default();
           //     $(square_length += self.$field * self.$field;)*
           //     f32::sqrt(square_length as f32)
           // }
        }

        // TODO: do we need to impl Symbol.iterator?
        // https://github.com/rustwasm/wasm-bindgen/issues/1036

        // TODO: if we need to implement indexing,
        // https://docs.rs/vector3d/0.2.1/src/vector3d/lib.rs.html#9-264
        // https://stackoverflow.com/questions/57203009/implementing-slice-for-custom-type
    }
}

vector_default_impl!(Vector2 { x, y });
vector_default_impl!(Vector3 { x, y, z });
vector_default_impl!(Vector4 { x, y, z, w });

impl<T> Vector2<T>
where
    T: Copy,
{
    pub fn set_component(&mut self, index: usize, value: T) -> Result<(), String> {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            _ => return Err(format!("index is out of range: {}", index)),
        }
        Ok(())
    }

    pub fn get_component(&self, index: usize) -> Result<T, String> {
        match index {
            0 => Ok(self.x),
            1 => Ok(self.y),
            _ => Err(format!("index is out of range: {}", index)),
        }
    }
}

impl<T> Vector3<T>
where
    T: Copy,
{
    pub fn set_component(&mut self, index: usize, value: T) -> Result<(), String> {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => return Err(format!("index is out of range: {}", index)),
        }
        Ok(())
    }

    pub fn get_component(&self, index: usize) -> Result<T, String> {
        match index {
            0 => Ok(self.x),
            1 => Ok(self.y),
            2 => Ok(self.z),
            _ => Err(format!("index is out of range: {}", index)),
        }
    }
}

impl<T> Vector4<T>
where
    T: Copy,
{
    pub fn set_component(&mut self, index: usize, value: T) -> Result<(), String> {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            3 => self.w = value,
            _ => return Err(format!("index is out of range: {}", index)),
        }
        Ok(())
    }

    pub fn get_component(&self, index: usize) -> Result<T, String> {
        match index {
            0 => Ok(self.x),
            1 => Ok(self.y),
            2 => Ok(self.z),
            3 => Ok(self.w),
            _ => Err(format!("index is out of range: {}", index)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
