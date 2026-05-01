#[doc = "Register `IO_AHB_MATRIX050` reader"]
pub type R = crate::R<IoAhbMatrix050Spec>;
#[doc = "Register `IO_AHB_MATRIX050` writer"]
pub type W = crate::W<IoAhbMatrix050Spec>;
impl W {}
#[doc = "AHBM050 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix050Spec;
impl crate::RegisterSpec for IoAhbMatrix050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix050::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix050Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix050::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX050 to value 0"]
impl crate::Resettable for IoAhbMatrix050Spec {}
