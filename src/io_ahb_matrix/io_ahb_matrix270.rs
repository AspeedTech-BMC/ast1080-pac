#[doc = "Register `IO_AHB_MATRIX270` reader"]
pub type R = crate::R<IoAhbMatrix270Spec>;
#[doc = "Register `IO_AHB_MATRIX270` writer"]
pub type W = crate::W<IoAhbMatrix270Spec>;
impl W {}
#[doc = "AHBM270 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix270::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix270::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix270Spec;
impl crate::RegisterSpec for IoAhbMatrix270Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix270::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix270Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix270::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix270Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX270 to value 0xffff_ffff"]
impl crate::Resettable for IoAhbMatrix270Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
