#[doc = "Register `HCIRHS04C` reader"]
pub type R = crate::R<Hcirhs04cSpec>;
#[doc = "Register `HCIRHS04C` writer"]
pub type W = crate::W<Hcirhs04cSpec>;
#[doc = "Field `REGTRANSFERABORTFORCE` reader - REG_TRANSFER_ABORT_FORCE"]
pub type RegtransferabortforceR = crate::BitReader;
#[doc = "Field `REGTRANSFERABORTFORCE` writer - REG_TRANSFER_ABORT_FORCE"]
pub type RegtransferabortforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIRINGFULLFORCE` reader - REG_IBI_RING_FULL_FORCE"]
pub type RegibiringfullforceR = crate::BitReader;
#[doc = "Field `REGIBIRINGFULLFORCE` writer - REG_IBI_RING_FULL_FORCE"]
pub type RegibiringfullforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGTRANSFERERRFORCE` reader - REG_TRANSFER_ERR_FORCE"]
pub type RegtransfererrforceR = crate::BitReader;
#[doc = "Field `REGTRANSFERERRFORCE` writer - REG_TRANSFER_ERR_FORCE"]
pub type RegtransfererrforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRINGOPFORCE` reader - REG_RING_OP_FORCE"]
pub type RegringopforceR = crate::BitReader;
#[doc = "Field `REGRINGOPFORCE` writer - REG_RING_OP_FORCE"]
pub type RegringopforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTRANSFERCOMPLETIONFORCE` reader - REG_TRANSFER_COMPLETION_FORCE"]
pub type RegtransfercompletionforceR = crate::BitReader;
#[doc = "Field `REGTRANSFERCOMPLETIONFORCE` writer - REG_TRANSFER_COMPLETION_FORCE"]
pub type RegtransfercompletionforceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIREADYFORCE` reader - REG_IBI_READY_FORCE"]
pub type RegibireadyforceR = crate::BitReader;
#[doc = "Field `REGIBIREADYFORCE` writer - REG_IBI_READY_FORCE"]
pub type RegibireadyforceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - REG_TRANSFER_ABORT_FORCE"]
    #[inline(always)]
    pub fn regtransferabortforce(&self) -> RegtransferabortforceR {
        RegtransferabortforceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_IBI_RING_FULL_FORCE"]
    #[inline(always)]
    pub fn regibiringfullforce(&self) -> RegibiringfullforceR {
        RegibiringfullforceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - REG_TRANSFER_ERR_FORCE"]
    #[inline(always)]
    pub fn regtransfererrforce(&self) -> RegtransfererrforceR {
        RegtransfererrforceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_RING_OP_FORCE"]
    #[inline(always)]
    pub fn regringopforce(&self) -> RegringopforceR {
        RegringopforceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_TRANSFER_COMPLETION_FORCE"]
    #[inline(always)]
    pub fn regtransfercompletionforce(&self) -> RegtransfercompletionforceR {
        RegtransfercompletionforceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REG_IBI_READY_FORCE"]
    #[inline(always)]
    pub fn regibireadyforce(&self) -> RegibireadyforceR {
        RegibireadyforceR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - REG_TRANSFER_ABORT_FORCE"]
    #[inline(always)]
    pub fn regtransferabortforce(&mut self) -> RegtransferabortforceW<Hcirhs04cSpec> {
        RegtransferabortforceW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_IBI_RING_FULL_FORCE"]
    #[inline(always)]
    pub fn regibiringfullforce(&mut self) -> RegibiringfullforceW<Hcirhs04cSpec> {
        RegibiringfullforceW::new(self, 6)
    }
    #[doc = "Bit 9 - REG_TRANSFER_ERR_FORCE"]
    #[inline(always)]
    pub fn regtransfererrforce(&mut self) -> RegtransfererrforceW<Hcirhs04cSpec> {
        RegtransfererrforceW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_RING_OP_FORCE"]
    #[inline(always)]
    pub fn regringopforce(&mut self) -> RegringopforceW<Hcirhs04cSpec> {
        RegringopforceW::new(self, 10)
    }
    #[doc = "Bit 11 - REG_TRANSFER_COMPLETION_FORCE"]
    #[inline(always)]
    pub fn regtransfercompletionforce(&mut self) -> RegtransfercompletionforceW<Hcirhs04cSpec> {
        RegtransfercompletionforceW::new(self, 11)
    }
    #[doc = "Bit 12 - REG_IBI_READY_FORCE"]
    #[inline(always)]
    pub fn regibireadyforce(&mut self) -> RegibireadyforceW<Hcirhs04cSpec> {
        RegibireadyforceW::new(self, 12)
    }
}
#[doc = "RH\\_INTR\\_FORCE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs04cSpec;
impl crate::RegisterSpec for Hcirhs04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs04c::R`](R) reader structure"]
impl crate::Readable for Hcirhs04cSpec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs04c::W`](W) writer structure"]
impl crate::Writable for Hcirhs04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS04C to value 0"]
impl crate::Resettable for Hcirhs04cSpec {}
