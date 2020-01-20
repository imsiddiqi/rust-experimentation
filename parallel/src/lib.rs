use num_traits::Float;
use rayon::prelude::*;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::vec::Vec;

pub trait FloatVector:
    Float + Default + AddAssign + DivAssign + MulAssign + SubAssign + Send + Sync
{}

impl<T> FloatVector for T where
    T: Float + Default + AddAssign + DivAssign + MulAssign + SubAssign + Send + Sync
{}

pub fn default<T: FloatVector>(v: &mut Vec<T>) {
    v.par_iter_mut().for_each(|e| *e = Default::default())
}

pub fn equal<T: FloatVector>(a: &Vec<T>, b: &Vec<T>) -> bool
{
    a.par_iter().zip(b).all(|(a, b)| *a == *b)
}

pub fn set<T: FloatVector>(v: &mut Vec<T>, s: T)
{
    v.par_iter_mut().for_each(|e| *e = s)
}

pub fn sc_add<T: FloatVector>(v: &mut Vec<T>, s: T)
{
    v.par_iter_mut().for_each(|e| *e += s)
}

pub fn sc_div<T: FloatVector>(v: &mut Vec<T>, s: T)
{
    v.par_iter_mut().for_each(|e| *e /= s)
}

pub fn sc_mul<T: FloatVector>(v: &mut Vec<T>, s: T)
{
    v.par_iter_mut().for_each(|e| *e *= s)
}

pub fn sc_sub<T: FloatVector>(v: &mut Vec<T>, s: T)
{
    v.par_iter_mut().for_each(|e| *e -= s)
}

pub fn vc_add<T: FloatVector>(a: &mut Vec<T>, b: &Vec<T>)
{
    a.par_iter_mut().zip(b).for_each(|(a, b)| *a += *b)
}

pub fn vc_div<T: FloatVector>(a: &mut Vec<T>, b: &Vec<T>)
{
    a.par_iter_mut().zip(b).for_each(|(a, b)| *a /= *b)
}

pub fn vc_mul<T: FloatVector>(a: &mut Vec<T>, b: &Vec<T>)
{
    a.par_iter_mut().zip(b).for_each(|(a, b)| *a *= *b)
}

pub fn vc_sub<T: FloatVector>(a: &mut Vec<T>, b: &Vec<T>)
{
    a.par_iter_mut().zip(b).for_each(|(a, b)| *a -= *b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn scalar_addition() {
        let mut a = vec![vec![1.0, 2.0, 3.0]; 3];
        let     b = vec![vec![2.0, 3.0, 4.0]; 3];

        for i in 0..a.len() {
            crate::sc_add(&mut a[i], 1.0);
        }

        assert_eq!(a, b);
    }

    #[test]
    fn vector_addition() {
        let mut a = vec![vec![1.0, 2.0, 3.0]; 3];
        let     b = vec![vec![1.0, 2.0, 3.0]; 3];
        let     c = vec![vec![2.0, 4.0, 6.0]; 3];

        for i in 0..a.len() {
            crate::vc_add(&mut a[i], &b[i]);
        }

        assert_eq!(a, c);
    }
}
