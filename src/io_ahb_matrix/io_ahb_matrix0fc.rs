#[doc = "Register `IO_AHB_MATRIX0FC` reader"]
pub type R = crate::R<IoAhbMatrix0fcSpec>;
#[doc = "Register `IO_AHB_MATRIX0FC` writer"]
pub type W = crate::W<IoAhbMatrix0fcSpec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 8:31 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "AHBM0FC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix0fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix0fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix0fcSpec;
impl crate::RegisterSpec for IoAhbMatrix0fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix0fc::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix0fcSpec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix0fc::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix0fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX0FC to value 0"]
impl crate::Resettable for IoAhbMatrix0fcSpec {}
