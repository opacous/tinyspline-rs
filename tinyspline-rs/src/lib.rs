use std::marker::PhantomData;
use std::slice::from_raw_parts;
use {std::ffi::c_short, tinyspline_ffi::*};

pub struct Error {
    code: tsError,
    message: String,
}
pub type Status = tsStatus;

pub type SplineResult<T> = Result<T, Error>;

fn read_err(status: &tsStatus, err: tsError) -> SplineResult<()> {
    todo!()
}

pub struct Vec2([f64;2]);
pub struct Vec3([f64;3]);
pub struct Vec4([f64;4]);

pub struct VecN(Box<[f64]>);

pub struct Point<const dim: usize>([f64;dim]);


// #[derive(Debug, Copy, Clone)]
// pub struct tsFrame {
//     pub position: [f64; 3usize],
//     pub tangent: [f64; 3usize],
//     pub normal: [f64; 3usize],
//     pub binormal: [f64; 3usize],
// }

#[derive(Debug, Copy, Clone)]
pub struct BSpline<const dim: usize> {
    val: tsBSpline,
    status: tsStatus,
    _phantom: PhantomData<Point<dim>>
}

impl<const dim: usize> BSpline<dim> {
    pub fn degree(&self) -> usize {
        unsafe { bspline_degree(&self.val) }
    }
    pub fn order(&self) -> usize {
        unsafe { bspline_order(&self.val) }
    }
    pub fn dimension(&self) -> usize {
        unsafe { bspline_dimension(&self.val) }
    }
    pub fn len_control_points(&self) -> usize {
        unsafe { bspline_len_control_points(&self.val) }
    }
    pub fn num_control_points(&self) -> usize {
        unsafe { bspline_num_control_points(&self.val) }
    }
    pub fn sof_control_points(&self) -> usize {
        unsafe { bspline_sof_control_points(&self.val) }
    }
    pub fn bspline_control_points_ptr<'a>(&'a self) -> &'a [Point<dim>] {
        unsafe {
            let len = self.num_control_points();
            let ptr = bspline_control_points_ptr(&self.val);

            from_raw_parts(ptr as *const Point<dim>, len)
        }
    }
    //
    // pub fn bspline_control_points(
    //     &self,
    //     ctrlp: *mut *mut f64,
    // ) -> SplineResult<()>{
    //     bspline_control_points(
    //         &self.val,
    //         ctrlp: *mut *mut f64,
    //         &mut self.status,
    //     )
    // }

    // pub fn bspline_control_point_at_ptr(
    //     &self,
    //     index: usize,
    //     ctrlp: *mut *const f64,
    // ) -> SplineResult<()>{
    //     bspline_control_point_at_ptr(
    //         &self.val,
    //         index: usize,
    //         ctrlp: *mut *const f64,
    //         &mut self.status,
    //     )
    // }
    // pub fn bspline_set_control_points(
    //     &mut self,
    //     ctrlp: *const f64,
    // ) -> SplineResult<()>{
    //     bspline_set_control_points(
    //         &self.val,
    //         ctrlp: *const f64,
    //         &mut self.status,
    //     )
    // }

    // pub fn bspline_set_control_point_at(
    //     &mut self,
    //     index: usize,
    //     ctrlp: *const f64,
    // ) -> SplineResult<()>{
    //     bspline_set_control_point_at(
    //         &self.val,
    //         index: usize,
    //         ctrlp: *const f64,
    //         &mut self.status,
    //     )
    // }
    pub fn bspline_num_knots(&self) -> usize {
        unsafe { bspline_num_knots(&self.val) }
    }
    pub fn bspline_sof_knots(&self) -> usize {
        unsafe { bspline_sof_knots(&self.val) }
    }

    pub fn bspline_knots_ptr(&self) -> *const f64 {
        unsafe { bspline_knots_ptr(&self.val) }
    }

    // pub fn bspline_knots(
    //     &self,
    //     knots: *mut *mut f64
    // ) -> SplineResult<()>{
    //     bspline_knots(
    //         &self.val,
    //         knots: *mut *mut f64,
    //         &mut self.status,
    //     )
    // }

    pub fn bspline_knot_at(&mut self, index: usize, knot: *mut f64) -> SplineResult<()> {
        unsafe {
            let err = bspline_knot_at(&self.val, index, knot, &mut self.status);
            read_err(&self.status, err)
        }
    }

    pub fn bspline_set_knots(&mut self, knots: *const f64) -> SplineResult<()> {
        unsafe {
            let err = bspline_set_knots(&mut self.val, knots, &mut self.status);
            read_err(&self.status, err)
        }
    }
    // pub fn bspline_set_knots_varargs(
    //     &mut self,
    //     knots: &[f64]
    // ) -> SplineResult<()>{
    //     bspline_set_knots_varargs(
    //         &mut self.val,
    //         &mut self.status,
    //         knot0: f64,
    //         knot1: f64,
    //         ...
    //     )
    // }

    pub fn bspline_set_knot_at(&mut self, index: usize, knot: f64) -> SplineResult<()> {
        unsafe {
            let err = bspline_set_knot_at(&mut self.val, index, knot, &mut self.status);

            read_err(&self.status, err)
        }
    }

    pub fn bspline_init() -> tsBSpline {
        unsafe { bspline_init() }
    }

    pub fn bspline_new(
        &mut self,
        num_control_points: usize,
        dimension: usize,
        degree: usize,
        spline_type: tsBSplineType,
    ) -> SplineResult<()> {
        unsafe {
            let err = bspline_new(
                num_control_points,
                dimension,
                degree,
                spline_type,
                &mut self.val,
                &mut self.status,
            );
            read_err(&self.status, err)
        }
    }

    // pub fn bspline_new_with_control_points(
    //     &mut self,
    //     num_control_points: usize,
    //     dimension: usize,
    //     degree: usize,
    //     type_: BSplineType,
    //     first: f64,
    //     ...
    // ) -> SplineResult<()>{
    //     bspline_new_with_control_points(
    //         num_control_points: usize,
    //         dimension: usize,
    //         degree: usize,
    //         type_: BSplineType,
    //         &mut self.val,
    //         &mut self.status,
    //         first: f64,
    //         ...
    //     )
    // }

    pub fn bspline_copy(&mut self, dest: &mut Self) -> SplineResult<()> {
        unsafe {
            let err = bspline_copy(&self.val, &mut dest.val, &mut self.status);
            read_err(&self.status, err)
        }
    }

    pub fn bspline_move(&mut self, dest: &mut Self) {
        unsafe { bspline_move(&mut self.val, &mut dest.val) }
    }

    pub fn bspline_free(&mut self) {
        unsafe { bspline_free(&mut self.val) }
    }
    // pub fn bspline_interpolate_cubic_natural(
    //     &mut self,
    //     points: *const f64,
    //     num_points: usize,
    //     dimension: usize,
    // ) -> SplineResult<()>{
    //     bspline_interpolate_cubic_natural(
    //         points: *const f64,
    //         num_points: usize,
    //         dimension: usize,
    //         &mut self.val,
    //         &mut self.status,
    //     )
    // }
    // pub fn bspline_interpolate_catmull_rom(
    //     &mut self,
    //     points: *const f64,
    //     num_points: usize,
    //     dimension: usize,
    //     alpha: f64,
    //     first: *const f64,
    //     last: *const f64,
    //     epsilon: f64,
    // ) -> SplineResult<()>{
    //     bspline_interpolate_catmull_rom(
    //         points: *const f64,
    //         num_points: usize,
    //         dimension: usize,
    //         alpha: f64,
    //         first: *const f64,
    //         last: *const f64,
    //         epsilon: f64,
    //         &mut self.val,
    //         &mut self.status,
    //     )
    // }
    // pub fn bspline_eval(
    //     &self,
    //     knot: f64,
    //     net: *mut tsDeBoorNet,
    // ) -> SplineResult<()>{
    //     bspline_eval(
    //         &self.val,
    //         knot: f64,
    //         net: *mut tsDeBoorNet,
    //         &mut self.status
    //     )
    // }

    // pub fn bspline_eval_all(
    //     &self,
    //     knots: *const f64,
    //     num: usize,
    //     points: *mut *mut f64,
    // ) -> SplineResult<()>{
    //     bspline_eval_all(
    //         &self.val,
    //         knots: *const f64,
    //         num: usize,
    //         points: *mut *mut f64,
    //         &mut self.status,
    //     )
    // }
    // pub fn bspline_sample(
    //     &self,
    //     num: usize,
    //     points: *mut *mut f64,
    //     actual_num: *mut usize,
    // ) -> SplineResult<()>{
    //     bspline_sample(
    //         &self.val,
    //         num: usize,
    //         points: *mut *mut f64,
    //         actual_num: *mut usize,
    //         &mut self.status,
    //     )
    // }
    pub fn bspline_bisect(
        &mut self,
        value: f64,
        epsilon: f64,
        persnickety: ::std::os::raw::c_int,
        index: usize,
        ascending: ::std::os::raw::c_int,
        max_iter: usize,
        net: &mut DeBoorNet,
    ) -> SplineResult<()> {
        unsafe {
            let err = bspline_bisect(
                &self.val,
                value,
                epsilon,
                persnickety,
                index,
                ascending,
                max_iter,
                &mut net.val,
                &mut self.status,
            );
            read_err(&self.status, err)
        }
    }
    // pub fn bspline_domain(&self, min: *mut f64, max: *mut f64){
    //     bspline_domain(&self.val, min: *mut f64, max: *mut f64)
    // }

    // pub fn bspline_is_closed(
    //     &self,
    //     epsilon: f64,
    //     closed: *mut ::std::os::raw::c_int,
    // ) -> SplineResult<()>{
    //     bspline_is_closed(
    //         &self.val,
    //         epsilon,
    //         closed: *mut ::std::os::raw::c_int,
    //         &mut self.status,
    //     )
    // }
    // pub fn bspline_compute_rmf(
    //     &self,
    //     knots: *const f64,
    //     num: usize,
    //     has_first_normal: ::std::os::raw::c_int,
    //     frames: *mut tsFrame,
    // ) -> SplineResult<()> {
    //     bspline_compute_rmf(
    //         &self.val,
    //         knots: *const f64,
    //         num: usize,
    //         has_first_normal: ::std::os::raw::c_int,
    //         frames: *mut tsFrame,
    //         &mut self.status,
    //     )
    // }
    //
    // pub fn bspline_chord_lengths(
    //     &self,
    //     knots: *const f64,
    //     num: usize,
    //     lengths: *mut f64,
    // ) -> SplineResult<()>{
    //     bspline_chord_lengths(
    //         &self.val,
    //         knots: *const f64,
    //         num: usize,
    //         lengths: *mut f64,
    //         &mut self.status,
    //     )
    // }
    pub fn bspline_sub_spline(&mut self, knot0: f64, knot1: f64, other: &Self) -> SplineResult<()> {
        unsafe {
            let err = bspline_sub_spline(&other.val, knot0, knot1, &mut self.val, &mut self.status);
            read_err(&self.status, err)
        }
    }

    // pub fn bspline_uniform_knot_seq(&self, num: usize, knots: *mut f64){
    //     bspline_uniform_knot_seq(&self, num: usize, knots: *mut f64)
    // }

    // pub fn bspline_equidistant_knot_seq(
    //     &self,
    //     num: usize,
    //     knots: *mut f64,
    //     num_samples: usize,
    // ) -> SplineResult<()>{
    //     bspline_equidistant_knot_seq(
    //         &self.val,
    //         num: usize,
    //         knots: *mut f64,
    //         num_samples: usize,
    //         &mut self.status,
    //     )
    // }

    pub fn bspline_derive(&mut self, n: usize, epsilon: f64, src: &Self) -> SplineResult<()> {
        unsafe {
            let err = bspline_derive(&src.val, n, epsilon, &mut self.val, &mut self.status);
            read_err(&self.status, err)
        }
    }
    // pub fn bspline_insert_knot(
    //     &mut self,
    //     knot: f64,
    //     num: usize,
    //     src: &Self,
    //     k: *mut usize,
    // ) -> SplineResult<()>{
    //     bspline_insert_knot(
    //         &src.val,
    //         knot,
    //         num,
    //         &mut self.val,
    //         k: *mut usize,
    //         &mut self.status,
    //     )
    //
    // }
    // pub fn bspline_split(
    //     &mut self,
    //     knot: f64,
    //     other: &mut self,
    //     k: *mut usize,
    // ) -> SplineResult<()>{
    //     bspline_split(
    //         &other.val,
    //         knot: f64,
    //         &mut self,
    //         k: *mut usize,
    //         &mut self.status,
    //     )
    // }

    pub fn bspline_tension(&mut self, beta: f64, input: &Self) -> SplineResult<()> {
        unsafe {
            let err = bspline_tension(&input.val, beta, &mut self.val, &mut self.status);
            read_err(&self.status, err)
        }
    }
    pub fn bspline_to_beziers(&mut self, input: &Self) -> SplineResult<()> {
        unsafe {
            let err = bspline_to_beziers(&input.val, &mut self.val, &mut self.status);
            read_err(&self.status, err)
        }
    }
    pub fn bspline_elevate_degree(
        &mut self,
        amount: usize,
        epsilon: f64,
        input: &Self,
    ) -> SplineResult<()> {
        unsafe {
            let err = bspline_elevate_degree(
                &input.val,
                amount,
                epsilon,
                &mut self.val,
                &mut self.status,
            );
            read_err(&self.status, err)
        }
    }

    pub fn bspline_morph(
        &mut self,
        origin: &Self,
        target: &Self,
        t: f64,
        epsilon: f64,
    ) -> SplineResult<()> {
        unsafe {
            let err = bspline_morph(
                &origin.val,
                &target.val,
                t,
                epsilon,
                &mut self.val,
                &mut self.status,
            );
            read_err(&self.status, err)
        }
    }
    pub fn bspline_to_json(&mut self, json: *mut *mut ::std::os::raw::c_char) -> SplineResult<()> {
        unsafe {
            let err = bspline_to_json(&self.val, json, &mut self.status);
            read_err(&self.status, err)
        }
    }
    pub fn bspline_parse_json(&mut self, json: *const ::std::os::raw::c_char) -> SplineResult<()> {
        unsafe {
            let err = bspline_parse_json(json, &mut self.val, &mut self.status);
            read_err(&self.status, err)
        }
    }
    pub fn bspline_save(&mut self, path: *const ::std::os::raw::c_char) -> SplineResult<()> {
        unsafe {
            let err = bspline_save(&self.val, path, &mut self.status);
            read_err(&self.status, err)
        }
    }
    pub fn bspline_load(&mut self, path: *const ::std::os::raw::c_char) -> SplineResult<()> {
        unsafe {
            let err = bspline_load(path, &mut self.val, &mut self.status);
            read_err(&self.status, err)
        }
    }
}

