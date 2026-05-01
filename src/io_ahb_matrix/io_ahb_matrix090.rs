#[doc = "Register `IO_AHB_MATRIX090` reader"]
pub type R = crate::R<IoAhbMatrix090Spec>;
#[doc = "Register `IO_AHB_MATRIX090` writer"]
pub type W = crate::W<IoAhbMatrix090Spec>;
impl W {}
#[doc = "AHBM090 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix090Spec;
impl crate::RegisterSpec for IoAhbMatrix090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix090::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix090Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix090::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX090 to value 0"]
impl crate::Resettable for IoAhbMatrix090Spec {}
