#[doc = "Register `SCU918` reader"]
pub type R = crate::R<Scu918Spec>;
#[doc = "Register `SCU918` writer"]
pub type W = crate::W<Scu918Spec>;
#[doc = "Field `SCUPSPDCACHEENABLE` reader - SCU_PSP_D_CACHE_ENABLE"]
pub type ScupspdcacheenableR = crate::BitReader;
#[doc = "Field `SCUPSPDCACHEENABLE` writer - SCU_PSP_D_CACHE_ENABLE"]
pub type ScupspdcacheenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPICACHEENABLE` reader - SCU_PSP_I_CACHE_ENABLE"]
pub type ScupspicacheenableR = crate::BitReader;
#[doc = "Field `SCUPSPICACHEENABLE` writer - SCU_PSP_I_CACHE_ENABLE"]
pub type ScupspicacheenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPDCACHERESET` reader - SCU_PSP_D_CACHE_RESET"]
pub type ScupspdcacheresetR = crate::BitReader;
#[doc = "Field `SCUPSPDCACHERESET` writer - SCU_PSP_D_CACHE_RESET"]
pub type ScupspdcacheresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPICACHERESET` reader - SCU_PSP_I_CACHE_RESET"]
pub type ScupspicacheresetR = crate::BitReader;
#[doc = "Field `SCUPSPICACHERESET` writer - SCU_PSP_I_CACHE_RESET"]
pub type ScupspicacheresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPDPREFETCHCLR` reader - SCU_PSP_D_PREFETCH_CLR"]
pub type ScupspdprefetchclrR = crate::BitReader;
#[doc = "Field `SCUPSPDPREFETCHCLR` writer - SCU_PSP_D_PREFETCH_CLR"]
pub type ScupspdprefetchclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPIPREFETCHCLR` reader - SCU_PSP_I_PREFETCH_CLR"]
pub type ScupspiprefetchclrR = crate::BitReader;
#[doc = "Field `SCUPSPIPREFETCHCLR` writer - SCU_PSP_I_PREFETCH_CLR"]
pub type ScupspiprefetchclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPSRAMMODEEN` reader - SCU_PSP_SRAM_MODE_EN"]
pub type ScupspsrammodeenR = crate::BitReader;
#[doc = "Field `SCUPSPSRAMMODEEN` writer - SCU_PSP_SRAM_MODE_EN"]
pub type ScupspsrammodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPSRAMMASTERSEL` reader - SCU_PSP_SRAM_MASTER_SEL"]
pub type ScupspsrammasterselR = crate::BitReader;
#[doc = "Field `SCUPSPSRAMMASTERSEL` writer - SCU_PSP_SRAM_MASTER_SEL"]
pub type ScupspsrammasterselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_PSP_D_CACHE_ENABLE"]
    #[inline(always)]
    pub fn scupspdcacheenable(&self) -> ScupspdcacheenableR {
        ScupspdcacheenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_PSP_I_CACHE_ENABLE"]
    #[inline(always)]
    pub fn scupspicacheenable(&self) -> ScupspicacheenableR {
        ScupspicacheenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_PSP_D_CACHE_RESET"]
    #[inline(always)]
    pub fn scupspdcachereset(&self) -> ScupspdcacheresetR {
        ScupspdcacheresetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_PSP_I_CACHE_RESET"]
    #[inline(always)]
    pub fn scupspicachereset(&self) -> ScupspicacheresetR {
        ScupspicacheresetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_PSP_D_PREFETCH_CLR"]
    #[inline(always)]
    pub fn scupspdprefetchclr(&self) -> ScupspdprefetchclrR {
        ScupspdprefetchclrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_PSP_I_PREFETCH_CLR"]
    #[inline(always)]
    pub fn scupspiprefetchclr(&self) -> ScupspiprefetchclrR {
        ScupspiprefetchclrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_PSP_SRAM_MODE_EN"]
    #[inline(always)]
    pub fn scupspsrammodeen(&self) -> ScupspsrammodeenR {
        ScupspsrammodeenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_PSP_SRAM_MASTER_SEL"]
    #[inline(always)]
    pub fn scupspsrammastersel(&self) -> ScupspsrammasterselR {
        ScupspsrammasterselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_PSP_D_CACHE_ENABLE"]
    #[inline(always)]
    pub fn scupspdcacheenable(&mut self) -> ScupspdcacheenableW<Scu918Spec> {
        ScupspdcacheenableW::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_PSP_I_CACHE_ENABLE"]
    #[inline(always)]
    pub fn scupspicacheenable(&mut self) -> ScupspicacheenableW<Scu918Spec> {
        ScupspicacheenableW::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_PSP_D_CACHE_RESET"]
    #[inline(always)]
    pub fn scupspdcachereset(&mut self) -> ScupspdcacheresetW<Scu918Spec> {
        ScupspdcacheresetW::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_PSP_I_CACHE_RESET"]
    #[inline(always)]
    pub fn scupspicachereset(&mut self) -> ScupspicacheresetW<Scu918Spec> {
        ScupspicacheresetW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_PSP_D_PREFETCH_CLR"]
    #[inline(always)]
    pub fn scupspdprefetchclr(&mut self) -> ScupspdprefetchclrW<Scu918Spec> {
        ScupspdprefetchclrW::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_PSP_I_PREFETCH_CLR"]
    #[inline(always)]
    pub fn scupspiprefetchclr(&mut self) -> ScupspiprefetchclrW<Scu918Spec> {
        ScupspiprefetchclrW::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_PSP_SRAM_MODE_EN"]
    #[inline(always)]
    pub fn scupspsrammodeen(&mut self) -> ScupspsrammodeenW<Scu918Spec> {
        ScupspsrammodeenW::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_PSP_SRAM_MASTER_SEL"]
    #[inline(always)]
    pub fn scupspsrammastersel(&mut self) -> ScupspsrammasterselW<Scu918Spec> {
        ScupspsrammasterselW::new(self, 7)
    }
}
#[doc = "\\PSP\\ Service Processor Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu918::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu918::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu918Spec;
impl crate::RegisterSpec for Scu918Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu918::R`](R) reader structure"]
impl crate::Readable for Scu918Spec {}
#[doc = "`write(|w| ..)` method takes [`scu918::W`](W) writer structure"]
impl crate::Writable for Scu918Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU918 to value 0"]
impl crate::Resettable for Scu918Spec {}
