#[doc = "Register `IO_AHB_MATRIX23C` reader"]
pub type R = crate::R<IoAhbMatrix23cSpec>;
#[doc = "Register `IO_AHB_MATRIX23C` writer"]
pub type W = crate::W<IoAhbMatrix23cSpec>;
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
#[doc = "AHBM23C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix23c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix23c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix23cSpec;
impl crate::RegisterSpec for IoAhbMatrix23cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix23c::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix23cSpec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix23c::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix23cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX23C to value 0"]
impl crate::Resettable for IoAhbMatrix23cSpec {}
