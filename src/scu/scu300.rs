#[doc = "Register `SCU300` reader"]
pub type R = crate::R<Scu300Spec>;
#[doc = "Register `SCU300` writer"]
pub type W = crate::W<Scu300Spec>;
#[doc = "Field `SCUHPLLNUM` reader - SCU_HPLL_NUM"]
pub type ScuhpllnumR = crate::FieldReader<u16>;
#[doc = "Field `SCUHPLLNUM` writer - SCU_HPLL_NUM"]
pub type ScuhpllnumW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `SCUHPLLDENUM` reader - SCU_HPLL_DENUM"]
pub type ScuhplldenumR = crate::FieldReader;
#[doc = "Field `SCUHPLLDENUM` writer - SCU_HPLL_DENUM"]
pub type ScuhplldenumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SCUHPLLPOST` reader - SCU_HPLL_POST"]
pub type ScuhpllpostR = crate::FieldReader;
#[doc = "Field `SCUHPLLPOST` writer - SCU_HPLL_POST"]
pub type ScuhpllpostW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUHPLLOFF` reader - SCU_HPLL_OFF"]
pub type ScuhplloffR = crate::BitReader;
#[doc = "Field `SCUHPLLOFF` writer - SCU_HPLL_OFF"]
pub type ScuhplloffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUHPLLBYPASS` reader - SCU_HPLL_BYPASS"]
pub type ScuhpllbypassR = crate::BitReader;
#[doc = "Field `SCUHPLLBYPASS` writer - SCU_HPLL_BYPASS"]
pub type ScuhpllbypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUHPLLRESET` reader - SCU_HPLL_RESET"]
pub type ScuhpllresetR = crate::BitReader;
#[doc = "Field `SCUHPLLRESET` writer - SCU_HPLL_RESET"]
pub type ScuhpllresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUHPLLTEST` reader - SCU_HPLL_TEST"]
pub type ScuhplltestR = crate::BitReader;
#[doc = "Field `SCUHPLLTEST` writer - SCU_HPLL_TEST"]
pub type ScuhplltestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUHPLLFAST` reader - SCU_HPLL_FAST"]
pub type ScuhpllfastR = crate::BitReader;
#[doc = "Field `SCUHPLLFAST` writer - SCU_HPLL_FAST"]
pub type ScuhpllfastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUHPLLENSAT` reader - SCU_HPLL_ENSAT"]
pub type ScuhpllensatR = crate::BitReader;
#[doc = "Field `SCUHPLLENSAT` writer - SCU_HPLL_ENSAT"]
pub type ScuhpllensatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - SCU_HPLL_NUM"]
    #[inline(always)]
    pub fn scuhpllnum(&self) -> ScuhpllnumR {
        ScuhpllnumR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:18 - SCU_HPLL_DENUM"]
    #[inline(always)]
    pub fn scuhplldenum(&self) -> ScuhplldenumR {
        ScuhplldenumR::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bits 19:22 - SCU_HPLL_POST"]
    #[inline(always)]
    pub fn scuhpllpost(&self) -> ScuhpllpostR {
        ScuhpllpostR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - SCU_HPLL_OFF"]
    #[inline(always)]
    pub fn scuhplloff(&self) -> ScuhplloffR {
        ScuhplloffR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SCU_HPLL_BYPASS"]
    #[inline(always)]
    pub fn scuhpllbypass(&self) -> ScuhpllbypassR {
        ScuhpllbypassR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_HPLL_RESET"]
    #[inline(always)]
    pub fn scuhpllreset(&self) -> ScuhpllresetR {
        ScuhpllresetR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SCU_HPLL_TEST"]
    #[inline(always)]
    pub fn scuhplltest(&self) -> ScuhplltestR {
        ScuhplltestR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SCU_HPLL_FAST"]
    #[inline(always)]
    pub fn scuhpllfast(&self) -> ScuhpllfastR {
        ScuhpllfastR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SCU_HPLL_ENSAT"]
    #[inline(always)]
    pub fn scuhpllensat(&self) -> ScuhpllensatR {
        ScuhpllensatR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - SCU_HPLL_NUM"]
    #[inline(always)]
    pub fn scuhpllnum(&mut self) -> ScuhpllnumW<Scu300Spec> {
        ScuhpllnumW::new(self, 0)
    }
    #[doc = "Bits 13:18 - SCU_HPLL_DENUM"]
    #[inline(always)]
    pub fn scuhplldenum(&mut self) -> ScuhplldenumW<Scu300Spec> {
        ScuhplldenumW::new(self, 13)
    }
    #[doc = "Bits 19:22 - SCU_HPLL_POST"]
    #[inline(always)]
    pub fn scuhpllpost(&mut self) -> ScuhpllpostW<Scu300Spec> {
        ScuhpllpostW::new(self, 19)
    }
    #[doc = "Bit 23 - SCU_HPLL_OFF"]
    #[inline(always)]
    pub fn scuhplloff(&mut self) -> ScuhplloffW<Scu300Spec> {
        ScuhplloffW::new(self, 23)
    }
    #[doc = "Bit 24 - SCU_HPLL_BYPASS"]
    #[inline(always)]
    pub fn scuhpllbypass(&mut self) -> ScuhpllbypassW<Scu300Spec> {
        ScuhpllbypassW::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_HPLL_RESET"]
    #[inline(always)]
    pub fn scuhpllreset(&mut self) -> ScuhpllresetW<Scu300Spec> {
        ScuhpllresetW::new(self, 25)
    }
    #[doc = "Bit 26 - SCU_HPLL_TEST"]
    #[inline(always)]
    pub fn scuhplltest(&mut self) -> ScuhplltestW<Scu300Spec> {
        ScuhplltestW::new(self, 26)
    }
    #[doc = "Bit 27 - SCU_HPLL_FAST"]
    #[inline(always)]
    pub fn scuhpllfast(&mut self) -> ScuhpllfastW<Scu300Spec> {
        ScuhpllfastW::new(self, 27)
    }
    #[doc = "Bit 28 - SCU_HPLL_ENSAT"]
    #[inline(always)]
    pub fn scuhpllensat(&mut self) -> ScuhpllensatW<Scu300Spec> {
        ScuhpllensatW::new(self, 28)
    }
}
#[doc = "HPLL Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu300::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu300::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu300Spec;
impl crate::RegisterSpec for Scu300Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu300::R`](R) reader structure"]
impl crate::Readable for Scu300Spec {}
#[doc = "`write(|w| ..)` method takes [`scu300::W`](W) writer structure"]
impl crate::Writable for Scu300Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU300 to value 0x1000_004f"]
impl crate::Resettable for Scu300Spec {
    const RESET_VALUE: u32 = 0x1000_004f;
}
