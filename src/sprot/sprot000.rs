#[doc = "Register `SPROT000` reader"]
pub type R = crate::R<Sprot000Spec>;
#[doc = "Register `SPROT000` writer"]
pub type W = crate::W<Sprot000Spec>;
#[doc = "Field `UNIT` reader - UNIT"]
pub type UnitR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - UNIT"]
    #[inline(always)]
    pub fn unit(&self) -> UnitR {
        UnitR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "SPROT\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot000Spec;
impl crate::RegisterSpec for Sprot000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot000::R`](R) reader structure"]
impl crate::Readable for Sprot000Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot000::W`](W) writer structure"]
impl crate::Writable for Sprot000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT000 to value 0x02"]
impl crate::Resettable for Sprot000Spec {
    const RESET_VALUE: u32 = 0x02;
}
