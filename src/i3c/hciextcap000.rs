#[doc = "Register `HCIEXTCAP000` reader"]
pub type R = crate::R<Hciextcap000Spec>;
#[doc = "Register `HCIEXTCAP000` writer"]
pub type W = crate::W<Hciextcap000Spec>;
#[doc = "Field `REGHWIDID` reader - REG_HW_ID_ID"]
pub type ReghwididR = crate::FieldReader;
#[doc = "Field `REGHWIDLENGTH` reader - REG_HW_ID_LENGTH"]
pub type ReghwidlengthR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - REG_HW_ID_ID"]
    #[inline(always)]
    pub fn reghwidid(&self) -> ReghwididR {
        ReghwididR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - REG_HW_ID_LENGTH"]
    #[inline(always)]
    pub fn reghwidlength(&self) -> ReghwidlengthR {
        ReghwidlengthR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "HW\\_ID\\_HEADER\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap000Spec;
impl crate::RegisterSpec for Hciextcap000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap000::R`](R) reader structure"]
impl crate::Readable for Hciextcap000Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap000::W`](W) writer structure"]
impl crate::Writable for Hciextcap000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP000 to value 0x0401"]
impl crate::Resettable for Hciextcap000Spec {
    const RESET_VALUE: u32 = 0x0401;
}
