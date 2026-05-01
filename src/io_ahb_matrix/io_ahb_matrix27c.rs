#[doc = "Register `IO_AHB_MATRIX27C` reader"]
pub type R = crate::R<IoAhbMatrix27cSpec>;
#[doc = "Register `IO_AHB_MATRIX27C` writer"]
pub type W = crate::W<IoAhbMatrix27cSpec>;
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
#[doc = "AHBM27C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix27c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix27c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix27cSpec;
impl crate::RegisterSpec for IoAhbMatrix27cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix27c::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix27cSpec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix27c::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix27cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX27C to value 0"]
impl crate::Resettable for IoAhbMatrix27cSpec {}
