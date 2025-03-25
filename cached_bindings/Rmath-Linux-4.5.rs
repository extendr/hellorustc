/* automatically generated by rust-bindgen 0.71.1 */

/* OS: unix */
/* Platform: x86_64-pc-linux-gnu */
/* rustc 1.85.0 (4d91de4e4 2025-02-17) */
/* R version: 4.5.0 */

pub const R_VERSION_STRING: &[u8; 6] = b"4.5.0\0";
pub const HAVE_EXPM1: u32 = 1;
pub const HAVE_HYPOT: u32 = 1;
pub const HAVE_LOG1P: u32 = 1;
pub const HAVE_WORKING_LOG1P: u32 = 1;
pub const M_2PI: f64 = 6.283185307179586;
pub const M_SQRT_3: f64 = 1.7320508075688772;
pub const M_SQRT_32: f64 = 5.656854249492381;
pub const M_LOG10_2: f64 = 0.3010299956639812;
pub const M_SQRT_PI: f64 = 1.772453850905516;
pub const M_1_SQRT_2PI: f64 = 0.3989422804014327;
pub const M_SQRT_2dPI: f64 = 0.7978845608028654;
pub const M_LN_2PI: f64 = 1.8378770664093456;
pub const M_LN_SQRT_PI: f64 = 0.5723649429247001;
pub const M_LN_SQRT_2PI: f64 = 0.9189385332046728;
pub const M_LN_SQRT_PId2: f64 = 0.22579135264472744;
unsafe extern "C" {
    #[doc = " R's versions with !R_FINITE checks"]
    pub fn R_pow(x: f64, y: f64) -> f64;
    pub fn R_pow_di(arg1: f64, arg2: ::std::os::raw::c_int) -> f64;
    #[doc = " Normal Distribution"]
    pub fn Rf_dnorm4(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pnorm5(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qnorm5(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rnorm(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_pnorm_both(
        arg1: f64,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    );
    #[doc = " Uniform Distribution"]
    pub fn Rf_dunif(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_punif(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qunif(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_runif(arg1: f64, arg2: f64) -> f64;
    #[doc = " Gamma Distribution"]
    pub fn Rf_dgamma(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pgamma(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qgamma(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rgamma(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_log1pmx(arg1: f64) -> f64;
    pub fn Rf_log1pexp(arg1: f64) -> f64;
    pub fn Rf_log1mexp(arg1: f64) -> f64;
    pub fn Rf_lgamma1p(arg1: f64) -> f64;
    pub fn Rf_pow1p(arg1: f64, arg2: f64) -> f64;
    #[doc = " Compute the log of a sum or difference from logs of terms, i.e.,\n\n     log (exp (logx) + exp (logy))\n or  log (exp (logx) - exp (logy))\n\n without causing overflows or throwing away too much accuracy:"]
    pub fn Rf_logspace_add(logx: f64, logy: f64) -> f64;
    pub fn Rf_logspace_sub(logx: f64, logy: f64) -> f64;
    pub fn Rf_logspace_sum(arg1: *const f64, arg2: ::std::os::raw::c_int) -> f64;
    #[doc = " Beta Distribution"]
    pub fn Rf_dbeta(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pbeta(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qbeta(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rbeta(arg1: f64, arg2: f64) -> f64;
    #[doc = " Lognormal Distribution"]
    pub fn Rf_dlnorm(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_plnorm(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qlnorm(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rlnorm(arg1: f64, arg2: f64) -> f64;
    #[doc = " Chi-squared Distribution"]
    pub fn Rf_dchisq(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pchisq(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qchisq(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rchisq(arg1: f64) -> f64;
    #[doc = " Non-central Chi-squared Distribution"]
    pub fn Rf_dnchisq(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pnchisq(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qnchisq(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rnchisq(arg1: f64, arg2: f64) -> f64;
    #[doc = " F Distribution"]
    pub fn Rf_df(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pf(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qf(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rf(arg1: f64, arg2: f64) -> f64;
    #[doc = " Student t Distribution"]
    pub fn Rf_dt(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pt(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qt(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rt(arg1: f64) -> f64;
    #[doc = " Binomial Distribution"]
    pub fn Rf_dbinom_raw(x: f64, n: f64, p: f64, q: f64, give_log: ::std::os::raw::c_int) -> f64;
    pub fn Rf_dbinom(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pbinom(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qbinom(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rbinom(arg1: f64, arg2: f64) -> f64;
    #[doc = " Multinomial Distribution"]
    pub fn Rf_rmultinom(
        arg1: ::std::os::raw::c_int,
        arg2: *mut f64,
        arg3: ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
    );
    #[doc = " Cauchy Distribution"]
    pub fn Rf_dcauchy(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pcauchy(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qcauchy(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rcauchy(arg1: f64, arg2: f64) -> f64;
    #[doc = " Exponential Distribution"]
    pub fn Rf_dexp(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pexp(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qexp(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rexp(arg1: f64) -> f64;
    #[doc = " Geometric Distribution"]
    pub fn Rf_dgeom(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pgeom(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qgeom(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rgeom(arg1: f64) -> f64;
    #[doc = " Hypergeometric Distribution"]
    pub fn Rf_dhyper(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_phyper(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qhyper(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rhyper(arg1: f64, arg2: f64, arg3: f64) -> f64;
    #[doc = " Negative Binomial Distribution"]
    pub fn Rf_dnbinom(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pnbinom(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qnbinom(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rnbinom(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_dnbinom_mu(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pnbinom_mu(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qnbinom_mu(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rnbinom_mu(arg1: f64, arg2: f64) -> f64;
    #[doc = " Poisson Distribution"]
    pub fn Rf_dpois_raw(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64;
    pub fn Rf_dpois(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64;
    pub fn Rf_ppois(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qpois(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rpois(arg1: f64) -> f64;
    #[doc = " Weibull Distribution"]
    pub fn Rf_dweibull(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pweibull(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qweibull(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rweibull(arg1: f64, arg2: f64) -> f64;
    #[doc = " Logistic Distribution"]
    pub fn Rf_dlogis(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_plogis(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qlogis(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rlogis(arg1: f64, arg2: f64) -> f64;
    #[doc = " Non-central Beta Distribution"]
    pub fn Rf_dnbeta(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_pnbeta(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qnbeta(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rnbeta(arg1: f64, arg2: f64, arg3: f64) -> f64;
    #[doc = " Non-central F Distribution"]
    pub fn Rf_dnf(arg1: f64, arg2: f64, arg3: f64, arg4: f64, arg5: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pnf(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qnf(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    ) -> f64;
    #[doc = " Non-central Student t Distribution"]
    pub fn Rf_dnt(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pnt(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qnt(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    #[doc = " Studentized Range Distribution"]
    pub fn Rf_ptukey(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qtukey(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: f64,
        arg5: ::std::os::raw::c_int,
        arg6: ::std::os::raw::c_int,
    ) -> f64;
    #[doc = " Wilcoxon Rank Sum Distribution"]
    pub fn Rf_dwilcox(arg1: f64, arg2: f64, arg3: f64, arg4: ::std::os::raw::c_int) -> f64;
    pub fn Rf_pwilcox(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qwilcox(
        arg1: f64,
        arg2: f64,
        arg3: f64,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rwilcox(arg1: f64, arg2: f64) -> f64;
    pub fn wilcox_free();
    #[doc = " Wilcoxon Signed Rank Distribution"]
    pub fn Rf_dsignrank(arg1: f64, arg2: f64, arg3: ::std::os::raw::c_int) -> f64;
    pub fn Rf_psignrank(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_qsignrank(
        arg1: f64,
        arg2: f64,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
    ) -> f64;
    pub fn Rf_rsignrank(arg1: f64) -> f64;
    pub fn signrank_free();
    #[doc = " Gamma and Related Functions"]
    pub fn Rf_gammafn(arg1: f64) -> f64;
    pub fn Rf_lgammafn(arg1: f64) -> f64;
    pub fn Rf_lgammafn_sign(arg1: f64, arg2: *mut ::std::os::raw::c_int) -> f64;
    pub fn Rf_dpsifn(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: *mut f64,
        arg6: *mut ::std::os::raw::c_int,
        arg7: *mut ::std::os::raw::c_int,
    );
    pub fn Rf_psigamma(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_digamma(arg1: f64) -> f64;
    pub fn Rf_trigamma(arg1: f64) -> f64;
    pub fn Rf_tetragamma(arg1: f64) -> f64;
    pub fn Rf_pentagamma(arg1: f64) -> f64;
    pub fn Rf_beta(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_lbeta(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_choose(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_lchoose(arg1: f64, arg2: f64) -> f64;
    #[doc = " Bessel Functions"]
    pub fn Rf_bessel_i(arg1: f64, arg2: f64, arg3: f64) -> f64;
    pub fn Rf_bessel_j(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_bessel_k(arg1: f64, arg2: f64, arg3: f64) -> f64;
    pub fn Rf_bessel_y(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_bessel_i_ex(arg1: f64, arg2: f64, arg3: f64, arg4: *mut f64) -> f64;
    pub fn Rf_bessel_j_ex(arg1: f64, arg2: f64, arg3: *mut f64) -> f64;
    pub fn Rf_bessel_k_ex(arg1: f64, arg2: f64, arg3: f64, arg4: *mut f64) -> f64;
    pub fn Rf_bessel_y_ex(arg1: f64, arg2: f64, arg3: *mut f64) -> f64;
    #[doc = " General Support Functions"]
    pub fn Rf_imax2(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn Rf_imin2(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn Rf_fmax2(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_fmin2(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_sign(arg1: f64) -> f64;
    pub fn Rf_fprec(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_fround(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_fsign(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_ftrunc(arg1: f64) -> f64;
    pub fn cospi(arg1: f64) -> f64;
    pub fn sinpi(arg1: f64) -> f64;
    pub fn tanpi(arg1: f64) -> f64;
    pub fn Rtanpi(arg1: f64) -> f64;
}
