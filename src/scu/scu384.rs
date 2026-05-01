#[doc = "Register `SCU384` reader"]
pub type R = crate::R<Scu384Spec>;
#[doc = "Register `SCU384` writer"]
pub type W = crate::W<Scu384Spec>;
#[doc = "Field `SCUDUTYSELRGMII` reader - SCU_DUTY_SEL_RGMII"]
pub type ScudutyselrgmiiR = crate::FieldReader;
#[doc = "Field `SCUDUTYSELRGMII` writer - SCU_DUTY_SEL_RGMII"]
pub type ScudutyselrgmiiW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 8:14 - SCU_DUTY_SEL_RGMII"]
    #[inline(always)]
    pub fn scudutyselrgmii(&self) -> ScudutyselrgmiiR {
        ScudutyselrgmiiR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:14 - SCU_DUTY_SEL_RGMII"]
    #[inline(always)]
    pub fn scudutyselrgmii(&mut self) -> ScudutyselrgmiiW<Scu384Spec> {
        ScudutyselrgmiiW::new(self, 8)
    }
}
#[doc = "Clock Duty Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu384::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu384::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu384Spec;
impl crate::RegisterSpec for Scu384Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu384::R`](R) reader structure"]
impl crate::Readable for Scu384Spec {}
#[doc = "`write(|w| ..)` method takes [`scu384::W`](W) writer structure"]
impl crate::Writable for Scu384Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU384 to value 0"]
impl crate::Resettable for Scu384Spec {}
