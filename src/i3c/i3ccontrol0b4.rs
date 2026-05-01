#[doc = "Register `I3CCONTROL0B4` reader"]
pub type R = crate::R<I3ccontrol0b4Spec>;
#[doc = "Register `I3CCONTROL0B4` writer"]
pub type W = crate::W<I3ccontrol0b4Spec>;
#[doc = "Field `REGIBIDONE` reader - REG_IBI_DONE"]
pub type RegibidoneR = crate::BitReader;
#[doc = "Field `REGMRDONE` reader - REG_MR_DONE"]
pub type RegmrdoneR = crate::BitReader;
#[doc = "Field `REGHJDONE` reader - REG_HJ_DONE"]
pub type ReghjdoneR = crate::BitReader;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGENINT` reader - REG_ENINT"]
pub type RegenintR = crate::BitReader;
#[doc = "Field `REGENINT` writer - REG_ENINT"]
pub type RegenintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGENCR` reader - REG_ENCR"]
pub type RegencrR = crate::BitReader;
#[doc = "Field `REGENCR` writer - REG_ENCR"]
pub type RegencrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGENHJ` reader - REG_ENHJ"]
pub type RegenhjR = crate::BitReader;
#[doc = "Field `REGENHJ` writer - REG_ENHJ"]
pub type RegenhjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGENTTM` reader - REG_ENTTM"]
pub type RegenttmR = crate::BitReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGSLVINBUSY` reader - REG_SLV_IN_BUSY"]
pub type RegslvinbusyR = crate::BitReader;
#[doc = "Field `REGENDXFERCONFIRM` reader - REG_ENDXFER_CONFIRM"]
pub type RegendxferconfirmR = crate::BitReader;
#[doc = "Field `REGENDXFERCRCWORD` reader - REG_ENDXFER_CRC_WORD"]
pub type RegendxfercrcwordR = crate::FieldReader;
#[doc = "Field `REGENDXFERENWET` reader - REG_ENDXFER_EN_W_ET"]
pub type RegendxferenwetR = crate::BitReader;
#[doc = "Field `REGENDXFERACKCAP` reader - REG_ENDXFER_ACK_CAP"]
pub type RegendxferackcapR = crate::BitReader;
#[doc = "Field `REGRSTACTDEFINE` reader - REG_RSTACT_DEFINE"]
pub type RegrstactdefineR = crate::FieldReader;
#[doc = "Field `REGIBIPAYLOADSIZE` reader - REG_IBI_PAYLOAD_SIZE"]
pub type RegibipayloadsizeR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - REG_IBI_DONE"]
    #[inline(always)]
    pub fn regibidone(&self) -> RegibidoneR {
        RegibidoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_MR_DONE"]
    #[inline(always)]
    pub fn regmrdone(&self) -> RegmrdoneR {
        RegmrdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_HJ_DONE"]
    #[inline(always)]
    pub fn reghjdone(&self) -> ReghjdoneR {
        ReghjdoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REG_ENINT"]
    #[inline(always)]
    pub fn regenint(&self) -> RegenintR {
        RegenintR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_ENCR"]
    #[inline(always)]
    pub fn regencr(&self) -> RegencrR {
        RegencrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_ENHJ"]
    #[inline(always)]
    pub fn regenhj(&self) -> RegenhjR {
        RegenhjR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REG_ENTTM"]
    #[inline(always)]
    pub fn regenttm(&self) -> RegenttmR {
        RegenttmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_SLV_IN_BUSY"]
    #[inline(always)]
    pub fn regslvinbusy(&self) -> RegslvinbusyR {
        RegslvinbusyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_ENDXFER_CONFIRM"]
    #[inline(always)]
    pub fn regendxferconfirm(&self) -> RegendxferconfirmR {
        RegendxferconfirmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - REG_ENDXFER_CRC_WORD"]
    #[inline(always)]
    pub fn regendxfercrcword(&self) -> RegendxfercrcwordR {
        RegendxfercrcwordR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - REG_ENDXFER_EN_W_ET"]
    #[inline(always)]
    pub fn regendxferenwet(&self) -> RegendxferenwetR {
        RegendxferenwetR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - REG_ENDXFER_ACK_CAP"]
    #[inline(always)]
    pub fn regendxferackcap(&self) -> RegendxferackcapR {
        RegendxferackcapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - REG_RSTACT_DEFINE"]
    #[inline(always)]
    pub fn regrstactdefine(&self) -> RegrstactdefineR {
        RegrstactdefineR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - REG_IBI_PAYLOAD_SIZE"]
    #[inline(always)]
    pub fn regibipayloadsize(&self) -> RegibipayloadsizeR {
        RegibipayloadsizeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - REG_ENINT"]
    #[inline(always)]
    pub fn regenint(&mut self) -> RegenintW<I3ccontrol0b4Spec> {
        RegenintW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_ENCR"]
    #[inline(always)]
    pub fn regencr(&mut self) -> RegencrW<I3ccontrol0b4Spec> {
        RegencrW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_ENHJ"]
    #[inline(always)]
    pub fn regenhj(&mut self) -> RegenhjW<I3ccontrol0b4Spec> {
        RegenhjW::new(self, 6)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0B4\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0b4Spec;
impl crate::RegisterSpec for I3ccontrol0b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0b4::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0b4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0b4::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0B4 to value 0x0001_0070"]
impl crate::Resettable for I3ccontrol0b4Spec {
    const RESET_VALUE: u32 = 0x0001_0070;
}
