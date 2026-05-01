#[doc = "Register `I3CPHYCTRLREG0E0` reader"]
pub type R = crate::R<I3cphyctrlreg0e0Spec>;
#[doc = "Register `I3CPHYCTRLREG0E0` writer"]
pub type W = crate::W<I3cphyctrlreg0e0Spec>;
#[doc = "Field `REGI3CSPECIALPATTERNSETUPCNT` reader - REG_I3C_SPECIAL_PATTERN_SETUP_CNT"]
pub type Regi3cspecialpatternsetupcntR = crate::FieldReader;
#[doc = "Field `REGI3CSPECIALPATTERNSETUPCNT` writer - REG_I3C_SPECIAL_PATTERN_SETUP_CNT"]
pub type Regi3cspecialpatternsetupcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGI3CSPECIALPATTERNSDATRANCNT` reader - REG_I3C_SPECIAL_PATTERN_SDA_TRAN_CNT"]
pub type Regi3cspecialpatternsdatrancntR = crate::FieldReader;
#[doc = "Field `REGI3CSPECIALPATTERNSDATRANCNT` writer - REG_I3C_SPECIAL_PATTERN_SDA_TRAN_CNT"]
pub type Regi3cspecialpatternsdatrancntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGTGTOCRHANDOFFWAITCNT` reader - REG_TG_TO_CR_HANDOFF_WAIT_CNT"]
pub type RegtgtocrhandoffwaitcntR = crate::FieldReader<u16>;
#[doc = "Field `REGTGTOCRHANDOFFWAITCNT` writer - REG_TG_TO_CR_HANDOFF_WAIT_CNT"]
pub type RegtgtocrhandoffwaitcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - REG_I3C_SPECIAL_PATTERN_SETUP_CNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternsetupcnt(&self) -> Regi3cspecialpatternsetupcntR {
        Regi3cspecialpatternsetupcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_I3C_SPECIAL_PATTERN_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternsdatrancnt(&self) -> Regi3cspecialpatternsdatrancntR {
        Regi3cspecialpatternsdatrancntR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - REG_TG_TO_CR_HANDOFF_WAIT_CNT"]
    #[inline(always)]
    pub fn regtgtocrhandoffwaitcnt(&self) -> RegtgtocrhandoffwaitcntR {
        RegtgtocrhandoffwaitcntR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - REG_I3C_SPECIAL_PATTERN_SETUP_CNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternsetupcnt(
        &mut self,
    ) -> Regi3cspecialpatternsetupcntW<I3cphyctrlreg0e0Spec> {
        Regi3cspecialpatternsetupcntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_I3C_SPECIAL_PATTERN_SDA_TRAN_CNT"]
    #[inline(always)]
    pub fn regi3cspecialpatternsdatrancnt(
        &mut self,
    ) -> Regi3cspecialpatternsdatrancntW<I3cphyctrlreg0e0Spec> {
        Regi3cspecialpatternsdatrancntW::new(self, 8)
    }
    #[doc = "Bits 16:25 - REG_TG_TO_CR_HANDOFF_WAIT_CNT"]
    #[inline(always)]
    pub fn regtgtocrhandoffwaitcnt(&mut self) -> RegtgtocrhandoffwaitcntW<I3cphyctrlreg0e0Spec> {
        RegtgtocrhandoffwaitcntW::new(self, 16)
    }
}
#[doc = "SPECIAL\\_PATTERN\\_SET\\_ADDITIONAL\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0e0Spec;
impl crate::RegisterSpec for I3cphyctrlreg0e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0e0::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0e0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0e0::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0E0 to value 0x00c8_0109"]
impl crate::Resettable for I3cphyctrlreg0e0Spec {
    const RESET_VALUE: u32 = 0x00c8_0109;
}
