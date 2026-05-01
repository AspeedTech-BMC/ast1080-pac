#[doc = "Register `IO_AHB_MATRIX134` reader"]
pub type R = crate::R<IoAhbMatrix134Spec>;
#[doc = "Register `IO_AHB_MATRIX134` writer"]
pub type W = crate::W<IoAhbMatrix134Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 6:31 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {}
#[doc = "AHBM134 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix134::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix134::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix134Spec;
impl crate::RegisterSpec for IoAhbMatrix134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix134::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix134Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix134::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix134Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX134 to value 0x08"]
impl crate::Resettable for IoAhbMatrix134Spec {
    const RESET_VALUE: u32 = 0x08;
}
