#[macro_use]
extern crate cpp;
extern crate libc; // TODO remove?

use std::{ptr, slice};

cpp!{{
    #include "VHACD.h"
}}

pub struct ConvexHull<'a> {
    pub points: &'a [[f64; 3]],
    pub triangles: &'a [[u32; 3]],
    pub volume: f64,
    pub center: [f64; 3],
}

pub enum ConvexHullList {}

impl ConvexHullList {
    pub fn len(&self) -> u32 {
        let ptr = self;
        unsafe {
            cpp!{[ptr as "VHACD::IVHACD *"] -> u32 as "uint32_t" {
                return ptr->GetNConvexHulls();
            }}
        }
    }
    pub fn get(&self, idx: u32) -> ConvexHull {
        assert!(idx < self.len());

        let ptr = self;
        let mut points: *const [f64; 3] = ptr::null();
        let mut num_points: u32 = 0;
        let mut triangles: *const [u32; 3] = ptr::null();
        let mut num_triangles: u32 = 0;
        let mut volume: f64 = 0.0;
        let mut center: [f64; 3] = [0.0; 3];

        unsafe {
            let mut points = &mut points;
            let mut num_points = &mut num_points;
            let mut triangles = &mut triangles;
            let mut num_triangles = &mut num_triangles;
            let mut volume = &mut volume;
            let mut center = &mut center;

            cpp!{[
                ptr as "VHACD::IVHACD *", 
                idx as "uint32_t", 
                mut points as "double **", 
                mut num_points as "uint32_t *",
                mut triangles as "uint32_t **", 
                mut num_triangles as "uint32_t *",
                mut volume as "double *",
                mut center as "double *"
            ]{
                VHACD::IVHACD::ConvexHull hull;
                ptr->GetConvexHull(idx, hull);
                *points = hull.m_points;
                *num_points = hull.m_nPoints;
                *triangles = hull.m_triangles;
                *num_triangles = hull.m_nTriangles;
                *volume = hull.m_volume;
                center[0] = hull.m_center[0];
                center[1] = hull.m_center[1];
                center[2] = hull.m_center[2];
            }}

            return ConvexHull{
                points: slice::from_raw_parts(*points, (*num_points) as usize),
                triangles: slice::from_raw_parts(*triangles, (*num_triangles) as usize),
                volume: *volume,
                center: *center,
            }
        }
    }
}

impl Drop for ConvexHullList {
    fn drop(&mut self) {
        let mut ptr = self;
        unsafe {
            cpp!{[mut ptr as "VHACD::IVHACD *"] {
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
    pub min_volume_per_ch: f64,
    _callback: usize,
    _logger: usize,
    pub resolution: u32,
    pub max_num_vertices_per_ch: u32,
    pub plane_downsampling: u32,
    pub convexhull_downsampling: u32,
    pub pca: u32,
    pub mode: u32,
    pub convexhull_approximation: u32,
    pub ocl_acceleration: u32,
    pub max_convex_hulls: u32,
    pub project_hull_vertices: bool, // TODO bool binary compatible with c_bool?
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
    triangles: &[[u32; 3]],
    mut params: HacdParams,
) -> Box<ConvexHullList> {
    // Ensure these two are NULL
    params._callback = 0;
    params._logger = 0;

    let num_points = points.len() as u32;
    let num_triangles = triangles.len() as u32;
    let points = points.as_ptr();
    let triangles = triangles.as_ptr();

    unsafe {
        let ptr = cpp!{[
            points as "float *", 
            num_points as "uint32_t",
            triangles as "uint32_t *", 
            num_triangles as "uint32_t",
            params as "VHACD::IVHACD::Parameters"
        ] -> *mut ConvexHullList as "VHACD::IVHACD *" {
            auto hacd = VHACD::CreateVHACD();
            hacd->Compute(points, num_points, triangles, num_triangles, params);
            return hacd;
        }};
        Box::from_raw(ptr)
    }
}

pub fn compute_f64(
    points: &[[f64; 3]],
    triangles: &[[u32; 3]],
    mut params: HacdParams,
) -> Box<ConvexHullList> {
    // Ensure these two are NULL
    params._callback = 0;
    params._logger = 0;

    let num_points = points.len() as u32;
    let num_triangles = triangles.len() as u32;
    let points = points.as_ptr();
    let triangles = triangles.as_ptr();

    unsafe {
        let ptr = cpp!{[
            points as "double *", 
            num_points as "uint32_t",
            triangles as "uint32_t *", 
            num_triangles as "uint32_t",
            params as "VHACD::IVHACD::Parameters"
        ] -> *mut ConvexHullList as "VHACD::IVHACD *" {
            auto hacd = VHACD::CreateVHACD();
            hacd->Compute(points, num_points, triangles, num_triangles, params);
            return hacd;
        }};
        Box::from_raw(ptr)
    }
}
