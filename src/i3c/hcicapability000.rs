#[doc = "Register `HCICAPABILITY000` reader"]
pub type R = crate::R<Hcicapability000Spec>;
#[doc = "Register `HCICAPABILITY000` writer"]
pub type W = crate::W<Hcicapability000Spec>;
#[doc = "Field `REGHCIVERSION` reader - REG_HCI_VERSION"]
pub type ReghciversionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_HCI_VERSION"]
    #[inline(always)]
    pub fn reghciversion(&self) -> ReghciversionR {
        ReghciversionR::new(self.bits)
    }
}
impl W {}
#[doc = "HCI\\_VERSION\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability000Spec;
impl crate::RegisterSpec for Hcicapability000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability000::R`](R) reader structure"]
impl crate::Readable for Hcicapability000Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability000::W`](W) writer structure"]
impl crate::Writable for Hcicapability000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY000 to value 0x0110"]
impl crate::Resettable for Hcicapability000Spec {
    const RESET_VALUE: u32 = 0x0110;
}
