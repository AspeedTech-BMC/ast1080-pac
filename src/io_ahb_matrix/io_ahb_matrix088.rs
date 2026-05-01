#[doc = "Register `IO_AHB_MATRIX088` reader"]
pub type R = crate::R<IoAhbMatrix088Spec>;
#[doc = "Register `IO_AHB_MATRIX088` writer"]
pub type W = crate::W<IoAhbMatrix088Spec>;
impl W {}
#[doc = "AHBM088 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix088Spec;
impl crate::RegisterSpec for IoAhbMatrix088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix088::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix088Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix088::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX088 to value 0"]
impl crate::Resettable for IoAhbMatrix088Spec {}
