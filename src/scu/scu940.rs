#[doc = "Register `SCU940` reader"]
pub type R = crate::R<Scu940Spec>;
#[doc = "Register `SCU940` writer"]
pub type W = crate::W<Scu940Spec>;
#[doc = "Field `SCUSSPENABLE` reader - SCU_SSP_ENABLE"]
pub type ScusspenableR = crate::BitReader;
#[doc = "Field `SCUSSPENABLE` writer - SCU_SSP_ENABLE"]
pub type ScusspenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPRESET` reader - SCU_SSP_RESET"]
pub type ScusspresetR = crate::BitReader;
#[doc = "Field `SCUSSPRESET` writer - SCU_SSP_RESET"]
pub type ScusspresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPDBGENABLE` reader - SCU_SSP_DBG_ENABLE"]
pub type ScusspdbgenableR = crate::BitReader;
#[doc = "Field `SCUSSPDBGENABLE` writer - SCU_SSP_DBG_ENABLE"]
pub type ScusspdbgenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPDBGEN` reader - SCU_SSP_DBGEN"]
pub type ScusspdbgenR = crate::BitReader;
#[doc = "Field `SCUSSPDBGEN` writer - SCU_SSP_DBGEN"]
pub type ScusspdbgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPNIDEN` reader - SCU_SSP_NIDEN"]
pub type ScusspnidenR = crate::BitReader;
#[doc = "Field `SCUSSPNIDEN` writer - SCU_SSP_NIDEN"]
pub type ScusspnidenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPSRAMSLP` reader - SCU_SSP_SRAM_SLP"]
pub type ScusspsramslpR = crate::BitReader;
#[doc = "Field `SCUSSPSRAMSLP` writer - SCU_SSP_SRAM_SLP"]
pub type ScusspsramslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPSRAMDSLP` reader - SCU_SSP_SRAM_DSLP"]
pub type ScusspsramdslpR = crate::BitReader;
#[doc = "Field `SCUSSPSRAMDSLP` writer - SCU_SSP_SRAM_DSLP"]
pub type ScusspsramdslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPSRAMSD` reader - SCU_SSP_SRAM_SD"]
pub type ScusspsramsdR = crate::BitReader;
#[doc = "Field `SCUSSPSRAMSD` writer - SCU_SSP_SRAM_SD"]
pub type ScusspsramsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENRESETSSP` reader - ENRESET_SSP"]
pub type EnresetsspR = crate::BitReader;
#[doc = "Field `SCUCRESETSSP` reader - SCU_CRESET_SSP"]
pub type ScucresetsspR = crate::BitReader;
#[doc = "Field `PSSPWDTRESET` reader - P_SSP_WDT_RESET"]
pub type PsspwdtresetR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SCU_SSP_ENABLE"]
    #[inline(always)]
    pub fn scusspenable(&self) -> ScusspenableR {
        ScusspenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_SSP_RESET"]
    #[inline(always)]
    pub fn scusspreset(&self) -> ScusspresetR {
        ScusspresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_SSP_DBG_ENABLE"]
    #[inline(always)]
    pub fn scusspdbgenable(&self) -> ScusspdbgenableR {
        ScusspdbgenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_SSP_DBGEN"]
    #[inline(always)]
    pub fn scusspdbgen(&self) -> ScusspdbgenR {
        ScusspdbgenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_SSP_NIDEN"]
    #[inline(always)]
    pub fn scusspniden(&self) -> ScusspnidenR {
        ScusspnidenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_SSP_SRAM_SLP"]
    #[inline(always)]
    pub fn scusspsramslp(&self) -> ScusspsramslpR {
        ScusspsramslpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_SSP_SRAM_DSLP"]
    #[inline(always)]
    pub fn scusspsramdslp(&self) -> ScusspsramdslpR {
        ScusspsramdslpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_SSP_SRAM_SD"]
    #[inline(always)]
    pub fn scusspsramsd(&self) -> ScusspsramsdR {
        ScusspsramsdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ENRESET_SSP"]
    #[inline(always)]
    pub fn enresetssp(&self) -> EnresetsspR {
        EnresetsspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_CRESET_SSP"]
    #[inline(always)]
    pub fn scucresetssp(&self) -> ScucresetsspR {
        ScucresetsspR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - P_SSP_WDT_RESET"]
    #[inline(always)]
    pub fn psspwdtreset(&self) -> PsspwdtresetR {
        PsspwdtresetR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_SSP_ENABLE"]
    #[inline(always)]
    pub fn scusspenable(&mut self) -> ScusspenableW<Scu940Spec> {
        ScusspenableW::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_SSP_RESET"]
    #[inline(always)]
    pub fn scusspreset(&mut self) -> ScusspresetW<Scu940Spec> {
        ScusspresetW::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_SSP_DBG_ENABLE"]
    #[inline(always)]
    pub fn scusspdbgenable(&mut self) -> ScusspdbgenableW<Scu940Spec> {
        ScusspdbgenableW::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_SSP_DBGEN"]
    #[inline(always)]
    pub fn scusspdbgen(&mut self) -> ScusspdbgenW<Scu940Spec> {
        ScusspdbgenW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_SSP_NIDEN"]
    #[inline(always)]
    pub fn scusspniden(&mut self) -> ScusspnidenW<Scu940Spec> {
        ScusspnidenW::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_SSP_SRAM_SLP"]
    #[inline(always)]
    pub fn scusspsramslp(&mut self) -> ScusspsramslpW<Scu940Spec> {
        ScusspsramslpW::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_SSP_SRAM_DSLP"]
    #[inline(always)]
    pub fn scusspsramdslp(&mut self) -> ScusspsramdslpW<Scu940Spec> {
        ScusspsramdslpW::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_SSP_SRAM_SD"]
    #[inline(always)]
    pub fn scusspsramsd(&mut self) -> ScusspsramsdW<Scu940Spec> {
        ScusspsramsdW::new(self, 7)
    }
}
#[doc = "\\SSP\\ Service Processor Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu940::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu940::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu940Spec;
impl crate::RegisterSpec for Scu940Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu940::R`](R) reader structure"]
impl crate::Readable for Scu940Spec {}
#[doc = "`write(|w| ..)` method takes [`scu940::W`](W) writer structure"]
impl crate::Writable for Scu940Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU940 to value 0xfe"]
impl crate::Resettable for Scu940Spec {
    const RESET_VALUE: u32 = 0xfe;
}
