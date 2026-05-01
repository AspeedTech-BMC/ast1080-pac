#[doc = "Register `HCICAPABILITY03C` reader"]
pub type R = crate::R<Hcicapability03cSpec>;
#[doc = "Register `HCICAPABILITY03C` writer"]
pub type W = crate::W<Hcicapability03cSpec>;
#[doc = "Field `REGPIOSECTIONOFFSET` reader - REG_PIO_SECTION_OFFSET"]
pub type RegpiosectionoffsetR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - REG_PIO_SECTION_OFFSET"]
    #[inline(always)]
    pub fn regpiosectionoffset(&self) -> RegpiosectionoffsetR {
        RegpiosectionoffsetR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "PIO\\_SECTION\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability03cSpec;
impl crate::RegisterSpec for Hcicapability03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability03c::R`](R) reader structure"]
impl crate::Readable for Hcicapability03cSpec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability03c::W`](W) writer structure"]
impl crate::Writable for Hcicapability03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY03C to value 0xd0"]
impl crate::Resettable for Hcicapability03cSpec {
    const RESET_VALUE: u32 = 0xd0;
}
