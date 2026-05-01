#[doc = "Register `IO_AHB_MATRIX148` reader"]
pub type R = crate::R<IoAhbMatrix148Spec>;
#[doc = "Register `IO_AHB_MATRIX148` writer"]
pub type W = crate::W<IoAhbMatrix148Spec>;
impl W {}
#[doc = "AHBM148 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix148::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix148::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix148Spec;
impl crate::RegisterSpec for IoAhbMatrix148Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix148::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix148Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix148::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix148Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX148 to value 0"]
impl crate::Resettable for IoAhbMatrix148Spec {}
