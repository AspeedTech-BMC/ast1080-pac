#[doc = "Register `HCIDATSINGLE000` reader"]
pub type R = crate::R<Hcidatsingle000Spec>;
#[doc = "Register `HCIDATSINGLE000` writer"]
pub type W = crate::W<Hcidatsingle000Spec>;
#[doc = "Field `REGTARGETSTATICADDRESS` reader - REG_TARGET_STATIC_ADDRESS"]
pub type RegtargetstaticaddressR = crate::FieldReader;
#[doc = "Field `REGTARGETSTATICADDRESS` writer - REG_TARGET_STATIC_ADDRESS"]
pub type RegtargetstaticaddressW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGTARGETIBIPAYLOAD` reader - REG_TARGET_IBI_PAYLOAD"]
pub type RegtargetibipayloadR = crate::BitReader;
#[doc = "Field `REGTARGETIBIPAYLOAD` writer - REG_TARGET_IBI_PAYLOAD"]
pub type RegtargetibipayloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTARGETIBIREJECT` reader - REG_TARGET_IBI_REJECT"]
pub type RegtargetibirejectR = crate::BitReader;
#[doc = "Field `REGTARGETIBIREJECT` writer - REG_TARGET_IBI_REJECT"]
pub type RegtargetibirejectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTARGETCRRREJECT` reader - REG_TARGET_CRR_REJECT"]
pub type RegtargetcrrrejectR = crate::BitReader;
#[doc = "Field `REGTARGETCRRREJECT` writer - REG_TARGET_CRR_REJECT"]
pub type RegtargetcrrrejectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTARGETDEVNACKRETRYCNT` reader - REG_TARGET_DEV_NACK_RETRY_CNT"]
pub type RegtargetdevnackretrycntR = crate::FieldReader;
#[doc = "Field `REGTARGETDEVNACKRETRYCNT` writer - REG_TARGET_DEV_NACK_RETRY_CNT"]
pub type RegtargetdevnackretrycntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGTARGETDEVICE` reader - REG_TARGET_DEVICE"]
pub type RegtargetdeviceR = crate::BitReader;
#[doc = "Field `REGTARGETDEVICE` writer - REG_TARGET_DEVICE"]
pub type RegtargetdeviceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - REG_TARGET_STATIC_ADDRESS"]
    #[inline(always)]
    pub fn regtargetstaticaddress(&self) -> RegtargetstaticaddressR {
        RegtargetstaticaddressR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - REG_TARGET_IBI_PAYLOAD"]
    #[inline(always)]
    pub fn regtargetibipayload(&self) -> RegtargetibipayloadR {
        RegtargetibipayloadR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REG_TARGET_IBI_REJECT"]
    #[inline(always)]
    pub fn regtargetibireject(&self) -> RegtargetibirejectR {
        RegtargetibirejectR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - REG_TARGET_CRR_REJECT"]
    #[inline(always)]
    pub fn regtargetcrrreject(&self) -> RegtargetcrrrejectR {
        RegtargetcrrrejectR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 29:30 - REG_TARGET_DEV_NACK_RETRY_CNT"]
    #[inline(always)]
    pub fn regtargetdevnackretrycnt(&self) -> RegtargetdevnackretrycntR {
        RegtargetdevnackretrycntR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - REG_TARGET_DEVICE"]
    #[inline(always)]
    pub fn regtargetdevice(&self) -> RegtargetdeviceR {
        RegtargetdeviceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - REG_TARGET_STATIC_ADDRESS"]
    #[inline(always)]
    pub fn regtargetstaticaddress(&mut self) -> RegtargetstaticaddressW<Hcidatsingle000Spec> {
        RegtargetstaticaddressW::new(self, 0)
    }
    #[doc = "Bit 12 - REG_TARGET_IBI_PAYLOAD"]
    #[inline(always)]
    pub fn regtargetibipayload(&mut self) -> RegtargetibipayloadW<Hcidatsingle000Spec> {
        RegtargetibipayloadW::new(self, 12)
    }
    #[doc = "Bit 13 - REG_TARGET_IBI_REJECT"]
    #[inline(always)]
    pub fn regtargetibireject(&mut self) -> RegtargetibirejectW<Hcidatsingle000Spec> {
        RegtargetibirejectW::new(self, 13)
    }
    #[doc = "Bit 14 - REG_TARGET_CRR_REJECT"]
    #[inline(always)]
    pub fn regtargetcrrreject(&mut self) -> RegtargetcrrrejectW<Hcidatsingle000Spec> {
        RegtargetcrrrejectW::new(self, 14)
    }
    #[doc = "Bits 29:30 - REG_TARGET_DEV_NACK_RETRY_CNT"]
    #[inline(always)]
    pub fn regtargetdevnackretrycnt(&mut self) -> RegtargetdevnackretrycntW<Hcidatsingle000Spec> {
        RegtargetdevnackretrycntW::new(self, 29)
    }
    #[doc = "Bit 31 - REG_TARGET_DEVICE"]
    #[inline(always)]
    pub fn regtargetdevice(&mut self) -> RegtargetdeviceW<Hcidatsingle000Spec> {
        RegtargetdeviceW::new(self, 31)
    }
}
#[doc = "TARGET\\_DAT\n\nYou can [`read`](crate::Reg::read) this register and get [`hcidatsingle000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcidatsingle000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcidatsingle000Spec;
impl crate::RegisterSpec for Hcidatsingle000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcidatsingle000::R`](R) reader structure"]
impl crate::Readable for Hcidatsingle000Spec {}
#[doc = "`write(|w| ..)` method takes [`hcidatsingle000::W`](W) writer structure"]
impl crate::Writable for Hcidatsingle000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIDATSINGLE000 to value 0"]
impl crate::Resettable for Hcidatsingle000Spec {}
