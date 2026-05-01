#[doc = "Register `HCICAPABILITY004` reader"]
pub type R = crate::R<Hcicapability004Spec>;
#[doc = "Register `HCICAPABILITY004` writer"]
pub type W = crate::W<Hcicapability004Spec>;
#[doc = "Field `REGIBAINCLUDE` reader - REG_IBA_INCLUDE"]
pub type RegibaincludeR = crate::BitReader;
#[doc = "Field `REGIBAINCLUDE` writer - REG_IBA_INCLUDE"]
pub type RegibaincludeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `REGDATABYTEORDERMODE` reader - REG_DATA_BYTE_ORDER_MODE"]
pub type RegdatabyteordermodeR = crate::BitReader;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGMODESELECTOR` reader - REG_MODE_SELECTOR"]
pub type RegmodeselectorR = crate::BitReader;
#[doc = "Field `REGMODESELECTOR` writer - REG_MODE_SELECTOR"]
pub type RegmodeselectorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGI2CDEVPRESENT` reader - REG_I2C_DEV_PRESENT"]
pub type Regi2cdevpresentR = crate::BitReader;
#[doc = "Field `REGI2CDEVPRESENT` writer - REG_I2C_DEV_PRESENT"]
pub type Regi2cdevpresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGHOTJOINCTRL` reader - REG_HOT_JOIN_CTRL"]
pub type ReghotjoinctrlR = crate::BitReader;
#[doc = "Field `REGHOTJOINCTRL` writer - REG_HOT_JOIN_CTRL"]
pub type ReghotjoinctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGHALTONCMDSEQTIMEOUT` reader - REG_HALT_ON_CMD_SEQ_TIMEOUT"]
pub type ReghaltoncmdseqtimeoutR = crate::BitReader;
#[doc = "Field `REGHALTONCMDSEQTIMEOUT` writer - REG_HALT_ON_CMD_SEQ_TIMEOUT"]
pub type ReghaltoncmdseqtimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGABORT` reader - REG_ABORT"]
pub type RegabortR = crate::BitReader;
#[doc = "Field `REGABORT` writer - REG_ABORT"]
pub type RegabortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRESUME` reader - REG_RESUME"]
pub type RegresumeR = crate::BitReader;
#[doc = "Field `REGRESUME` writer - REG_RESUME"]
pub type RegresumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBUSENABLE` reader - REG_BUS_ENABLE"]
pub type RegbusenableR = crate::BitReader;
#[doc = "Field `REGBUSENABLE` writer - REG_BUS_ENABLE"]
pub type RegbusenableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_IBA_INCLUDE"]
    #[inline(always)]
    pub fn regibainclude(&self) -> RegibaincludeR {
        RegibaincludeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - REG_DATA_BYTE_ORDER_MODE"]
    #[inline(always)]
    pub fn regdatabyteordermode(&self) -> RegdatabyteordermodeR {
        RegdatabyteordermodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_MODE_SELECTOR"]
    #[inline(always)]
    pub fn regmodeselector(&self) -> RegmodeselectorR {
        RegmodeselectorR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REG_I2C_DEV_PRESENT"]
    #[inline(always)]
    pub fn regi2cdevpresent(&self) -> Regi2cdevpresentR {
        Regi2cdevpresentR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REG_HOT_JOIN_CTRL"]
    #[inline(always)]
    pub fn reghotjoinctrl(&self) -> ReghotjoinctrlR {
        ReghotjoinctrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - REG_HALT_ON_CMD_SEQ_TIMEOUT"]
    #[inline(always)]
    pub fn reghaltoncmdseqtimeout(&self) -> ReghaltoncmdseqtimeoutR {
        ReghaltoncmdseqtimeoutR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 29 - REG_ABORT"]
    #[inline(always)]
    pub fn regabort(&self) -> RegabortR {
        RegabortR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - REG_RESUME"]
    #[inline(always)]
    pub fn regresume(&self) -> RegresumeR {
        RegresumeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - REG_BUS_ENABLE"]
    #[inline(always)]
    pub fn regbusenable(&self) -> RegbusenableR {
        RegbusenableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_IBA_INCLUDE"]
    #[inline(always)]
    pub fn regibainclude(&mut self) -> RegibaincludeW<Hcicapability004Spec> {
        RegibaincludeW::new(self, 0)
    }
    #[doc = "Bit 6 - REG_MODE_SELECTOR"]
    #[inline(always)]
    pub fn regmodeselector(&mut self) -> RegmodeselectorW<Hcicapability004Spec> {
        RegmodeselectorW::new(self, 6)
    }
    #[doc = "Bit 7 - REG_I2C_DEV_PRESENT"]
    #[inline(always)]
    pub fn regi2cdevpresent(&mut self) -> Regi2cdevpresentW<Hcicapability004Spec> {
        Regi2cdevpresentW::new(self, 7)
    }
    #[doc = "Bit 8 - REG_HOT_JOIN_CTRL"]
    #[inline(always)]
    pub fn reghotjoinctrl(&mut self) -> ReghotjoinctrlW<Hcicapability004Spec> {
        ReghotjoinctrlW::new(self, 8)
    }
    #[doc = "Bit 12 - REG_HALT_ON_CMD_SEQ_TIMEOUT"]
    #[inline(always)]
    pub fn reghaltoncmdseqtimeout(&mut self) -> ReghaltoncmdseqtimeoutW<Hcicapability004Spec> {
        ReghaltoncmdseqtimeoutW::new(self, 12)
    }
    #[doc = "Bit 29 - REG_ABORT"]
    #[inline(always)]
    pub fn regabort(&mut self) -> RegabortW<Hcicapability004Spec> {
        RegabortW::new(self, 29)
    }
    #[doc = "Bit 30 - REG_RESUME"]
    #[inline(always)]
    pub fn regresume(&mut self) -> RegresumeW<Hcicapability004Spec> {
        RegresumeW::new(self, 30)
    }
    #[doc = "Bit 31 - REG_BUS_ENABLE"]
    #[inline(always)]
    pub fn regbusenable(&mut self) -> RegbusenableW<Hcicapability004Spec> {
        RegbusenableW::new(self, 31)
    }
}
#[doc = "HC\\_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`hcicapability004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcicapability004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcicapability004Spec;
impl crate::RegisterSpec for Hcicapability004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcicapability004::R`](R) reader structure"]
impl crate::Readable for Hcicapability004Spec {}
#[doc = "`write(|w| ..)` method takes [`hcicapability004::W`](W) writer structure"]
impl crate::Writable for Hcicapability004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCICAPABILITY004 to value 0"]
impl crate::Resettable for Hcicapability004Spec {}
