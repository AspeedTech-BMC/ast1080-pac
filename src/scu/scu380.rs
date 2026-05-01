#[doc = "Register `SCU380` reader"]
pub type R = crate::R<Scu380Spec>;
#[doc = "Register `SCU380` writer"]
pub type W = crate::W<Scu380Spec>;
#[doc = "Field `SCUDUTYRINGEN` reader - SCU_DUTY_RING_EN"]
pub type ScudutyringenR = crate::BitReader;
#[doc = "Field `SCUDUTYRINGEN` writer - SCU_DUTY_RING_EN"]
pub type ScudutyringenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDUTYRINGSEL` reader - SCU_DUTY_RING_SEL"]
pub type ScudutyringselR = crate::FieldReader;
#[doc = "Field `SCUDUTYRINGSEL` writer - SCU_DUTY_RING_SEL"]
pub type ScudutyringselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCUDUTYSTART` reader - SCU_DUTY_START"]
pub type ScudutystartR = crate::BitReader;
#[doc = "Field `SCUDUTYSTART` writer - SCU_DUTY_START"]
pub type ScudutystartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDUTYRESULTSEL` reader - SCU_DUTY_RESULT_SEL"]
pub type ScudutyresultselR = crate::FieldReader;
#[doc = "Field `SCUDUTYRESULTSEL` writer - SCU_DUTY_RESULT_SEL"]
pub type ScudutyresultselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `SCUDUTYDONERGMII` reader - SCU_DUTY_DONE_RGMII"]
pub type ScudutydonergmiiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SCU_DUTY_RING_EN"]
    #[inline(always)]
    pub fn scudutyringen(&self) -> ScudutyringenR {
        ScudutyringenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - SCU_DUTY_RING_SEL"]
    #[inline(always)]
    pub fn scudutyringsel(&self) -> ScudutyringselR {
        ScudutyringselR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - SCU_DUTY_START"]
    #[inline(always)]
    pub fn scudutystart(&self) -> ScudutystartR {
        ScudutystartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_DUTY_RESULT_SEL"]
    #[inline(always)]
    pub fn scudutyresultsel(&self) -> ScudutyresultselR {
        ScudutyresultselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - SCU_DUTY_DONE_RGMII"]
    #[inline(always)]
    pub fn scudutydonergmii(&self) -> ScudutydonergmiiR {
        ScudutydonergmiiR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DUTY_RING_EN"]
    #[inline(always)]
    pub fn scudutyringen(&mut self) -> ScudutyringenW<Scu380Spec> {
        ScudutyringenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - SCU_DUTY_RING_SEL"]
    #[inline(always)]
    pub fn scudutyringsel(&mut self) -> ScudutyringselW<Scu380Spec> {
        ScudutyringselW::new(self, 1)
    }
    #[doc = "Bit 3 - SCU_DUTY_START"]
    #[inline(always)]
    pub fn scudutystart(&mut self) -> ScudutystartW<Scu380Spec> {
        ScudutystartW::new(self, 3)
    }
    #[doc = "Bits 4:6 - SCU_DUTY_RESULT_SEL"]
    #[inline(always)]
    pub fn scudutyresultsel(&mut self) -> ScudutyresultselW<Scu380Spec> {
        ScudutyresultselW::new(self, 4)
    }
}
#[doc = "Clock Duty Measurement Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu380::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu380::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu380Spec;
impl crate::RegisterSpec for Scu380Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu380::R`](R) reader structure"]
impl crate::Readable for Scu380Spec {}
#[doc = "`write(|w| ..)` method takes [`scu380::W`](W) writer structure"]
impl crate::Writable for Scu380Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU380 to value 0"]
impl crate::Resettable for Scu380Spec {}
