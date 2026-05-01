#[doc = "Register `SCU914` reader"]
pub type R = crate::R<Scu914Spec>;
#[doc = "Register `SCU914` writer"]
pub type W = crate::W<Scu914Spec>;
#[doc = "Field `SCUPSPICACHEINVALIDADDR` reader - SCU_PSP_I_CACHE_INVALID_ADDR"]
pub type ScupspicacheinvalidaddrR = crate::FieldReader<u16>;
#[doc = "Field `SCUPSPICACHEINVALIDADDR` writer - SCU_PSP_I_CACHE_INVALID_ADDR"]
pub type ScupspicacheinvalidaddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUPSPICACHEINVALIDCMD` reader - SCU_PSP_I_CACHE_INVALID_CMD"]
pub type ScupspicacheinvalidcmdR = crate::BitReader;
#[doc = "Field `SCUPSPICACHEINVALIDCMD` writer - SCU_PSP_I_CACHE_INVALID_CMD"]
pub type ScupspicacheinvalidcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUPSPDCACHEINVALIDADDR` reader - SCU_PSP_D_CACHE_INVALID_ADDR"]
pub type ScupspdcacheinvalidaddrR = crate::FieldReader<u16>;
#[doc = "Field `SCUPSPDCACHEINVALIDADDR` writer - SCU_PSP_D_CACHE_INVALID_ADDR"]
pub type ScupspdcacheinvalidaddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `SCUPSPDCACHEINVALIDCMD` reader - SCU_PSP_D_CACHE_INVALID_CMD"]
pub type ScupspdcacheinvalidcmdR = crate::BitReader;
#[doc = "Field `SCUPSPDCACHEINVALIDCMD` writer - SCU_PSP_D_CACHE_INVALID_CMD"]
pub type ScupspdcacheinvalidcmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - SCU_PSP_I_CACHE_INVALID_ADDR"]
    #[inline(always)]
    pub fn scupspicacheinvalidaddr(&self) -> ScupspicacheinvalidaddrR {
        ScupspicacheinvalidaddrR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:14 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SCU_PSP_I_CACHE_INVALID_CMD"]
    #[inline(always)]
    pub fn scupspicacheinvalidcmd(&self) -> ScupspicacheinvalidcmdR {
        ScupspicacheinvalidcmdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:28 - SCU_PSP_D_CACHE_INVALID_ADDR"]
    #[inline(always)]
    pub fn scupspdcacheinvalidaddr(&self) -> ScupspdcacheinvalidaddrR {
        ScupspdcacheinvalidaddrR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - SCU_PSP_D_CACHE_INVALID_CMD"]
    #[inline(always)]
    pub fn scupspdcacheinvalidcmd(&self) -> ScupspdcacheinvalidcmdR {
        ScupspdcacheinvalidcmdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - SCU_PSP_I_CACHE_INVALID_ADDR"]
    #[inline(always)]
    pub fn scupspicacheinvalidaddr(&mut self) -> ScupspicacheinvalidaddrW<Scu914Spec> {
        ScupspicacheinvalidaddrW::new(self, 0)
    }
    #[doc = "Bit 15 - SCU_PSP_I_CACHE_INVALID_CMD"]
    #[inline(always)]
    pub fn scupspicacheinvalidcmd(&mut self) -> ScupspicacheinvalidcmdW<Scu914Spec> {
        ScupspicacheinvalidcmdW::new(self, 15)
    }
    #[doc = "Bits 16:28 - SCU_PSP_D_CACHE_INVALID_ADDR"]
    #[inline(always)]
    pub fn scupspdcacheinvalidaddr(&mut self) -> ScupspdcacheinvalidaddrW<Scu914Spec> {
        ScupspdcacheinvalidaddrW::new(self, 16)
    }
    #[doc = "Bit 31 - SCU_PSP_D_CACHE_INVALID_CMD"]
    #[inline(always)]
    pub fn scupspdcacheinvalidcmd(&mut self) -> ScupspdcacheinvalidcmdW<Scu914Spec> {
        ScupspdcacheinvalidcmdW::new(self, 31)
    }
}
#[doc = "\\PSP\\ Service Processor Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu914::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu914::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu914Spec;
impl crate::RegisterSpec for Scu914Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu914::R`](R) reader structure"]
impl crate::Readable for Scu914Spec {}
#[doc = "`write(|w| ..)` method takes [`scu914::W`](W) writer structure"]
impl crate::Writable for Scu914Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU914 to value 0"]
impl crate::Resettable for Scu914Spec {}
