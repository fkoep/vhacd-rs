#[macro_use]
extern crate cpp;
extern crate libc;

use std::{ptr, slice};
use libc::{c_int, c_uint};

cpp!{{
    #include "VHACD.h"
}}

pub enum ConvexHullList {}

impl ConvexHullList {
    pub fn len(&self) -> c_uint {
        let ptr = self;
        unsafe {
            cpp!{[ptr as "VHACD::IVHACD *"] -> c_uint as "unsigned int" {
                return ptr->GetNConvexHulls();
            }}
        }
    }
    pub fn points(&self, idx: c_uint) -> &[[f64; 3]] {
        let ptr = self;
        // FIXME
        // let ref mut data: *const [f64; 3] = ptr::null();
        // let ref mut len: c_uint = 0;
        let mut data: *const [f64; 3] = ptr::null();
        let mut len: c_uint = 0;
        let mut data = &mut data;
        let mut len = &mut len;

        unsafe {
            cpp!{[
                ptr as "VHACD::IVHACD *", 
                idx as "unsigned int", 
                mut data as "double **", 
                mut len as "unsigned int *"
            ]{
                VHACD::IVHACD::ConvexHull hull;
                ptr->GetConvexHull(idx, hull);
                *data = hull.m_points;
                *len = hull.m_nPoints;
            }}
            slice::from_raw_parts(*data, (*len) as usize)
        }
    }
    pub fn triangles(&self, idx: c_uint) -> &[[c_int; 3]] {
        let ptr = self;
        // FIXME
        // let ref mut data: *const [c_int; 3] = ptr::null();
        // let ref mut len: c_uint = 0;
        let mut data: *const [c_int; 3] = ptr::null();
        let mut len: c_uint = 0;
        let mut data = &mut data;
        let mut len = &mut len;

        unsafe {
            cpp!{[
                ptr as "VHACD::IVHACD *", 
                idx as "unsigned int", 
                mut data as "int **", 
                mut len as "unsigned int *"
            ]{
                VHACD::IVHACD::ConvexHull hull;
                ptr->GetConvexHull(idx, hull);
                *data = hull.m_triangles;
                *len = hull.m_nTriangles;
            }}
            slice::from_raw_parts(*data, (*len) as usize)
        }
    }
}

impl Drop for ConvexHullList {
    fn drop(&mut self) {
        let mut ptr = self;
        unsafe {
            cpp!{[mut ptr as "VHACD::IVHACD *"] {
                // TODO deconstructor is protected... is this the correct way to clean up?
                // delete ptr;

                ptr->Clean();
                ptr->Release();
            }}
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum Mode {
    VoxelBased = 0,
    TetrahedronBased = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HacdParams {
    pub concavity: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub min_volume_per_ch: f64,
    _callback: usize,
    _logger: usize,
    pub resolution: c_uint,
    pub max_num_vertices_per_ch: c_uint,
    pub depth: c_int,
    pub plane_downsampling: c_int,
    pub convexhull_downsampling: c_int,
    pub pca: c_int,
    pub mode: c_int,
    pub convexhull_approximation: c_int,
    pub ocl_acceleration: c_int,
}

impl Default for HacdParams {
    fn default() -> Self {
        unsafe {
            cpp!{[] -> HacdParams as "VHACD::IVHACD::Parameters" {
                return {};
            }}
        }
    }
}

pub fn compute_f32(
    points: &[[f32; 3]],
    triangles: &[[c_int; 3]],
    mut params: HacdParams,
) -> Box<ConvexHullList> {
    // Ensure these two are NULL
    params._callback = 0;
    params._logger = 0;

    let num_points = points.len() as c_uint;
    let num_triangles = triangles.len() as c_uint;
    let points = points.as_ptr();
    let triangles = triangles.as_ptr();

    unsafe {
        let ptr = cpp!{[
            points as "float *", 
            triangles as "int *", 
            num_points as "unsigned int",
            num_triangles as "unsigned int",
            params as "VHACD::IVHACD::Parameters"
        ] -> *mut ConvexHullList as "VHACD::IVHACD *" {
            auto hacd = VHACD::CreateVHACD();
            hacd->Compute(points, 0, num_points, triangles, 0, num_triangles, params);
            return hacd;
        }};
        Box::from_raw(ptr)
    }
}

pub fn compute_f64(
    points: &[[f64; 3]],
    triangles: &[[c_int; 3]],
    mut params: HacdParams,
) -> Box<ConvexHullList> {
    // Ensure these two are NULL
    params._callback = 0;
    params._logger = 0;

    let num_points = points.len() as c_uint;
    let num_triangles = triangles.len() as c_uint;
    let points = points.as_ptr();
    let triangles = triangles.as_ptr();

    unsafe {
        let ptr = cpp!{[
            points as "double *", 
            triangles as "int *", 
            num_points as "unsigned int",
            num_triangles as "unsigned int",
            params as "VHACD::IVHACD::Parameters"
        ] -> *mut ConvexHullList as "VHACD::IVHACD *" {
            auto hacd = VHACD::CreateVHACD();
            hacd->Compute(points, 0, num_points, triangles, 0, num_triangles, params);
            return hacd;
        }};
        Box::from_raw(ptr)
    }
}
