#[doc = "Register `SCU3A0` reader"]
pub type R = crate::R<Scu3a0Spec>;
#[doc = "Register `SCU3A0` writer"]
pub type W = crate::W<Scu3a0Spec>;
#[doc = "Field `SCUFREQRINGEN` reader - SCU_FREQ_RING_EN"]
pub type ScufreqringenR = crate::BitReader;
#[doc = "Field `SCUFREQRINGEN` writer - SCU_FREQ_RING_EN"]
pub type ScufreqringenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUFREQOSCEN` reader - SCU_FREQ_OSC_EN"]
pub type ScufreqoscenR = crate::BitReader;
#[doc = "Field `SCUFREQOSCEN` writer - SCU_FREQ_OSC_EN"]
pub type ScufreqoscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUFREQSELECT` reader - SCU_FREQ_SELECT"]
pub type ScufreqselectR = crate::FieldReader;
#[doc = "Field `SCUFREQSELECT` writer - SCU_FREQ_SELECT"]
pub type ScufreqselectW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUFREQDONE` reader - SCU_FREQ_DONE"]
pub type ScufreqdoneR = crate::BitReader;
#[doc = "Field `SCUFREQRESULT` reader - SCU_FREQ_RESULT"]
pub type ScufreqresultR = crate::BitReader;
#[doc = "Field `SCUFREQOE` reader - SCU_FREQ_OE"]
pub type ScufreqoeR = crate::BitReader;
#[doc = "Field `SCUFREQOE` writer - SCU_FREQ_OE"]
pub type ScufreqoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUFREQRINGSTG` reader - SCU_FREQ_RING_STG"]
pub type ScufreqringstgR = crate::FieldReader;
#[doc = "Field `SCUFREQRINGSTG` writer - SCU_FREQ_RING_STG"]
pub type ScufreqringstgW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUFREQCOUNTER` reader - SCU_FREQ_COUNTER"]
pub type ScufreqcounterR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - SCU_FREQ_RING_EN"]
    #[inline(always)]
    pub fn scufreqringen(&self) -> ScufreqringenR {
        ScufreqringenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_FREQ_OSC_EN"]
    #[inline(always)]
    pub fn scufreqoscen(&self) -> ScufreqoscenR {
        ScufreqoscenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - SCU_FREQ_SELECT"]
    #[inline(always)]
    pub fn scufreqselect(&self) -> ScufreqselectR {
        ScufreqselectR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - SCU_FREQ_DONE"]
    #[inline(always)]
    pub fn scufreqdone(&self) -> ScufreqdoneR {
        ScufreqdoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_FREQ_RESULT"]
    #[inline(always)]
    pub fn scufreqresult(&self) -> ScufreqresultR {
        ScufreqresultR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_FREQ_OE"]
    #[inline(always)]
    pub fn scufreqoe(&self) -> ScufreqoeR {
        ScufreqoeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - SCU_FREQ_RING_STG"]
    #[inline(always)]
    pub fn scufreqringstg(&self) -> ScufreqringstgR {
        ScufreqringstgR::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:29 - SCU_FREQ_COUNTER"]
    #[inline(always)]
    pub fn scufreqcounter(&self) -> ScufreqcounterR {
        ScufreqcounterR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_FREQ_RING_EN"]
    #[inline(always)]
    pub fn scufreqringen(&mut self) -> ScufreqringenW<Scu3a0Spec> {
        ScufreqringenW::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_FREQ_OSC_EN"]
    #[inline(always)]
    pub fn scufreqoscen(&mut self) -> ScufreqoscenW<Scu3a0Spec> {
        ScufreqoscenW::new(self, 1)
    }
    #[doc = "Bits 2:5 - SCU_FREQ_SELECT"]
    #[inline(always)]
    pub fn scufreqselect(&mut self) -> ScufreqselectW<Scu3a0Spec> {
        ScufreqselectW::new(self, 2)
    }
    #[doc = "Bit 8 - SCU_FREQ_OE"]
    #[inline(always)]
    pub fn scufreqoe(&mut self) -> ScufreqoeW<Scu3a0Spec> {
        ScufreqoeW::new(self, 8)
    }
    #[doc = "Bits 9:14 - SCU_FREQ_RING_STG"]
    #[inline(always)]
    pub fn scufreqringstg(&mut self) -> ScufreqringstgW<Scu3a0Spec> {
        ScufreqringstgW::new(self, 9)
    }
}
#[doc = "Frequency Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu3a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu3a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu3a0Spec;
impl crate::RegisterSpec for Scu3a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu3a0::R`](R) reader structure"]
impl crate::Readable for Scu3a0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu3a0::W`](W) writer structure"]
impl crate::Writable for Scu3a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU3A0 to value 0"]
impl crate::Resettable for Scu3a0Spec {}
