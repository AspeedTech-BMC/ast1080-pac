#[doc = "Register `IO_AHB_MATRIX010` reader"]
pub type R = crate::R<IoAhbMatrix010Spec>;
#[doc = "Register `IO_AHB_MATRIX010` writer"]
pub type W = crate::W<IoAhbMatrix010Spec>;
impl W {}
#[doc = "AHBM010 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix010Spec;
impl crate::RegisterSpec for IoAhbMatrix010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix010::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix010Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix010::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX010 to value 0"]
impl crate::Resettable for IoAhbMatrix010Spec {}
