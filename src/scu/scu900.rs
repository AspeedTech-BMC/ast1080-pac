#[doc = "Register `SCU900` reader"]
pub type R = crate::R<Scu900Spec>;
#[doc = "Register `SCU900` writer"]
pub type W = crate::W<Scu900Spec>;
#[doc = "Field `SCUPSPENABLE` reader - SCU_PSP_ENABLE"]
pub type ScupspenableR = crate::BitReader;
#[doc = "Field `SCUPSPENABLE` writer - SCU_PSP_ENABLE"]
pub type ScupspenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPRESET` reader - SCU_PSP_RESET"]
pub type ScupspresetR = crate::BitReader;
#[doc = "Field `SCUPSPRESET` writer - SCU_PSP_RESET"]
pub type ScupspresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPDBGENABLE` reader - SCU_PSP_DBG_ENABLE"]
pub type ScupspdbgenableR = crate::BitReader;
#[doc = "Field `SCUPSPDBGENABLE` writer - SCU_PSP_DBG_ENABLE"]
pub type ScupspdbgenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPDBGEN` reader - SCU_PSP_DBGEN"]
pub type ScupspdbgenR = crate::BitReader;
#[doc = "Field `SCUPSPDBGEN` writer - SCU_PSP_DBGEN"]
pub type ScupspdbgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPNIDEN` reader - SCU_PSP_NIDEN"]
pub type ScupspnidenR = crate::BitReader;
#[doc = "Field `SCUPSPNIDEN` writer - SCU_PSP_NIDEN"]
pub type ScupspnidenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPSRAMSLP` reader - SCU_PSP_SRAM_SLP"]
pub type ScupspsramslpR = crate::BitReader;
#[doc = "Field `SCUPSPSRAMSLP` writer - SCU_PSP_SRAM_SLP"]
pub type ScupspsramslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPSRAMDSLP` reader - SCU_PSP_SRAM_DSLP"]
pub type ScupspsramdslpR = crate::BitReader;
#[doc = "Field `SCUPSPSRAMDSLP` writer - SCU_PSP_SRAM_DSLP"]
pub type ScupspsramdslpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPSRAMSD` reader - SCU_PSP_SRAM_SD"]
pub type ScupspsramsdR = crate::BitReader;
#[doc = "Field `SCUPSPSRAMSD` writer - SCU_PSP_SRAM_SD"]
pub type ScupspsramsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENRESETPSP` reader - ENRESET_PSP"]
pub type EnresetpspR = crate::BitReader;
#[doc = "Field `SCUCRESETPSP` reader - SCU_CRESET_PSP"]
pub type ScucresetpspR = crate::BitReader;
#[doc = "Field `PPSPWDTRESET` reader - P_PSP_WDT_RESET"]
pub type PpspwdtresetR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SCU_PSP_ENABLE"]
    #[inline(always)]
    pub fn scupspenable(&self) -> ScupspenableR {
        ScupspenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_PSP_RESET"]
    #[inline(always)]
    pub fn scupspreset(&self) -> ScupspresetR {
        ScupspresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_PSP_DBG_ENABLE"]
    #[inline(always)]
    pub fn scupspdbgenable(&self) -> ScupspdbgenableR {
        ScupspdbgenableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_PSP_DBGEN"]
    #[inline(always)]
    pub fn scupspdbgen(&self) -> ScupspdbgenR {
        ScupspdbgenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_PSP_NIDEN"]
    #[inline(always)]
    pub fn scupspniden(&self) -> ScupspnidenR {
        ScupspnidenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_PSP_SRAM_SLP"]
    #[inline(always)]
    pub fn scupspsramslp(&self) -> ScupspsramslpR {
        ScupspsramslpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_PSP_SRAM_DSLP"]
    #[inline(always)]
    pub fn scupspsramdslp(&self) -> ScupspsramdslpR {
        ScupspsramdslpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_PSP_SRAM_SD"]
    #[inline(always)]
    pub fn scupspsramsd(&self) -> ScupspsramsdR {
        ScupspsramsdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ENRESET_PSP"]
    #[inline(always)]
    pub fn enresetpsp(&self) -> EnresetpspR {
        EnresetpspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_CRESET_PSP"]
    #[inline(always)]
    pub fn scucresetpsp(&self) -> ScucresetpspR {
        ScucresetpspR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - P_PSP_WDT_RESET"]
    #[inline(always)]
    pub fn ppspwdtreset(&self) -> PpspwdtresetR {
        PpspwdtresetR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_PSP_ENABLE"]
    #[inline(always)]
    pub fn scupspenable(&mut self) -> ScupspenableW<Scu900Spec> {
        ScupspenableW::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_PSP_RESET"]
    #[inline(always)]
    pub fn scupspreset(&mut self) -> ScupspresetW<Scu900Spec> {
        ScupspresetW::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_PSP_DBG_ENABLE"]
    #[inline(always)]
    pub fn scupspdbgenable(&mut self) -> ScupspdbgenableW<Scu900Spec> {
        ScupspdbgenableW::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_PSP_DBGEN"]
    #[inline(always)]
    pub fn scupspdbgen(&mut self) -> ScupspdbgenW<Scu900Spec> {
        ScupspdbgenW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_PSP_NIDEN"]
    #[inline(always)]
    pub fn scupspniden(&mut self) -> ScupspnidenW<Scu900Spec> {
        ScupspnidenW::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_PSP_SRAM_SLP"]
    #[inline(always)]
    pub fn scupspsramslp(&mut self) -> ScupspsramslpW<Scu900Spec> {
        ScupspsramslpW::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_PSP_SRAM_DSLP"]
    #[inline(always)]
    pub fn scupspsramdslp(&mut self) -> ScupspsramdslpW<Scu900Spec> {
        ScupspsramdslpW::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_PSP_SRAM_SD"]
    #[inline(always)]
    pub fn scupspsramsd(&mut self) -> ScupspsramsdW<Scu900Spec> {
        ScupspsramsdW::new(self, 7)
    }
}
#[doc = "\\PSP\\ Service Processor Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu900::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu900::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu900Spec;
impl crate::RegisterSpec for Scu900Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu900::R`](R) reader structure"]
impl crate::Readable for Scu900Spec {}
#[doc = "`write(|w| ..)` method takes [`scu900::W`](W) writer structure"]
impl crate::Writable for Scu900Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU900 to value 0xfe"]
impl crate::Resettable for Scu900Spec {
    const RESET_VALUE: u32 = 0xfe;
}
