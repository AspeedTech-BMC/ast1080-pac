#[doc = "Register `IO_AHB_MATRIX180` reader"]
pub type R = crate::R<IoAhbMatrix180Spec>;
#[doc = "Register `IO_AHB_MATRIX180` writer"]
pub type W = crate::W<IoAhbMatrix180Spec>;
impl W {}
#[doc = "AHBM180 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix180::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix180::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix180Spec;
impl crate::RegisterSpec for IoAhbMatrix180Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix180::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix180Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix180::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix180Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX180 to value 0"]
impl crate::Resettable for IoAhbMatrix180Spec {}
