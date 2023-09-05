/* automatically generated by rust-bindgen 0.65.1 */

pub const TS_PI: f64 = 3.141592653589793;
pub const TS_MAX_NUM_KNOTS: u32 = 10000;
pub const TS_DOMAIN_DEFAULT_MIN: f64 = 0.0;
pub const TS_DOMAIN_DEFAULT_MAX: f64 = 1.0;
pub const TS_KNOT_EPSILON: f64 = 0.0001;
pub const TS_POINT_EPSILON: f64 = 0.00001;
pub const TS_LENGTH_ZERO: f64 = 0.0001;

pub type wchar_t = ::std::os::raw::c_int;


pub type tsReal = f64;


type TinysplineError = tsError;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tsStatus {
    
    pub code: tsError,
    
    pub message: [::std::os::raw::c_char; 100usize],
}
#[test]
fn bindgen_test_layout_tsStatus() {
    const UNINIT: ::std::mem::MaybeUninit<tsStatus> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tsStatus>(),
        104usize,
        concat!("Size of: ", stringify!(tsStatus))
    );
    assert_eq!(
        ::std::mem::align_of::<tsStatus>(),
        4usize,
        concat!("Alignment of ", stringify!(tsStatus))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).code) as usize - ptr as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tsStatus),
        "::",
        stringify!(code)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).message) as usize - ptr as usize },
        4usize,
        concat!(
        "Offset of field: ",
        stringify!(tsStatus),
        "::",
        stringify!(message)
        )
    );
}
#[repr(u32)]

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum tsBSplineType {
    
    TS_OPENED = 0,
    
    TS_CLAMPED = 1,
    
    TS_BEZIERS = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tsFrame {
    
    pub position: [tsReal; 3usize],
    
    pub tangent: [tsReal; 3usize],
    
    pub normal: [tsReal; 3usize],
    
    pub binormal: [tsReal; 3usize],
}
#[test]
fn bindgen_test_layout_tsFrame() {
    const UNINIT: ::std::mem::MaybeUninit<tsFrame> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tsFrame>(),
        96usize,
        concat!("Size of: ", stringify!(tsFrame))
    );
    assert_eq!(
        ::std::mem::align_of::<tsFrame>(),
        8usize,
        concat!("Alignment of ", stringify!(tsFrame))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).position) as usize - ptr as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tsFrame),
        "::",
        stringify!(position)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tangent) as usize - ptr as usize },
        24usize,
        concat!(
        "Offset of field: ",
        stringify!(tsFrame),
        "::",
        stringify!(tangent)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).normal) as usize - ptr as usize },
        48usize,
        concat!(
        "Offset of field: ",
        stringify!(tsFrame),
        "::",
        stringify!(normal)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).binormal) as usize - ptr as usize },
        72usize,
        concat!(
        "Offset of field: ",
        stringify!(tsFrame),
        "::",
        stringify!(binormal)
        )
    );
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tsBSpline {
    
    pub pImpl: *mut tsBSplineImpl,
}
#[test]
fn bindgen_test_layout_tsBSpline() {
    const UNINIT: ::std::mem::MaybeUninit<tsBSpline> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tsBSpline>(),
        8usize,
        concat!("Size of: ", stringify!(tsBSpline))
    );
    assert_eq!(
        ::std::mem::align_of::<tsBSpline>(),
        8usize,
        concat!("Alignment of ", stringify!(tsBSpline))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pImpl) as usize - ptr as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tsBSpline),
        "::",
        stringify!(pImpl)
        )
    );
}
extern "C" {
    
    pub fn ts_bspline_degree(spline: *const tsBSpline) -> usize;
}
extern "C" {
    
    pub fn ts_bspline_order(spline: *const tsBSpline) -> usize;
}
extern "C" {
    
    pub fn ts_bspline_dimension(spline: *const tsBSpline) -> usize;
}
extern "C" {
    
    pub fn ts_bspline_len_control_points(spline: *const tsBSpline) -> usize;
}
extern "C" {
    
    pub fn ts_bspline_num_control_points(spline: *const tsBSpline) -> usize;
}
extern "C" {
    
    pub fn ts_bspline_sof_control_points(spline: *const tsBSpline) -> usize;
}
extern "C" {
    
    pub fn ts_bspline_control_points_ptr(spline: *const tsBSpline) -> *const tsReal;
}
extern "C" {
    
    pub fn ts_bspline_control_points(
        spline: *const tsBSpline,
        ctrlp: *mut *mut tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_control_point_at_ptr(
        spline: *const tsBSpline,
        index: usize,
        ctrlp: *mut *const tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_set_control_points(
        spline: *mut tsBSpline,
        ctrlp: *const tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_set_control_point_at(
        spline: *mut tsBSpline,
        index: usize,
        ctrlp: *const tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_num_knots(spline: *const tsBSpline) -> usize;
}
extern "C" {
    
    pub fn ts_bspline_sof_knots(spline: *const tsBSpline) -> usize;
}
extern "C" {
    
    pub fn ts_bspline_knots_ptr(spline: *const tsBSpline) -> *const tsReal;
}
extern "C" {
    
    pub fn ts_bspline_knots(
        spline: *const tsBSpline,
        knots: *mut *mut tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_knot_at(
        spline: *const tsBSpline,
        index: usize,
        knot: *mut tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_set_knots(
        spline: *mut tsBSpline,
        knots: *const tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_set_knots_varargs(
        spline: *mut tsBSpline,
        status: *mut tsStatus,
        knot0: tsReal,
        knot1: f64,
        ...
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_set_knot_at(
        spline: *mut tsBSpline,
        index: usize,
        knot: tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_init() -> tsBSpline;
}
extern "C" {
    
    pub fn ts_bspline_new(
        num_control_points: usize,
        dimension: usize,
        degree: usize,
        type_: tsBSplineType,
        spline: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_new_with_control_points(
        num_control_points: usize,
        dimension: usize,
        degree: usize,
        type_: tsBSplineType,
        spline: *mut tsBSpline,
        status: *mut tsStatus,
        first: f64,
        ...
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_copy(
        src: *const tsBSpline,
        dest: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_move(src: *mut tsBSpline, dest: *mut tsBSpline);
}
extern "C" {
    
    pub fn ts_bspline_free(spline: *mut tsBSpline);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tsDeBoorNet {
    
    pub pImpl: *mut tsDeBoorNetImpl,
}
#[test]
fn bindgen_test_layout_tsDeBoorNet() {
    const UNINIT: ::std::mem::MaybeUninit<tsDeBoorNet> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<tsDeBoorNet>(),
        8usize,
        concat!("Size of: ", stringify!(tsDeBoorNet))
    );
    assert_eq!(
        ::std::mem::align_of::<tsDeBoorNet>(),
        8usize,
        concat!("Alignment of ", stringify!(tsDeBoorNet))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pImpl) as usize - ptr as usize },
        0usize,
        concat!(
        "Offset of field: ",
        stringify!(tsDeBoorNet),
        "::",
        stringify!(pImpl)
        )
    );
}
extern "C" {
    
    pub fn ts_deboornet_knot(net: *const tsDeBoorNet) -> tsReal;
}
extern "C" {
    
    pub fn ts_deboornet_index(net: *const tsDeBoorNet) -> usize;
}
extern "C" {
    
    pub fn ts_deboornet_multiplicity(net: *const tsDeBoorNet) -> usize;
}
extern "C" {
    
    pub fn ts_deboornet_num_insertions(net: *const tsDeBoorNet) -> usize;
}
extern "C" {
    
    pub fn ts_deboornet_dimension(net: *const tsDeBoorNet) -> usize;
}
extern "C" {
    
    pub fn ts_deboornet_len_points(net: *const tsDeBoorNet) -> usize;
}
extern "C" {
    
    pub fn ts_deboornet_num_points(net: *const tsDeBoorNet) -> usize;
}
extern "C" {
    
    pub fn ts_deboornet_sof_points(net: *const tsDeBoorNet) -> usize;
}
extern "C" {
    
    pub fn ts_deboornet_points_ptr(net: *const tsDeBoorNet) -> *const tsReal;
}
extern "C" {
    
    pub fn ts_deboornet_points(
        net: *const tsDeBoorNet,
        points: *mut *mut tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_deboornet_len_result(net: *const tsDeBoorNet) -> usize;
}
extern "C" {
    
    pub fn ts_deboornet_num_result(net: *const tsDeBoorNet) -> usize;
}
extern "C" {
    
    pub fn ts_deboornet_sof_result(net: *const tsDeBoorNet) -> usize;
}
extern "C" {
    
    pub fn ts_deboornet_result_ptr(net: *const tsDeBoorNet) -> *const tsReal;
}
extern "C" {
    
    pub fn ts_deboornet_result(
        net: *const tsDeBoorNet,
        result: *mut *mut tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_deboornet_init() -> tsDeBoorNet;
}
extern "C" {
    
    pub fn ts_deboornet_copy(
        src: *const tsDeBoorNet,
        dest: *mut tsDeBoorNet,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_deboornet_move(src: *mut tsDeBoorNet, dest: *mut tsDeBoorNet);
}
extern "C" {
    
    pub fn ts_deboornet_free(net: *mut tsDeBoorNet);
}
extern "C" {
    
    pub fn ts_bspline_interpolate_cubic_natural(
        points: *const tsReal,
        num_points: usize,
        dimension: usize,
        spline: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_interpolate_catmull_rom(
        points: *const tsReal,
        num_points: usize,
        dimension: usize,
        alpha: tsReal,
        first: *const tsReal,
        last: *const tsReal,
        epsilon: tsReal,
        spline: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_eval(
        spline: *const tsBSpline,
        knot: tsReal,
        net: *mut tsDeBoorNet,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_eval_all(
        spline: *const tsBSpline,
        knots: *const tsReal,
        num: usize,
        points: *mut *mut tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_sample(
        spline: *const tsBSpline,
        num: usize,
        points: *mut *mut tsReal,
        actual_num: *mut usize,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_bisect(
        spline: *const tsBSpline,
        value: tsReal,
        epsilon: tsReal,
        persnickety: ::std::os::raw::c_int,
        index: usize,
        ascending: ::std::os::raw::c_int,
        max_iter: usize,
        net: *mut tsDeBoorNet,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_domain(spline: *const tsBSpline, min: *mut tsReal, max: *mut tsReal);
}
extern "C" {
    
    pub fn ts_bspline_is_closed(
        spline: *const tsBSpline,
        epsilon: tsReal,
        closed: *mut ::std::os::raw::c_int,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_compute_rmf(
        spline: *const tsBSpline,
        knots: *const tsReal,
        num: usize,
        has_first_normal: ::std::os::raw::c_int,
        frames: *mut tsFrame,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_chord_lengths(
        spline: *const tsBSpline,
        knots: *const tsReal,
        num: usize,
        lengths: *mut tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_sub_spline(
        spline: *const tsBSpline,
        knot0: tsReal,
        knot1: tsReal,
        sub: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_uniform_knot_seq(spline: *const tsBSpline, num: usize, knots: *mut tsReal);
}
extern "C" {
    
    pub fn ts_bspline_equidistant_knot_seq(
        spline: *const tsBSpline,
        num: usize,
        knots: *mut tsReal,
        num_samples: usize,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_derive(
        spline: *const tsBSpline,
        n: usize,
        epsilon: tsReal,
        deriv: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_insert_knot(
        spline: *const tsBSpline,
        knot: tsReal,
        num: usize,
        result: *mut tsBSpline,
        k: *mut usize,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_split(
        spline: *const tsBSpline,
        knot: tsReal,
        split: *mut tsBSpline,
        k: *mut usize,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_tension(
        spline: *const tsBSpline,
        beta: tsReal,
        out: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_to_beziers(
        spline: *const tsBSpline,
        beziers: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_elevate_degree(
        spline: *const tsBSpline,
        amount: usize,
        epsilon: tsReal,
        elevated: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_align(
        s1: *const tsBSpline,
        s2: *const tsBSpline,
        epsilon: tsReal,
        s1_out: *mut tsBSpline,
        s2_out: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_morph(
        origin: *const tsBSpline,
        target: *const tsBSpline,
        t: tsReal,
        epsilon: tsReal,
        out: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_to_json(
        spline: *const tsBSpline,
        json: *mut *mut ::std::os::raw::c_char,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_parse_json(
        json: *const ::std::os::raw::c_char,
        spline: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_save(
        spline: *const tsBSpline,
        path: *const ::std::os::raw::c_char,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_bspline_load(
        path: *const ::std::os::raw::c_char,
        spline: *mut tsBSpline,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_vec2_init(out: *mut tsReal, x: tsReal, y: tsReal);
}
extern "C" {
    
    pub fn ts_vec3_init(out: *mut tsReal, x: tsReal, y: tsReal, z: tsReal);
}
extern "C" {
    
    pub fn ts_vec4_init(out: *mut tsReal, x: tsReal, y: tsReal, z: tsReal, w: tsReal);
}
extern "C" {
    
    pub fn ts_vec2_set(out: *mut tsReal, x: *const tsReal, dim: usize);
}
extern "C" {
    
    pub fn ts_vec3_set(out: *mut tsReal, x: *const tsReal, dim: usize);
}
extern "C" {
    
    pub fn ts_vec4_set(out: *mut tsReal, x: *const tsReal, dim: usize);
}
extern "C" {
    
    pub fn ts_vec_add(x: *const tsReal, y: *const tsReal, dim: usize, out: *mut tsReal);
}
extern "C" {
    
    pub fn ts_vec_sub(x: *const tsReal, y: *const tsReal, dim: usize, out: *mut tsReal);
}
extern "C" {
    
    pub fn ts_vec_dot(x: *const tsReal, y: *const tsReal, dim: usize) -> tsReal;
}
extern "C" {
    
    pub fn ts_vec_angle(x: *const tsReal, y: *const tsReal, buf: *mut tsReal, dim: usize)
                        -> tsReal;
}
extern "C" {
    
    pub fn ts_vec3_cross(x: *const tsReal, y: *const tsReal, out: *mut tsReal);
}
extern "C" {
    
    pub fn ts_vec_norm(x: *const tsReal, dim: usize, out: *mut tsReal);
}
extern "C" {
    
    pub fn ts_vec_mag(x: *const tsReal, dim: usize) -> tsReal;
}
extern "C" {
    
    pub fn ts_vec_mul(x: *const tsReal, dim: usize, val: tsReal, out: *mut tsReal);
}
extern "C" {
    
    pub fn ts_chord_lengths_length_to_knot(
        knots: *const tsReal,
        lengths: *const tsReal,
        num: usize,
        len: tsReal,
        knot: *mut tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_chord_lengths_t_to_knot(
        knots: *const tsReal,
        lengths: *const tsReal,
        num: usize,
        t: tsReal,
        knot: *mut tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_chord_lengths_equidistant_knot_seq(
        knots: *const tsReal,
        lengths: *const tsReal,
        num: usize,
        num_knot_seq: usize,
        knot_seq: *mut tsReal,
        status: *mut tsStatus,
    ) -> tsError;
}
extern "C" {
    
    pub fn ts_knots_equal(x: tsReal, y: tsReal) -> ::std::os::raw::c_int;
}
extern "C" {
    
    pub fn ts_arr_fill(arr: *mut tsReal, num: usize, val: tsReal);
}
extern "C" {
    
    pub fn ts_distance(x: *const tsReal, y: *const tsReal, dim: usize) -> tsReal;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tsBSplineImpl {
    pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tsDeBoorNetImpl {
    pub _address: u8,
}
