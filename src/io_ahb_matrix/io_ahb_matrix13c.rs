#[doc = "Register `IO_AHB_MATRIX13C` reader"]
pub type R = crate::R<IoAhbMatrix13cSpec>;
#[doc = "Register `IO_AHB_MATRIX13C` writer"]
pub type W = crate::W<IoAhbMatrix13cSpec>;
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
#[doc = "AHBM13C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix13c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix13c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix13cSpec;
impl crate::RegisterSpec for IoAhbMatrix13cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix13c::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix13cSpec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix13c::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix13cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX13C to value 0"]
impl crate::Resettable for IoAhbMatrix13cSpec {}
