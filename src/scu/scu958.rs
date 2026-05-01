#[doc = "Register `SCU958` reader"]
pub type R = crate::R<Scu958Spec>;
#[doc = "Register `SCU958` writer"]
pub type W = crate::W<Scu958Spec>;
#[doc = "Field `SCUSSPDCACHEENABLE` reader - SCU_SSP_D_CACHE_ENABLE"]
pub type ScusspdcacheenableR = crate::BitReader;
#[doc = "Field `SCUSSPDCACHEENABLE` writer - SCU_SSP_D_CACHE_ENABLE"]
pub type ScusspdcacheenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPICACHEENABLE` reader - SCU_SSP_I_CACHE_ENABLE"]
pub type ScusspicacheenableR = crate::BitReader;
#[doc = "Field `SCUSSPICACHEENABLE` writer - SCU_SSP_I_CACHE_ENABLE"]
pub type ScusspicacheenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPDCACHERESET` reader - SCU_SSP_D_CACHE_RESET"]
pub type ScusspdcacheresetR = crate::BitReader;
#[doc = "Field `SCUSSPDCACHERESET` writer - SCU_SSP_D_CACHE_RESET"]
pub type ScusspdcacheresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPICACHERESET` reader - SCU_SSP_I_CACHE_RESET"]
pub type ScusspicacheresetR = crate::BitReader;
#[doc = "Field `SCUSSPICACHERESET` writer - SCU_SSP_I_CACHE_RESET"]
pub type ScusspicacheresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPDPREFETCHCLR` reader - SCU_SSP_D_PREFETCH_CLR"]
pub type ScusspdprefetchclrR = crate::BitReader;
#[doc = "Field `SCUSSPDPREFETCHCLR` writer - SCU_SSP_D_PREFETCH_CLR"]
pub type ScusspdprefetchclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPIPREFETCHCLR` reader - SCU_SSP_I_PREFETCH_CLR"]
pub type ScusspiprefetchclrR = crate::BitReader;
#[doc = "Field `SCUSSPIPREFETCHCLR` writer - SCU_SSP_I_PREFETCH_CLR"]
pub type ScusspiprefetchclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPSRAMMODEEN` reader - SCU_SSP_SRAM_MODE_EN"]
pub type ScusspsrammodeenR = crate::BitReader;
#[doc = "Field `SCUSSPSRAMMODEEN` writer - SCU_SSP_SRAM_MODE_EN"]
pub type ScusspsrammodeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPSRAMMASTERSEL` reader - SCU_SSP_SRAM_MASTER_SEL"]
pub type ScusspsrammasterselR = crate::BitReader;
#[doc = "Field `SCUSSPSRAMMASTERSEL` writer - SCU_SSP_SRAM_MASTER_SEL"]
pub type ScusspsrammasterselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_SSP_D_CACHE_ENABLE"]
    #[inline(always)]
    pub fn scusspdcacheenable(&self) -> ScusspdcacheenableR {
        ScusspdcacheenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_SSP_I_CACHE_ENABLE"]
    #[inline(always)]
    pub fn scusspicacheenable(&self) -> ScusspicacheenableR {
        ScusspicacheenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_SSP_D_CACHE_RESET"]
    #[inline(always)]
    pub fn scusspdcachereset(&self) -> ScusspdcacheresetR {
        ScusspdcacheresetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_SSP_I_CACHE_RESET"]
    #[inline(always)]
    pub fn scusspicachereset(&self) -> ScusspicacheresetR {
        ScusspicacheresetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_SSP_D_PREFETCH_CLR"]
    #[inline(always)]
    pub fn scusspdprefetchclr(&self) -> ScusspdprefetchclrR {
        ScusspdprefetchclrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_SSP_I_PREFETCH_CLR"]
    #[inline(always)]
    pub fn scusspiprefetchclr(&self) -> ScusspiprefetchclrR {
        ScusspiprefetchclrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_SSP_SRAM_MODE_EN"]
    #[inline(always)]
    pub fn scusspsrammodeen(&self) -> ScusspsrammodeenR {
        ScusspsrammodeenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_SSP_SRAM_MASTER_SEL"]
    #[inline(always)]
    pub fn scusspsrammastersel(&self) -> ScusspsrammasterselR {
        ScusspsrammasterselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_SSP_D_CACHE_ENABLE"]
    #[inline(always)]
    pub fn scusspdcacheenable(&mut self) -> ScusspdcacheenableW<Scu958Spec> {
        ScusspdcacheenableW::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_SSP_I_CACHE_ENABLE"]
    #[inline(always)]
    pub fn scusspicacheenable(&mut self) -> ScusspicacheenableW<Scu958Spec> {
        ScusspicacheenableW::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_SSP_D_CACHE_RESET"]
    #[inline(always)]
    pub fn scusspdcachereset(&mut self) -> ScusspdcacheresetW<Scu958Spec> {
        ScusspdcacheresetW::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_SSP_I_CACHE_RESET"]
    #[inline(always)]
    pub fn scusspicachereset(&mut self) -> ScusspicacheresetW<Scu958Spec> {
        ScusspicacheresetW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_SSP_D_PREFETCH_CLR"]
    #[inline(always)]
    pub fn scusspdprefetchclr(&mut self) -> ScusspdprefetchclrW<Scu958Spec> {
        ScusspdprefetchclrW::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_SSP_I_PREFETCH_CLR"]
    #[inline(always)]
    pub fn scusspiprefetchclr(&mut self) -> ScusspiprefetchclrW<Scu958Spec> {
        ScusspiprefetchclrW::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_SSP_SRAM_MODE_EN"]
    #[inline(always)]
    pub fn scusspsrammodeen(&mut self) -> ScusspsrammodeenW<Scu958Spec> {
        ScusspsrammodeenW::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_SSP_SRAM_MASTER_SEL"]
    #[inline(always)]
    pub fn scusspsrammastersel(&mut self) -> ScusspsrammasterselW<Scu958Spec> {
        ScusspsrammasterselW::new(self, 7)
    }
}
#[doc = "\\SSP\\ Service Processor Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu958::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu958::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu958Spec;
impl crate::RegisterSpec for Scu958Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu958::R`](R) reader structure"]
impl crate::Readable for Scu958Spec {}
#[doc = "`write(|w| ..)` method takes [`scu958::W`](W) writer structure"]
impl crate::Writable for Scu958Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU958 to value 0"]
impl crate::Resettable for Scu958Spec {}
