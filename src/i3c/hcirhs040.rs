#[doc = "Register `HCIRHS040` reader"]
pub type R = crate::R<Hcirhs040Spec>;
#[doc = "Register `HCIRHS040` writer"]
pub type W = crate::W<Hcirhs040Spec>;
#[doc = "Field `REGTRANSFERABORTSTAT` reader - REG_TRANSFER_ABORT_STAT"]
pub type RegtransferabortstatR = crate::BitReader;
#[doc = "Field `REGTRANSFERABORTSTAT` writer - REG_TRANSFER_ABORT_STAT"]
pub type RegtransferabortstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIRINGFULLSTAT` reader - REG_IBI_RING_FULL_STAT"]
pub type RegibiringfullstatR = crate::BitReader;
#[doc = "Field `REGIBIRINGFULLSTAT` writer - REG_IBI_RING_FULL_STAT"]
pub type RegibiringfullstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGTRANSFERERRSTAT` reader - REG_TRANSFER_ERR_STAT"]
pub type RegtransfererrstatR = crate::BitReader;
#[doc = "Field `REGTRANSFERERRSTAT` writer - REG_TRANSFER_ERR_STAT"]
pub type RegtransfererrstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRINGOPSTAT` reader - REG_RING_OP_STAT"]
pub type RegringopstatR = crate::BitReader;
#[doc = "Field `REGRINGOPSTAT` writer - REG_RING_OP_STAT"]
pub type RegringopstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTRANSFERCOMPLETIONSTAT` reader - REG_TRANSFER_COMPLETION_STAT"]
pub type RegtransfercompletionstatR = crate::BitReader;
#[doc = "Field `REGTRANSFERCOMPLETIONSTAT` writer - REG_TRANSFER_COMPLETION_STAT"]
pub type RegtransfercompletionstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIREADYSTAT` reader - REG_IBI_READY_STAT"]
pub type RegibireadystatR = crate::BitReader;
#[doc = "Field `REGIBIREADYSTAT` writer - REG_IBI_READY_STAT"]
pub type RegibireadystatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - REG_TRANSFER_ABORT_STAT"]
    #[inline(always)]
    pub fn regtransferabortstat(&self) -> RegtransferabortstatR {
        RegtransferabortstatR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_IBI_RING_FULL_STAT"]
    #[inline(always)]
    pub fn regibiringfullstat(&self) -> RegibiringfullstatR {
        RegibiringfullstatR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - REG_TRANSFER_ERR_STAT"]
    #[inline(always)]
    pub fn regtransfererrstat(&self) -> RegtransfererrstatR {
        RegtransfererrstatR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_RING_OP_STAT"]
    #[inline(always)]
    pub fn regringopstat(&self) -> RegringopstatR {
        RegringopstatR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_TRANSFER_COMPLETION_STAT"]
    #[inline(always)]
    pub fn regtransfercompletionstat(&self) -> RegtransfercompletionstatR {
        RegtransfercompletionstatR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REG_IBI_READY_STAT"]
    #[inline(always)]
    pub fn regibireadystat(&self) -> RegibireadystatR {
        RegibireadystatR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - REG_TRANSFER_ABORT_STAT"]
    #[inline(always)]
    pub fn regtransferabortstat(&mut self) -> RegtransferabortstatW<Hcirhs040Spec> {
        RegtransferabortstatW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_IBI_RING_FULL_STAT"]
    #[inline(always)]
    pub fn regibiringfullstat(&mut self) -> RegibiringfullstatW<Hcirhs040Spec> {
        RegibiringfullstatW::new(self, 6)
    }
    #[doc = "Bit 9 - REG_TRANSFER_ERR_STAT"]
    #[inline(always)]
    pub fn regtransfererrstat(&mut self) -> RegtransfererrstatW<Hcirhs040Spec> {
        RegtransfererrstatW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_RING_OP_STAT"]
    #[inline(always)]
    pub fn regringopstat(&mut self) -> RegringopstatW<Hcirhs040Spec> {
        RegringopstatW::new(self, 10)
    }
    #[doc = "Bit 11 - REG_TRANSFER_COMPLETION_STAT"]
    #[inline(always)]
    pub fn regtransfercompletionstat(&mut self) -> RegtransfercompletionstatW<Hcirhs040Spec> {
        RegtransfercompletionstatW::new(self, 11)
    }
    #[doc = "Bit 12 - REG_IBI_READY_STAT"]
    #[inline(always)]
    pub fn regibireadystat(&mut self) -> RegibireadystatW<Hcirhs040Spec> {
        RegibireadystatW::new(self, 12)
    }
}
#[doc = "RH\\_INTR\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs040Spec;
impl crate::RegisterSpec for Hcirhs040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs040::R`](R) reader structure"]
impl crate::Readable for Hcirhs040Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs040::W`](W) writer structure"]
impl crate::Writable for Hcirhs040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS040 to value 0"]
impl crate::Resettable for Hcirhs040Spec {}
