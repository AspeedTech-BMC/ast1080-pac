#[doc = "Register `HCICAPABILITY040` reader"]
pub type R = crate::R<Hcicapability040Spec>;
#[doc = "Register `HCICAPABILITY040` writer"]
pub type W = crate::W<Hcicapability040Spec>;
#[doc = "Field `REGEXTCAPSSECTIONOFFSET` reader - REG_EXT_CAPS_SECTION_OFFSET"]
pub type RegextcapssectionoffsetR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - REG_EXT_CAPS_SECTION_OFFSET"]
    #[inline(always)]
    pub fn regextcapssectionoffset(&self) -> RegextcapssectionoffsetR {
        RegextcapssectionoffsetR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "EXT\\_CAPS\\_SECTION\\_OFFSET\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability040Spec;
impl crate::RegisterSpec for Hcicapability040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability040::R`](R) reader structure"]
impl crate::Readable for Hcicapability040Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability040::W`](W) writer structure"]
impl crate::Writable for Hcicapability040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY040 to value 0x0f00"]
impl crate::Resettable for Hcicapability040Spec {
    const RESET_VALUE: u32 = 0x0f00;
}
