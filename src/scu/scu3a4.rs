#[doc = "Register `SCU3A4` reader"]
pub type R = crate::R<Scu3a4Spec>;
#[doc = "Register `SCU3A4` writer"]
pub type W = crate::W<Scu3a4Spec>;
#[doc = "Field `SCUFREQLOWER` reader - SCU_FREQ_LOWER"]
pub type ScufreqlowerR = crate::FieldReader<u16>;
#[doc = "Field `SCUFREQLOWER` writer - SCU_FREQ_LOWER"]
pub type ScufreqlowerW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUFREQUPPER` reader - SCU_FREQ_UPPER"]
pub type ScufrequpperR = crate::FieldReader<u16>;
#[doc = "Field `SCUFREQUPPER` writer - SCU_FREQ_UPPER"]
pub type ScufrequpperW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - SCU_FREQ_LOWER"]
    #[inline(always)]
    pub fn scufreqlower(&self) -> ScufreqlowerR {
        ScufreqlowerR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:29 - SCU_FREQ_UPPER"]
    #[inline(always)]
    pub fn scufrequpper(&self) -> ScufrequpperR {
        ScufrequpperR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - SCU_FREQ_LOWER"]
    #[inline(always)]
    pub fn scufreqlower(&mut self) -> ScufreqlowerW<Scu3a4Spec> {
        ScufreqlowerW::new(self, 0)
    }
    #[doc = "Bits 16:29 - SCU_FREQ_UPPER"]
    #[inline(always)]
    pub fn scufrequpper(&mut self) -> ScufrequpperW<Scu3a4Spec> {
        ScufrequpperW::new(self, 16)
    }
}
#[doc = "Frequency Counter Comparison\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu3a4Spec;
impl crate::RegisterSpec for Scu3a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu3a4::R`](R) reader structure"]
impl crate::Readable for Scu3a4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu3a4::W`](W) writer structure"]
impl crate::Writable for Scu3a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU3A4 to value 0"]
impl crate::Resettable for Scu3a4Spec {}
