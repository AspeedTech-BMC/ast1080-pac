#[doc = "Register `IO_AHB_MATRIX0C8` reader"]
pub type R = crate::R<IoAhbMatrix0c8Spec>;
#[doc = "Register `IO_AHB_MATRIX0C8` writer"]
pub type W = crate::W<IoAhbMatrix0c8Spec>;
impl W {}
#[doc = "AHBM0C8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix0c8Spec;
impl crate::RegisterSpec for IoAhbMatrix0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix0c8::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix0c8::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX0C8 to value 0"]
impl crate::Resettable for IoAhbMatrix0c8Spec {}
