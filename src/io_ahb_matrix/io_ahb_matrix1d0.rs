#[doc = "Register `IO_AHB_MATRIX1D0` reader"]
pub type R = crate::R<IoAhbMatrix1d0Spec>;
#[doc = "Register `IO_AHB_MATRIX1D0` writer"]
pub type W = crate::W<IoAhbMatrix1d0Spec>;
impl W {}
#[doc = "AHBM1D0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix1d0Spec;
impl crate::RegisterSpec for IoAhbMatrix1d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix1d0::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix1d0Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix1d0::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix1d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX1D0 to value 0"]
impl crate::Resettable for IoAhbMatrix1d0Spec {}
