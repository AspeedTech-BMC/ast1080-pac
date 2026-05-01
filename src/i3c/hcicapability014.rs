#[doc = "Register `HCICAPABILITY014` reader"]
pub type R = crate::R<Hcicapability014Spec>;
#[doc = "Register `HCICAPABILITY014` writer"]
pub type W = crate::W<Hcicapability014Spec>;
#[doc = "Field `REGCURRENTCONTROLLER` reader - REG_CURRENT_CONTROLLER"]
pub type RegcurrentcontrollerR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - REG_CURRENT_CONTROLLER"]
    #[inline(always)]
    pub fn regcurrentcontroller(&self) -> RegcurrentcontrollerR {
        RegcurrentcontrollerR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {}
#[doc = "PRESENT\\_STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability014Spec;
impl crate::RegisterSpec for Hcicapability014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability014::R`](R) reader structure"]
impl crate::Readable for Hcicapability014Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability014::W`](W) writer structure"]
impl crate::Writable for Hcicapability014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY014 to value 0"]
impl crate::Resettable for Hcicapability014Spec {}
