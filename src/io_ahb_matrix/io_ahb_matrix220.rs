#[doc = "Register `IO_AHB_MATRIX220` reader"]
pub type R = crate::R<IoAhbMatrix220Spec>;
#[doc = "Register `IO_AHB_MATRIX220` writer"]
pub type W = crate::W<IoAhbMatrix220Spec>;
impl W {}
#[doc = "AHBM220 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix220::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix220::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix220Spec;
impl crate::RegisterSpec for IoAhbMatrix220Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix220::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix220Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix220::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix220Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX220 to value 0xffff_ffff"]
impl crate::Resettable for IoAhbMatrix220Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
