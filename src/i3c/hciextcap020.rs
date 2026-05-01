#[doc = "Register `HCIEXTCAP020` reader"]
pub type R = crate::R<Hciextcap020Spec>;
#[doc = "Register `HCIEXTCAP020` writer"]
pub type W = crate::W<Hciextcap020Spec>;
#[doc = "Field `REGEXTCAPPHY` reader - REG_EXTCAP_PHY"]
pub type RegextcapphyR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_EXTCAP_PHY"]
    #[inline(always)]
    pub fn regextcapphy(&self) -> RegextcapphyR {
        RegextcapphyR::new(self.bits)
    }
}
impl W {}
#[doc = "EXTCAP\\_PHY\n\nYou can [`read`](crate::Reg::read) this register and get [`hciextcap020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hciextcap020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hciextcap020Spec;
impl crate::RegisterSpec for Hciextcap020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hciextcap020::R`](R) reader structure"]
impl crate::Readable for Hciextcap020Spec {}
#[doc = "`write(|w| ..)` method takes [`hciextcap020::W`](W) writer structure"]
impl crate::Writable for Hciextcap020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIEXTCAP020 to value 0x0e00"]
impl crate::Resettable for Hciextcap020Spec {
    const RESET_VALUE: u32 = 0x0e00;
}
