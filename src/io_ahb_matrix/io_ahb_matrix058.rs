#[doc = "Register `IO_AHB_MATRIX058` reader"]
pub type R = crate::R<IoAhbMatrix058Spec>;
#[doc = "Register `IO_AHB_MATRIX058` writer"]
pub type W = crate::W<IoAhbMatrix058Spec>;
impl W {}
#[doc = "AHBM058 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix058Spec;
impl crate::RegisterSpec for IoAhbMatrix058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix058::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix058Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix058::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX058 to value 0"]
impl crate::Resettable for IoAhbMatrix058Spec {}
