#[doc = "Register `SCU31C` reader"]
pub type R = crate::R<Scu31cSpec>;
#[doc = "Register `SCU31C` writer"]
pub type W = crate::W<Scu31cSpec>;
#[doc = "Field `SCUDIPLLDIVPOSTEN` reader - SCU_DIPLL_DIV_POST_EN"]
pub type ScudiplldivpostenR = crate::BitReader;
#[doc = "Field `SCUDIPLLDIVPOSTEN` writer - SCU_DIPLL_DIV_POST_EN"]
pub type ScudiplldivpostenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLDIVPOST` reader - SCU_DIPLL_DIV_POST"]
pub type ScudiplldivpostR = crate::FieldReader;
#[doc = "Field `SCUDIPLLDIVPOST` writer - SCU_DIPLL_DIV_POST"]
pub type ScudiplldivpostW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SCUDIPLLSPREADEN` reader - SCU_DIPLL_SPREAD_EN"]
pub type ScudipllspreadenR = crate::BitReader;
#[doc = "Field `SCUDIPLLSPREADEN` writer - SCU_DIPLL_SPREAD_EN"]
pub type ScudipllspreadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLMOLPFEN` reader - SCU_DIPLL_MOLPF_EN"]
pub type ScudipllmolpfenR = crate::BitReader;
#[doc = "Field `SCUDIPLLMOLPFEN` writer - SCU_DIPLL_MOLPF_EN"]
pub type ScudipllmolpfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLDBGEN` reader - SCU_DIPLL_DBG_EN"]
pub type ScudiplldbgenR = crate::BitReader;
#[doc = "Field `SCUDIPLLDBGEN` writer - SCU_DIPLL_DBG_EN"]
pub type ScudiplldbgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDIPLLDBGSEL` reader - SCU_DIPLL_DBG_SEL"]
pub type ScudiplldbgselR = crate::FieldReader;
#[doc = "Field `SCUDIPLLDBGSEL` writer - SCU_DIPLL_DBG_SEL"]
pub type ScudiplldbgselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUDIPLLBYPASSEN` reader - SCU_DIPLL_BYPASS_EN"]
pub type ScudipllbypassenR = crate::BitReader;
#[doc = "Field `SCUDIPLLBYPASSEN` writer - SCU_DIPLL_BYPASS_EN"]
pub type ScudipllbypassenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIPLL_DIV_POST_EN"]
    #[inline(always)]
    pub fn scudiplldivposten(&self) -> ScudiplldivpostenR {
        ScudiplldivpostenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - SCU_DIPLL_DIV_POST"]
    #[inline(always)]
    pub fn scudiplldivpost(&self) -> ScudiplldivpostR {
        ScudiplldivpostR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - SCU_DIPLL_SPREAD_EN"]
    #[inline(always)]
    pub fn scudipllspreaden(&self) -> ScudipllspreadenR {
        ScudipllspreadenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_DIPLL_MOLPF_EN"]
    #[inline(always)]
    pub fn scudipllmolpfen(&self) -> ScudipllmolpfenR {
        ScudipllmolpfenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_DIPLL_DBG_EN"]
    #[inline(always)]
    pub fn scudiplldbgen(&self) -> ScudiplldbgenR {
        ScudiplldbgenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - SCU_DIPLL_DBG_SEL"]
    #[inline(always)]
    pub fn scudiplldbgsel(&self) -> ScudiplldbgselR {
        ScudiplldbgselR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - SCU_DIPLL_BYPASS_EN"]
    #[inline(always)]
    pub fn scudipllbypassen(&self) -> ScudipllbypassenR {
        ScudipllbypassenR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIPLL_DIV_POST_EN"]
    #[inline(always)]
    pub fn scudiplldivposten(&mut self) -> ScudiplldivpostenW<Scu31cSpec> {
        ScudiplldivpostenW::new(self, 0)
    }
    #[doc = "Bits 1:5 - SCU_DIPLL_DIV_POST"]
    #[inline(always)]
    pub fn scudiplldivpost(&mut self) -> ScudiplldivpostW<Scu31cSpec> {
        ScudiplldivpostW::new(self, 1)
    }
    #[doc = "Bit 6 - SCU_DIPLL_SPREAD_EN"]
    #[inline(always)]
    pub fn scudipllspreaden(&mut self) -> ScudipllspreadenW<Scu31cSpec> {
        ScudipllspreadenW::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_DIPLL_MOLPF_EN"]
    #[inline(always)]
    pub fn scudipllmolpfen(&mut self) -> ScudipllmolpfenW<Scu31cSpec> {
        ScudipllmolpfenW::new(self, 7)
    }
    #[doc = "Bit 8 - SCU_DIPLL_DBG_EN"]
    #[inline(always)]
    pub fn scudiplldbgen(&mut self) -> ScudiplldbgenW<Scu31cSpec> {
        ScudiplldbgenW::new(self, 8)
    }
    #[doc = "Bits 9:12 - SCU_DIPLL_DBG_SEL"]
    #[inline(always)]
    pub fn scudiplldbgsel(&mut self) -> ScudiplldbgselW<Scu31cSpec> {
        ScudiplldbgselW::new(self, 9)
    }
    #[doc = "Bit 13 - SCU_DIPLL_BYPASS_EN"]
    #[inline(always)]
    pub fn scudipllbypassen(&mut self) -> ScudipllbypassenW<Scu31cSpec> {
        ScudipllbypassenW::new(self, 13)
    }
}
#[doc = "DIPLL Parameter Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu31c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu31c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu31cSpec;
impl crate::RegisterSpec for Scu31cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu31c::R`](R) reader structure"]
impl crate::Readable for Scu31cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu31c::W`](W) writer structure"]
impl crate::Writable for Scu31cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU31C to value 0x03"]
impl crate::Resettable for Scu31cSpec {
    const RESET_VALUE: u32 = 0x03;
}
