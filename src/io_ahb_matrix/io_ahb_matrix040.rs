#[doc = "Register `IO_AHB_MATRIX040` reader"]
pub type R = crate::R<IoAhbMatrix040Spec>;
#[doc = "Register `IO_AHB_MATRIX040` writer"]
pub type W = crate::W<IoAhbMatrix040Spec>;
impl W {}
#[doc = "AHBM040 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix040Spec;
impl crate::RegisterSpec for IoAhbMatrix040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix040::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix040Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix040::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX040 to value 0"]
impl crate::Resettable for IoAhbMatrix040Spec {}
