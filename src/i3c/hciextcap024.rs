#[doc = "Register `HCIEXTCAP024` reader"]
pub type R = crate::R<Hciextcap024Spec>;
#[doc = "Register `HCIEXTCAP024` writer"]
pub type W = crate::W<Hciextcap024Spec>;
#[doc = "Field `REGEXTCAPDMAARB` reader - REG_EXTCAP_DMAARB"]
pub type RegextcapdmaarbR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_EXTCAP_DMAARB"]
    #[inline(always)]
    pub fn regextcapdmaarb(&self) -> RegextcapdmaarbR {
        RegextcapdmaarbR::new(self.bits)
    }
}
impl W {}
#[doc = "EXTCAP\\_DMAARB\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap024Spec;
impl crate::RegisterSpec for Hciextcap024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap024::R`](R) reader structure"]
impl crate::Readable for Hciextcap024Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap024::W`](W) writer structure"]
impl crate::Writable for Hciextcap024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP024 to value 0x0f80"]
impl crate::Resettable for Hciextcap024Spec {
    const RESET_VALUE: u32 = 0x0f80;
}
