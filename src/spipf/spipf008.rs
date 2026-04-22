#[doc = "Register `SPIPF008` reader"]
pub type R = crate::R<Spipf008Spec>;
#[doc = "Register `SPIPF008` writer"]
pub type W = crate::W<Spipf008Spec>;
#[doc = "Field `CurExtendAddrRegRxdFromSPIMaster` reader - Current Extend Address Register Received from SPI master"]
pub type CurExtendAddrRegRxdFromSpimasterR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Current Extend Address Register Received from SPI master"]
    #[inline(always)]
    pub fn cur_extend_addr_reg_rxd_from_spimaster(&self) -> CurExtendAddrRegRxdFromSpimasterR {
        CurExtendAddrRegRxdFromSpimasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "EAR and Over Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf008Spec;
impl crate::RegisterSpec for Spipf008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf008::R`](R) reader structure"]
impl crate::Readable for Spipf008Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf008::W`](W) writer structure"]
impl crate::Writable for Spipf008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF008 to value 0"]
impl crate::Resettable for Spipf008Spec {}
