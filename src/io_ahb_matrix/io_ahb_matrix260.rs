#[doc = "Register `IO_AHB_MATRIX260` reader"]
pub type R = crate::R<IoAhbMatrix260Spec>;
#[doc = "Register `IO_AHB_MATRIX260` writer"]
pub type W = crate::W<IoAhbMatrix260Spec>;
impl W {}
#[doc = "AHBM260 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix260::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix260::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix260Spec;
impl crate::RegisterSpec for IoAhbMatrix260Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix260::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix260Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix260::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix260Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX260 to value 0xffff_ffff"]
impl crate::Resettable for IoAhbMatrix260Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
