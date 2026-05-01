#[doc = "Register `IO_AHB_MATRIX018` reader"]
pub type R = crate::R<IoAhbMatrix018Spec>;
#[doc = "Register `IO_AHB_MATRIX018` writer"]
pub type W = crate::W<IoAhbMatrix018Spec>;
impl W {}
#[doc = "AHBM018 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix018Spec;
impl crate::RegisterSpec for IoAhbMatrix018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix018::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix018Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix018::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX018 to value 0"]
impl crate::Resettable for IoAhbMatrix018Spec {}
