#[doc = "Register `SCU388` reader"]
pub type R = crate::R<Scu388Spec>;
#[doc = "Register `SCU388` writer"]
pub type W = crate::W<Scu388Spec>;
#[doc = "Field `SCUDUTYPVALUE` reader - SCU_DUTY_P_VALUE"]
pub type ScudutypvalueR = crate::FieldReader<u16>;
#[doc = "Field `SCUDUTYNVALUE` reader - SCU_DUTY_N_VALUE"]
pub type ScudutynvalueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - SCU_DUTY_P_VALUE"]
    #[inline(always)]
    pub fn scudutypvalue(&self) -> ScudutypvalueR {
        ScudutypvalueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SCU_DUTY_N_VALUE"]
    #[inline(always)]
    pub fn scudutynvalue(&self) -> ScudutynvalueR {
        ScudutynvalueR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Clock Duty Measurement Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu388::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu388::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu388Spec;
impl crate::RegisterSpec for Scu388Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu388::R`](R) reader structure"]
impl crate::Readable for Scu388Spec {}
#[doc = "`write(|w| ..)` method takes [`scu388::W`](W) writer structure"]
impl crate::Writable for Scu388Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU388 to value 0"]
impl crate::Resettable for Scu388Spec {}
