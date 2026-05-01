#[doc = "Register `HCICAPABILITY038` reader"]
pub type R = crate::R<Hcicapability038Spec>;
#[doc = "Register `HCICAPABILITY038` writer"]
pub type W = crate::W<Hcicapability038Spec>;
#[doc = "Field `REGRINGHEADERSECTIONOFFSET` reader - REG_RING_HEADER_SECTION_OFFSET"]
pub type RegringheadersectionoffsetR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - REG_RING_HEADER_SECTION_OFFSET"]
    #[inline(always)]
    pub fn regringheadersectionoffset(&self) -> RegringheadersectionoffsetR {
        RegringheadersectionoffsetR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "RING\\_HEADERS\\_SECTION\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability038Spec;
impl crate::RegisterSpec for Hcicapability038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability038::R`](R) reader structure"]
impl crate::Readable for Hcicapability038Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability038::W`](W) writer structure"]
impl crate::Writable for Hcicapability038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY038 to value 0x0800"]
impl crate::Resettable for Hcicapability038Spec {
    const RESET_VALUE: u32 = 0x0800;
}
