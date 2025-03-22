/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: aarch64-apple-darwin20 */
/* rustc 1.85.1 (4eb161250 2025-03-15) */
/* R version: 4.6.0 */

unsafe extern "C" {
    pub fn dasum_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn daxpy_(
        n: *const ::std::os::raw::c_int,
        da: *const f64,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn dcopy_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn ddot_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
        dy: *const f64,
        incy: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn dnrm2_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn drot_(
        n: *const ::std::os::raw::c_int,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
        c: *const f64,
        s: *const f64,
    );
    pub fn drotg_(a: *const f64, b: *const f64, c: *mut f64, s: *mut f64);
    pub fn drotm_(
        n: *const ::std::os::raw::c_int,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
        dparam: *const f64,
    );
    pub fn drotmg_(
        dd1: *const f64,
        dd2: *const f64,
        dx1: *const f64,
        dy1: *const f64,
        param: *mut f64,
    );
    pub fn dscal_(
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
    );
    pub fn dswap_(
        n: *const ::std::os::raw::c_int,
        dx: *mut f64,
        incx: *const ::std::os::raw::c_int,
        dy: *mut f64,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn idamax_(
        n: *const ::std::os::raw::c_int,
        dx: *const f64,
        incx: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn dgbmv_(
        trans: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        kl: *const ::std::os::raw::c_int,
        ku: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dgemv_(
        trans: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dsbmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dspmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        ap: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dsymv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        beta: *const f64,
        y: *mut f64,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dtbmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtpmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const f64,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtrmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtbsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtpsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const f64,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dtrsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        x: *mut f64,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn dger_(
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        y: *const f64,
        incy: *const ::std::os::raw::c_int,
        a: *mut f64,
        lda: *const ::std::os::raw::c_int,
    );
    pub fn dsyr_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        a: *mut f64,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dspr_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        ap: *mut f64,
        arg1: usize,
    );
    pub fn dsyr2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        y: *const f64,
        incy: *const ::std::os::raw::c_int,
        a: *mut f64,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn dspr2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const f64,
        incx: *const ::std::os::raw::c_int,
        y: *const f64,
        incy: *const ::std::os::raw::c_int,
        ap: *mut f64,
        arg1: usize,
    );
    pub fn dgemm_(
        transa: *const ::std::os::raw::c_char,
        transb: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dtrsm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *mut f64,
        ldb: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn dtrmm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *mut f64,
        ldb: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn dsymm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dsyrk_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dsyr2k_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const f64,
        lda: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn dcabs1_(z: *const Rcomplex) -> f64;
    pub fn dzasum_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn dznrm2_(
        n: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
    ) -> f64;
    pub fn izamax_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn zaxpy_(
        n: *const ::std::os::raw::c_int,
        za: *const Rcomplex,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn zcopy_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn zdotc_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    ) -> Rcomplex;
    pub fn zdotu_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
    ) -> Rcomplex;
    pub fn zdrot_(
        n: *const ::std::os::raw::c_int,
        zx: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        c: *const f64,
        s: *const f64,
    );
    pub fn zdscal_(
        n: *const ::std::os::raw::c_int,
        da: *const f64,
        zx: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
    );
    pub fn zgbmv_(
        trans: *const ::std::os::raw::c_char,
        m: *mut ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        kl: *mut ::std::os::raw::c_int,
        ku: *mut ::std::os::raw::c_int,
        alpha: *mut Rcomplex,
        a: *mut Rcomplex,
        lda: *mut ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *mut ::std::os::raw::c_int,
        beta: *mut Rcomplex,
        y: *mut Rcomplex,
        incy: *mut ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zgemm_(
        transa: *const ::std::os::raw::c_char,
        transb: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zgemv_(
        trans: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zgerc_(
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
    );
    pub fn zgeru_(
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
    );
    pub fn zhbmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zhemm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zhemv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zher_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zher2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        a: *mut Rcomplex,
        lda: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zher2k_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zherk_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zhpmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        ap: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
    pub fn zhpr_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        ap: *mut Rcomplex,
        arg1: usize,
    );
    pub fn zhpr2_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        y: *const Rcomplex,
        incy: *const ::std::os::raw::c_int,
        ap: *mut Rcomplex,
        arg1: usize,
    );
    pub fn zrotg_(ca: *const Rcomplex, cb: *const Rcomplex, c: *mut f64, s: *mut Rcomplex);
    pub fn zscal_(
        n: *const ::std::os::raw::c_int,
        za: *const Rcomplex,
        zx: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
    );
    pub fn zswap_(
        n: *const ::std::os::raw::c_int,
        zx: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        zy: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
    );
    pub fn zsymm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *const Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zsyr2k_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *mut ::std::os::raw::c_int,
        k: *mut ::std::os::raw::c_int,
        alpha: *mut Rcomplex,
        a: *mut Rcomplex,
        lda: *mut ::std::os::raw::c_int,
        b: *mut Rcomplex,
        ldb: *mut ::std::os::raw::c_int,
        beta: *mut Rcomplex,
        c: *mut Rcomplex,
        ldc: *mut ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn zsyrk_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        c: *mut Rcomplex,
        ldc: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
    );
    pub fn ztbmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztbsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztpmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const Rcomplex,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztpsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        ap: *const Rcomplex,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztrmm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        b: *mut Rcomplex,
        ldb: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn ztrmv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn ztrsm_(
        side: *const ::std::os::raw::c_char,
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *mut ::std::os::raw::c_int,
        n: *mut ::std::os::raw::c_int,
        alpha: *mut Rcomplex,
        a: *mut Rcomplex,
        lda: *mut ::std::os::raw::c_int,
        b: *mut Rcomplex,
        ldb: *mut ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    );
    pub fn ztrsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn zspmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        ap: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
}
4: usize,
    );
    pub fn ztrsv_(
        uplo: *const ::std::os::raw::c_char,
        trans: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        a: *const Rcomplex,
        lda: *const ::std::os::raw::c_int,
        x: *mut Rcomplex,
        incx: *const ::std::os::raw::c_int,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
    pub fn zspmv_(
        uplo: *const ::std::os::raw::c_char,
        n: *const ::std::os::raw::c_int,
        alpha: *const Rcomplex,
        ap: *const Rcomplex,
        x: *const Rcomplex,
        incx: *const ::std::os::raw::c_int,
        beta: *const Rcomplex,
        y: *mut Rcomplex,
        incy: *const ::std::os::raw::c_int,
        arg1: usize,
    );
}
