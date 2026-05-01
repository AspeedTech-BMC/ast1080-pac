#[doc = "Register `HCIRHS044` reader"]
pub type R = crate::R<Hcirhs044Spec>;
#[doc = "Register `HCIRHS044` writer"]
pub type W = crate::W<Hcirhs044Spec>;
#[doc = "Field `REGTRANSFERABORTSTATEN` reader - REG_TRANSFER_ABORT_STAT_EN"]
pub type RegtransferabortstatenR = crate::BitReader;
#[doc = "Field `REGTRANSFERABORTSTATEN` writer - REG_TRANSFER_ABORT_STAT_EN"]
pub type RegtransferabortstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIRINGFULLSTATEN` reader - REG_IBI_RING_FULL_STAT_EN"]
pub type RegibiringfullstatenR = crate::BitReader;
#[doc = "Field `REGIBIRINGFULLSTATEN` writer - REG_IBI_RING_FULL_STAT_EN"]
pub type RegibiringfullstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGTRANSFERERRSTATEN` reader - REG_TRANSFER_ERR_STAT_EN"]
pub type RegtransfererrstatenR = crate::BitReader;
#[doc = "Field `REGTRANSFERERRSTATEN` writer - REG_TRANSFER_ERR_STAT_EN"]
pub type RegtransfererrstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRINGOPSTATEN` reader - REG_RING_OP_STAT_EN"]
pub type RegringopstatenR = crate::BitReader;
#[doc = "Field `REGRINGOPSTATEN` writer - REG_RING_OP_STAT_EN"]
pub type RegringopstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTRANSFERCOMPLETIONSTATEN` reader - REG_TRANSFER_COMPLETION_STAT_EN"]
pub type RegtransfercompletionstatenR = crate::BitReader;
#[doc = "Field `REGTRANSFERCOMPLETIONSTATEN` writer - REG_TRANSFER_COMPLETION_STAT_EN"]
pub type RegtransfercompletionstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGIBIREADYSTATEN` reader - REG_IBI_READY_STAT_EN"]
pub type RegibireadystatenR = crate::BitReader;
#[doc = "Field `REGIBIREADYSTATEN` writer - REG_IBI_READY_STAT_EN"]
pub type RegibireadystatenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - REG_TRANSFER_ABORT_STAT_EN"]
    #[inline(always)]
    pub fn regtransferabortstaten(&self) -> RegtransferabortstatenR {
        RegtransferabortstatenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_IBI_RING_FULL_STAT_EN"]
    #[inline(always)]
    pub fn regibiringfullstaten(&self) -> RegibiringfullstatenR {
        RegibiringfullstatenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - REG_TRANSFER_ERR_STAT_EN"]
    #[inline(always)]
    pub fn regtransfererrstaten(&self) -> RegtransfererrstatenR {
        RegtransfererrstatenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_RING_OP_STAT_EN"]
    #[inline(always)]
    pub fn regringopstaten(&self) -> RegringopstatenR {
        RegringopstatenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_TRANSFER_COMPLETION_STAT_EN"]
    #[inline(always)]
    pub fn regtransfercompletionstaten(&self) -> RegtransfercompletionstatenR {
        RegtransfercompletionstatenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REG_IBI_READY_STAT_EN"]
    #[inline(always)]
    pub fn regibireadystaten(&self) -> RegibireadystatenR {
        RegibireadystatenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - REG_TRANSFER_ABORT_STAT_EN"]
    #[inline(always)]
    pub fn regtransferabortstaten(&mut self) -> RegtransferabortstatenW<Hcirhs044Spec> {
        RegtransferabortstatenW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_IBI_RING_FULL_STAT_EN"]
    #[inline(always)]
    pub fn regibiringfullstaten(&mut self) -> RegibiringfullstatenW<Hcirhs044Spec> {
        RegibiringfullstatenW::new(self, 6)
    }
    #[doc = "Bit 9 - REG_TRANSFER_ERR_STAT_EN"]
    #[inline(always)]
    pub fn regtransfererrstaten(&mut self) -> RegtransfererrstatenW<Hcirhs044Spec> {
        RegtransfererrstatenW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_RING_OP_STAT_EN"]
    #[inline(always)]
    pub fn regringopstaten(&mut self) -> RegringopstatenW<Hcirhs044Spec> {
        RegringopstatenW::new(self, 10)
    }
    #[doc = "Bit 11 - REG_TRANSFER_COMPLETION_STAT_EN"]
    #[inline(always)]
    pub fn regtransfercompletionstaten(&mut self) -> RegtransfercompletionstatenW<Hcirhs044Spec> {
        RegtransfercompletionstatenW::new(self, 11)
    }
    #[doc = "Bit 12 - REG_IBI_READY_STAT_EN"]
    #[inline(always)]
    pub fn regibireadystaten(&mut self) -> RegibireadystatenW<Hcirhs044Spec> {
        RegibireadystatenW::new(self, 12)
    }
}
#[doc = "RH\\_INTR\\_STATUS\\_ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs044Spec;
impl crate::RegisterSpec for Hcirhs044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs044::R`](R) reader structure"]
impl crate::Readable for Hcirhs044Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs044::W`](W) writer structure"]
impl crate::Writable for Hcirhs044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS044 to value 0"]
impl crate::Resettable for Hcirhs044Spec {}
