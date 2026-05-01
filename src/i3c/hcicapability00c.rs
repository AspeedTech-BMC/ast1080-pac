#[doc = "Register `HCICAPABILITY00C` reader"]
pub type R = crate::R<Hcicapability00cSpec>;
#[doc = "Register `HCICAPABILITY00C` writer"]
pub type W = crate::W<Hcicapability00cSpec>;
#[doc = "Field `REGCOMBOCOMMAND` reader - REG_COMBO_COMMAND"]
pub type RegcombocommandR = crate::BitReader;
#[doc = "Field `REGAUTOCOMMAND` reader - REG_AUTO_COMMAND"]
pub type RegautocommandR = crate::BitReader;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGSTANDBYCRCAP` reader - REG_STANDBY_CR_CAP"]
pub type RegstandbycrcapR = crate::BitReader;
#[doc = "Field `REGHDRDDREN` reader - REG_HDR_DDR_EN"]
pub type ReghdrddrenR = crate::BitReader;
#[doc = "Field `REGHDRTSEN` reader - REG_HDR_TS_EN"]
pub type ReghdrtsenR = crate::BitReader;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `REGCMDCCCDEFBYTE` reader - REG_CMD_CCC_DEFBYTE"]
pub type RegcmdcccdefbyteR = crate::BitReader;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `REGCMDSIZE` reader - REG_CMD_SIZE"]
pub type RegcmdsizeR = crate::FieldReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGSGCAPABILITYCREN` reader - REG_SG_CAPABILITY_CR_EN"]
pub type RegsgcapabilitycrenR = crate::BitReader;
#[doc = "Field `REGSGCAPABILITYIBIEN` reader - REG_SG_CAPABILITY_IBI_EN"]
pub type RegsgcapabilityibienR = crate::BitReader;
#[doc = "Field `REGSGCAPABILITYDCEN` reader - REG_SG_CAPABILITY_DC_EN"]
pub type RegsgcapabilitydcenR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - REG_COMBO_COMMAND"]
    #[inline(always)]
    pub fn regcombocommand(&self) -> RegcombocommandR {
        RegcombocommandR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REG_AUTO_COMMAND"]
    #[inline(always)]
    pub fn regautocommand(&self) -> RegautocommandR {
        RegautocommandR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_STANDBY_CR_CAP"]
    #[inline(always)]
    pub fn regstandbycrcap(&self) -> RegstandbycrcapR {
        RegstandbycrcapR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_HDR_DDR_EN"]
    #[inline(always)]
    pub fn reghdrddren(&self) -> ReghdrddrenR {
        ReghdrddrenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REG_HDR_TS_EN"]
    #[inline(always)]
    pub fn reghdrtsen(&self) -> ReghdrtsenR {
        ReghdrtsenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - REG_CMD_CCC_DEFBYTE"]
    #[inline(always)]
    pub fn regcmdcccdefbyte(&self) -> RegcmdcccdefbyteR {
        RegcmdcccdefbyteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:19 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:21 - REG_CMD_SIZE"]
    #[inline(always)]
    pub fn regcmdsize(&self) -> RegcmdsizeR {
        RegcmdsizeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bit 28 - REG_SG_CAPABILITY_CR_EN"]
    #[inline(always)]
    pub fn regsgcapabilitycren(&self) -> RegsgcapabilitycrenR {
        RegsgcapabilitycrenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - REG_SG_CAPABILITY_IBI_EN"]
    #[inline(always)]
    pub fn regsgcapabilityibien(&self) -> RegsgcapabilityibienR {
        RegsgcapabilityibienR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - REG_SG_CAPABILITY_DC_EN"]
    #[inline(always)]
    pub fn regsgcapabilitydcen(&self) -> RegsgcapabilitydcenR {
        RegsgcapabilitydcenR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {}
#[doc = "HC\\_CAPABILITIES\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability00cSpec;
impl crate::RegisterSpec for Hcicapability00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability00c::R`](R) reader structure"]
impl crate::Readable for Hcicapability00cSpec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability00c::W`](W) writer structure"]
impl crate::Writable for Hcicapability00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY00C to value 0x0468"]
impl crate::Resettable for Hcicapability00cSpec {
    const RESET_VALUE: u32 = 0x0468;
}
