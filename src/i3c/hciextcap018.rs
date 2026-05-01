#[doc = "Register `HCIEXTCAP018` reader"]
pub type R = crate::R<Hciextcap018Spec>;
#[doc = "Register `HCIEXTCAP018` writer"]
pub type W = crate::W<Hciextcap018Spec>;
#[doc = "Field `REGEXTCAPID` reader - REG_EXTCAP_ID"]
pub type RegextcapidR = crate::FieldReader;
#[doc = "Field `REGEXTCAPLENGTH` reader - REG_EXTCAP_LENGTH"]
pub type RegextcaplengthR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - REG_EXTCAP_ID"]
    #[inline(always)]
    pub fn regextcapid(&self) -> RegextcapidR {
        RegextcapidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - REG_EXTCAP_LENGTH"]
    #[inline(always)]
    pub fn regextcaplength(&self) -> RegextcaplengthR {
        RegextcaplengthR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "EXTCAP\\_HEADER\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap018Spec;
impl crate::RegisterSpec for Hciextcap018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap018::R`](R) reader structure"]
impl crate::Readable for Hciextcap018Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap018::W`](W) writer structure"]
impl crate::Writable for Hciextcap018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP018 to value 0x04c0"]
impl crate::Resettable for Hciextcap018Spec {
    const RESET_VALUE: u32 = 0x04c0;
}