pub fn bspline_align<const dim: usize>(
    s1: &BSpline<dim>,
    s2: &BSpline<dim>,
    epsilon: f64,
    s1_out: &mut BSpline<dim>,
    s2_out: &mut BSpline<dim>,
) -> SplineResult<()> {
    let mut status = tsStatus {
        code: tsError::TsSuccess,
        message: [0; 100],
    };
    unsafe {
        let err = tinyspline_ffi::bspline_align(
            &s1.val,
            &s2.val,
            epsilon,
            &mut s1_out.val,
            &mut s2_out.val,
            &mut status,
        );
        read_err(&status, err)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DeBoorNet {
    val: tsDeBoorNet,
    status: tsStatus,
}

impl DeBoorNet {
    pub fn deboornet_knot(&self) -> f64 {
        unsafe { deboornet_knot(&self.val) }
    }

    pub fn deboornet_index(&self) -> usize {
        unsafe { deboornet_index(&self.val) }
    }

    pub fn deboornet_multiplicity(&self) -> usize {
        unsafe { deboornet_multiplicity(&self.val) }
    }

    pub fn deboornet_num_insertions(&self) -> usize {
        unsafe { deboornet_num_insertions(&self.val) }
    }
    pub fn deboornet_dimension(&self) -> usize {
        unsafe { deboornet_dimension(&self.val) }
    }

    pub fn deboornet_len_points(&self) -> usize {
        unsafe { deboornet_len_points(&self.val) }
    }
    pub fn deboornet_num_points(&self) -> usize {
        unsafe { deboornet_num_points(&self.val) }
    }
    pub fn deboornet_sof_points(&self) -> usize {
        unsafe { deboornet_sof_points(&self.val) }
    }
    pub fn deboornet_points_ptr(&self) -> *const f64 {
        unsafe { deboornet_points_ptr(&self.val) }
    }
    pub fn deboornet_points(&mut self, points: *mut *mut f64) -> SplineResult<()> {
        unsafe {
            let err = deboornet_points(&self.val, points, &mut self.status);
            read_err(&self.status, err)
        }
    }
    pub fn deboornet_len_result(&self) -> usize {
        unsafe { deboornet_len_result(&self.val) }
    }
    pub fn deboornet_num_result(&self) -> usize {
        unsafe { deboornet_num_result(&self.val) }
    }
    pub fn deboornet_sof_result(&self) -> usize {
        unsafe { deboornet_sof_result(&self.val) }
    }
    pub fn deboornet_result_ptr(&self) -> *const f64 {
        unsafe { deboornet_result_ptr(&self.val) }
    }
    pub fn deboornet_result(&mut self, result: *mut *mut f64) -> SplineResult<()> {
        unsafe {
            let err = deboornet_result(&self.val, result, &mut self.status);
            read_err(&self.status, err)
        }
    }
    pub fn deboornet_init() -> tsDeBoorNet {
        unsafe { deboornet_init() }
    }
    pub fn deboornet_copy(&mut self, src: &Self) -> SplineResult<()> {
        unsafe {
            let err = deboornet_copy(&src.val, &mut self.val, &mut self.status);
            read_err(&self.status, err)
        }
    }
    pub fn deboornet_move(&mut self, dest: &mut Self) {
        unsafe { deboornet_move(&mut self.val, &mut dest.val) }
    }
    pub fn deboornet_free(&mut self) {
        unsafe { deboornet_free(&mut self.val) }
    }
}
//
// pub fn chord_lengths_length_to_knot(
//     knots: *const f64,
//     lengths: *const f64,
//     num: usize,
//     len: f64,
//     knot: *mut f64,
//     status: *mut tsStatus,
// ) -> SplineResult<()> {
//     chord_lengths_length_to_knot(
//         knots: *const f64,
//         lengths: *const f64,
//         num: usize,
//         len: f64,
//         knot: *mut f64,
//         status: *mut tsStatus,
//     )
// }
// pub fn chord_lengths_t_to_knot(
//     knots: *const f64,
//     lengths: *const f64,
//     num: usize,
//     t: f64,
//     knot: *mut f64,
//     status: *mut tsStatus,
// ) -> SplineResult<()> {
//     chord_lengths_t_to_knot(
//         knots: *const f64,
//         lengths: *const f64,
//         num: usize,
//         t: f64,
//         knot: *mut f64,
//         status: *mut tsStatus,
//     )
// }
//
// pub fn chord_lengths_equidistant_knot_seq(
//     knots: *const f64,
//     lengths: *const f64,
//     num: usize,
//     num_knot_seq: usize,
//     knot_seq: *mut f64,
//     status: *mut tsStatus,
// ) -> SplineResult<()> {
//     chord_lengths_equidistant_knot_seq(
//         knots: *const f64,
//         lengths: *const f64,
//         num: usize,
//         num_knot_seq: usize,
//         knot_seq: *mut f64,
//         status: *mut tsStatus,
//     )
// }
//
//
// pub fn knots_equal(x: f64, y: f64) -> ::std::os::raw::c_int;
// pub fn arr_fill(arr: *mut f64, num: usize, val: f64);
// pub fn distance(x: *const f64, y: *const f64, dim: usize) -> f64;
