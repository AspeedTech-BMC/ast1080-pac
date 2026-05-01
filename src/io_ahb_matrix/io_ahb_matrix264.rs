#[doc = "Register `IO_AHB_MATRIX264` reader"]
pub type R = crate::R<IoAhbMatrix264Spec>;
#[doc = "Register `IO_AHB_MATRIX264` writer"]
pub type W = crate::W<IoAhbMatrix264Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:31 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {}
#[doc = "AHBM264 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix264::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix264::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix264Spec;
impl crate::RegisterSpec for IoAhbMatrix264Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix264::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix264Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix264::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix264Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX264 to value 0x07"]
impl crate::Resettable for IoAhbMatrix264Spec {
    const RESET_VALUE: u32 = 0x07;
}
