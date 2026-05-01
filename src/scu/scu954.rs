#[doc = "Register `SCU954` reader"]
pub type R = crate::R<Scu954Spec>;
#[doc = "Register `SCU954` writer"]
pub type W = crate::W<Scu954Spec>;
#[doc = "Field `SCUSSPICACHEINVALIDADDR` reader - SCU_SSP_I_CACHE_INVALID_ADDR"]
pub type ScusspicacheinvalidaddrR = crate::FieldReader<u16>;
#[doc = "Field `SCUSSPICACHEINVALIDADDR` writer - SCU_SSP_I_CACHE_INVALID_ADDR"]
pub type ScusspicacheinvalidaddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUSSPICACHEINVALIDCMD` reader - SCU_SSP_I_CACHE_INVALID_CMD"]
pub type ScusspicacheinvalidcmdR = crate::BitReader;
#[doc = "Field `SCUSSPICACHEINVALIDCMD` writer - SCU_SSP_I_CACHE_INVALID_CMD"]
pub type ScusspicacheinvalidcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSSPDCACHEINVALIDADDR` reader - SCU_SSP_D_CACHE_INVALID_ADDR"]
pub type ScusspdcacheinvalidaddrR = crate::FieldReader<u16>;
#[doc = "Field `SCUSSPDCACHEINVALIDADDR` writer - SCU_SSP_D_CACHE_INVALID_ADDR"]
pub type ScusspdcacheinvalidaddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `SCUSSPDCACHEINVALIDCMD` reader - SCU_SSP_D_CACHE_INVALID_CMD"]
pub type ScusspdcacheinvalidcmdR = crate::BitReader;
#[doc = "Field `SCUSSPDCACHEINVALIDCMD` writer - SCU_SSP_D_CACHE_INVALID_CMD"]
pub type ScusspdcacheinvalidcmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - SCU_SSP_I_CACHE_INVALID_ADDR"]
    #[inline(always)]
    pub fn scusspicacheinvalidaddr(&self) -> ScusspicacheinvalidaddrR {
        ScusspicacheinvalidaddrR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:14 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SCU_SSP_I_CACHE_INVALID_CMD"]
    #[inline(always)]
    pub fn scusspicacheinvalidcmd(&self) -> ScusspicacheinvalidcmdR {
        ScusspicacheinvalidcmdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:28 - SCU_SSP_D_CACHE_INVALID_ADDR"]
    #[inline(always)]
    pub fn scusspdcacheinvalidaddr(&self) -> ScusspdcacheinvalidaddrR {
        ScusspdcacheinvalidaddrR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - SCU_SSP_D_CACHE_INVALID_CMD"]
    #[inline(always)]
    pub fn scusspdcacheinvalidcmd(&self) -> ScusspdcacheinvalidcmdR {
        ScusspdcacheinvalidcmdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - SCU_SSP_I_CACHE_INVALID_ADDR"]
    #[inline(always)]
    pub fn scusspicacheinvalidaddr(&mut self) -> ScusspicacheinvalidaddrW<Scu954Spec> {
        ScusspicacheinvalidaddrW::new(self, 0)
    }
    #[doc = "Bit 15 - SCU_SSP_I_CACHE_INVALID_CMD"]
    #[inline(always)]
    pub fn scusspicacheinvalidcmd(&mut self) -> ScusspicacheinvalidcmdW<Scu954Spec> {
        ScusspicacheinvalidcmdW::new(self, 15)
    }
    #[doc = "Bits 16:28 - SCU_SSP_D_CACHE_INVALID_ADDR"]
    #[inline(always)]
    pub fn scusspdcacheinvalidaddr(&mut self) -> ScusspdcacheinvalidaddrW<Scu954Spec> {
        ScusspdcacheinvalidaddrW::new(self, 16)
    }
    #[doc = "Bit 31 - SCU_SSP_D_CACHE_INVALID_CMD"]
    #[inline(always)]
    pub fn scusspdcacheinvalidcmd(&mut self) -> ScusspdcacheinvalidcmdW<Scu954Spec> {
        ScusspdcacheinvalidcmdW::new(self, 31)
    }
}
#[doc = "\\SSP\\ Service Processor Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu954::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu954::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu954Spec;
impl crate::RegisterSpec for Scu954Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu954::R`](R) reader structure"]
impl crate::Readable for Scu954Spec {}
#[doc = "`write(|w| ..)` method takes [`scu954::W`](W) writer structure"]
impl crate::Writable for Scu954Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU954 to value 0"]
impl crate::Resettable for Scu954Spec {}
