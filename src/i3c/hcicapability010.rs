#[doc = "Register `HCICAPABILITY010` reader"]
pub type R = crate::R<Hcicapability010Spec>;
#[doc = "Register `HCICAPABILITY010` writer"]
pub type W = crate::W<Hcicapability010Spec>;
#[doc = "Field `REGSOFTRST` reader - REG_SOFT_RST"]
pub type RegsoftrstR = crate::BitReader;
#[doc = "Field `REGSOFTRST` writer - REG_SOFT_RST"]
pub type RegsoftrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGCMDQUEUERST` reader - REG_CMD_QUEUE_RST"]
pub type RegcmdqueuerstR = crate::BitReader;
#[doc = "Field `REGCMDQUEUERST` writer - REG_CMD_QUEUE_RST"]
pub type RegcmdqueuerstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRESPQUEUERST` reader - REG_RESP_QUEUE_RST"]
pub type RegrespqueuerstR = crate::BitReader;
#[doc = "Field `REGRESPQUEUERST` writer - REG_RESP_QUEUE_RST"]
pub type RegrespqueuerstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTXFIFORST` reader - REG_TX_FIFO_RST"]
pub type RegtxfiforstR = crate::BitReader;
#[doc = "Field `REGTXFIFORST` writer - REG_TX_FIFO_RST"]
pub type RegtxfiforstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRXFIFORST` reader - REG_RX_FIFO_RST"]
pub type RegrxfiforstR = crate::BitReader;
#[doc = "Field `REGRXFIFORST` writer - REG_RX_FIFO_RST"]
pub type RegrxfiforstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIQUEUERST` reader - REG_IBI_QUEUE_RST"]
pub type RegibiqueuerstR = crate::BitReader;
#[doc = "Field `REGIBIQUEUERST` writer - REG_IBI_QUEUE_RST"]
pub type RegibiqueuerstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_SOFT_RST"]
    #[inline(always)]
    pub fn regsoftrst(&self) -> RegsoftrstR {
        RegsoftrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_CMD_QUEUE_RST"]
    #[inline(always)]
    pub fn regcmdqueuerst(&self) -> RegcmdqueuerstR {
        RegcmdqueuerstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_RESP_QUEUE_RST"]
    #[inline(always)]
    pub fn regrespqueuerst(&self) -> RegrespqueuerstR {
        RegrespqueuerstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REG_TX_FIFO_RST"]
    #[inline(always)]
    pub fn regtxfiforst(&self) -> RegtxfiforstR {
        RegtxfiforstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REG_RX_FIFO_RST"]
    #[inline(always)]
    pub fn regrxfiforst(&self) -> RegrxfiforstR {
        RegrxfiforstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_IBI_QUEUE_RST"]
    #[inline(always)]
    pub fn regibiqueuerst(&self) -> RegibiqueuerstR {
        RegibiqueuerstR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_SOFT_RST"]
    #[inline(always)]
    pub fn regsoftrst(&mut self) -> RegsoftrstW<Hcicapability010Spec> {
        RegsoftrstW::new(self, 0)
    }
    #[doc = "Bit 1 - REG_CMD_QUEUE_RST"]
    #[inline(always)]
    pub fn regcmdqueuerst(&mut self) -> RegcmdqueuerstW<Hcicapability010Spec> {
        RegcmdqueuerstW::new(self, 1)
    }
    #[doc = "Bit 2 - REG_RESP_QUEUE_RST"]
    #[inline(always)]
    pub fn regrespqueuerst(&mut self) -> RegrespqueuerstW<Hcicapability010Spec> {
        RegrespqueuerstW::new(self, 2)
    }
    #[doc = "Bit 3 - REG_TX_FIFO_RST"]
    #[inline(always)]
    pub fn regtxfiforst(&mut self) -> RegtxfiforstW<Hcicapability010Spec> {
        RegtxfiforstW::new(self, 3)
    }
    #[doc = "Bit 4 - REG_RX_FIFO_RST"]
    #[inline(always)]
    pub fn regrxfiforst(&mut self) -> RegrxfiforstW<Hcicapability010Spec> {
        RegrxfiforstW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_IBI_QUEUE_RST"]
    #[inline(always)]
    pub fn regibiqueuerst(&mut self) -> RegibiqueuerstW<Hcicapability010Spec> {
        RegibiqueuerstW::new(self, 5)
    }
}
#[doc = "RESET\\_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability010Spec;
impl crate::RegisterSpec for Hcicapability010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability010::R`](R) reader structure"]
impl crate::Readable for Hcicapability010Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability010::W`](W) writer structure"]
impl crate::Writable for Hcicapability010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY010 to value 0"]
impl crate::Resettable for Hcicapability010Spec {}
