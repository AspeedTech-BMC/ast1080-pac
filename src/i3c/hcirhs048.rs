#[doc = "Register `HCIRHS048` reader"]
pub type R = crate::R<Hcirhs048Spec>;
#[doc = "Register `HCIRHS048` writer"]
pub type W = crate::W<Hcirhs048Spec>;
#[doc = "Field `REGTRANSFERABORTSIGNALEN` reader - REG_TRANSFER_ABORT_SIGNAL_EN"]
pub type RegtransferabortsignalenR = crate::BitReader;
#[doc = "Field `REGTRANSFERABORTSIGNALEN` writer - REG_TRANSFER_ABORT_SIGNAL_EN"]
pub type RegtransferabortsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIRINGFULLSIGNALEN` reader - REG_IBI_RING_FULL_SIGNAL_EN"]
pub type RegibiringfullsignalenR = crate::BitReader;
#[doc = "Field `REGIBIRINGFULLSIGNALEN` writer - REG_IBI_RING_FULL_SIGNAL_EN"]
pub type RegibiringfullsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGTRANSFERERRSIGNALEN` reader - REG_TRANSFER_ERR_SIGNAL_EN"]
pub type RegtransfererrsignalenR = crate::BitReader;
#[doc = "Field `REGTRANSFERERRSIGNALEN` writer - REG_TRANSFER_ERR_SIGNAL_EN"]
pub type RegtransfererrsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRINGOPSIGNALEN` reader - REG_RING_OP_SIGNAL_EN"]
pub type RegringopsignalenR = crate::BitReader;
#[doc = "Field `REGRINGOPSIGNALEN` writer - REG_RING_OP_SIGNAL_EN"]
pub type RegringopsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTRANSFERCOMPLETIONSIGNALEN` reader - REG_TRANSFER_COMPLETION_SIGNAL_EN"]
pub type RegtransfercompletionsignalenR = crate::BitReader;
#[doc = "Field `REGTRANSFERCOMPLETIONSIGNALEN` writer - REG_TRANSFER_COMPLETION_SIGNAL_EN"]
pub type RegtransfercompletionsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIREADYSIGNALEN` reader - REG_IBI_READY_SIGNAL_EN"]
pub type RegibireadysignalenR = crate::BitReader;
#[doc = "Field `REGIBIREADYSIGNALEN` writer - REG_IBI_READY_SIGNAL_EN"]
pub type RegibireadysignalenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - REG_TRANSFER_ABORT_SIGNAL_EN"]
    #[inline(always)]
    pub fn regtransferabortsignalen(&self) -> RegtransferabortsignalenR {
        RegtransferabortsignalenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_IBI_RING_FULL_SIGNAL_EN"]
    #[inline(always)]
    pub fn regibiringfullsignalen(&self) -> RegibiringfullsignalenR {
        RegibiringfullsignalenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - REG_TRANSFER_ERR_SIGNAL_EN"]
    #[inline(always)]
    pub fn regtransfererrsignalen(&self) -> RegtransfererrsignalenR {
        RegtransfererrsignalenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_RING_OP_SIGNAL_EN"]
    #[inline(always)]
    pub fn regringopsignalen(&self) -> RegringopsignalenR {
        RegringopsignalenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_TRANSFER_COMPLETION_SIGNAL_EN"]
    #[inline(always)]
    pub fn regtransfercompletionsignalen(&self) -> RegtransfercompletionsignalenR {
        RegtransfercompletionsignalenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REG_IBI_READY_SIGNAL_EN"]
    #[inline(always)]
    pub fn regibireadysignalen(&self) -> RegibireadysignalenR {
        RegibireadysignalenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - REG_TRANSFER_ABORT_SIGNAL_EN"]
    #[inline(always)]
    pub fn regtransferabortsignalen(&mut self) -> RegtransferabortsignalenW<Hcirhs048Spec> {
        RegtransferabortsignalenW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_IBI_RING_FULL_SIGNAL_EN"]
    #[inline(always)]
    pub fn regibiringfullsignalen(&mut self) -> RegibiringfullsignalenW<Hcirhs048Spec> {
        RegibiringfullsignalenW::new(self, 6)
    }
    #[doc = "Bit 9 - REG_TRANSFER_ERR_SIGNAL_EN"]
    #[inline(always)]
    pub fn regtransfererrsignalen(&mut self) -> RegtransfererrsignalenW<Hcirhs048Spec> {
        RegtransfererrsignalenW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_RING_OP_SIGNAL_EN"]
    #[inline(always)]
    pub fn regringopsignalen(&mut self) -> RegringopsignalenW<Hcirhs048Spec> {
        RegringopsignalenW::new(self, 10)
    }
    #[doc = "Bit 11 - REG_TRANSFER_COMPLETION_SIGNAL_EN"]
    #[inline(always)]
    pub fn regtransfercompletionsignalen(
        &mut self,
    ) -> RegtransfercompletionsignalenW<Hcirhs048Spec> {
        RegtransfercompletionsignalenW::new(self, 11)
    }
    #[doc = "Bit 12 - REG_IBI_READY_SIGNAL_EN"]
    #[inline(always)]
    pub fn regibireadysignalen(&mut self) -> RegibireadysignalenW<Hcirhs048Spec> {
        RegibireadysignalenW::new(self, 12)
    }
}
#[doc = "RH\\_INTR\\_SIGNAL\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs048Spec;
impl crate::RegisterSpec for Hcirhs048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs048::R`](R) reader structure"]
impl crate::Readable for Hcirhs048Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs048::W`](W) writer structure"]
impl crate::Writable for Hcirhs048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS048 to value 0"]
impl crate::Resettable for Hcirhs048Spec {}
