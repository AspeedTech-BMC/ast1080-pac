#[doc = "Register `I3CPHYCTRLREG0E4` reader"]
pub type R = crate::R<I3cphyctrlreg0e4Spec>;
#[doc = "Register `I3CPHYCTRLREG0E4` writer"]
pub type W = crate::W<I3cphyctrlreg0e4Spec>;
#[doc = "Field `REGBUSCONTCHKSEL` reader - REG_BUS_CONT_CHK_SEL"]
pub type RegbuscontchkselR = crate::FieldReader;
#[doc = "Field `REGBUSCONTCHKSEL` writer - REG_BUS_CONT_CHK_SEL"]
pub type RegbuscontchkselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REGBUSCONTARBADDRCHKENB` reader - REG_BUS_CONT_ARB_ADDR_CHK_ENB"]
pub type RegbuscontarbaddrchkenbR = crate::BitReader;
#[doc = "Field `REGBUSCONTARBADDRCHKENB` writer - REG_BUS_CONT_ARB_ADDR_CHK_ENB"]
pub type RegbuscontarbaddrchkenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBUSCONTDAACHKEN` reader - REG_BUS_CONT_DAA_CHK_EN"]
pub type RegbuscontdaachkenR = crate::BitReader;
#[doc = "Field `REGBUSCONTDAACHKEN` writer - REG_BUS_CONT_DAA_CHK_EN"]
pub type RegbuscontdaachkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBUSCONTCRSTARTCHKEN` reader - REG_BUS_CONT_CR_START_CHK_EN"]
pub type RegbuscontcrstartchkenR = crate::BitReader;
#[doc = "Field `REGBUSCONTCRSTARTCHKEN` writer - REG_BUS_CONT_CR_START_CHK_EN"]
pub type RegbuscontcrstartchkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBUSCONTTGSTARTCHKENB` reader - REG_BUS_CONT_TG_START_CHK_ENB"]
pub type RegbusconttgstartchkenbR = crate::BitReader;
#[doc = "Field `REGBUSCONTTGSTARTCHKENB` writer - REG_BUS_CONT_TG_START_CHK_ENB"]
pub type RegbusconttgstartchkenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBUSCONTCRDDRCRCCHKENB` reader - REG_BUS_CONT_CR_DDR_CRC_CHK_ENB"]
pub type RegbuscontcrddrcrcchkenbR = crate::BitReader;
#[doc = "Field `REGBUSCONTCRDDRCRCCHKENB` writer - REG_BUS_CONT_CR_DDR_CRC_CHK_ENB"]
pub type RegbuscontcrddrcrcchkenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBUSCONTCRDDRCRCLASTBITCHKENB` reader - REG_BUS_CONT_CR_DDR_CRC_LAST_BIT_CHK_ENB"]
pub type RegbuscontcrddrcrclastbitchkenbR = crate::BitReader;
#[doc = "Field `REGBUSCONTCRDDRCRCLASTBITCHKENB` writer - REG_BUS_CONT_CR_DDR_CRC_LAST_BIT_CHK_ENB"]
pub type RegbuscontcrddrcrclastbitchkenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBUSCONTCRDDRFINCHKENB` reader - REG_BUS_CONT_CR_DDR_FIN_CHK_ENB"]
pub type RegbuscontcrddrfinchkenbR = crate::BitReader;
#[doc = "Field `REGBUSCONTCRDDRFINCHKENB` writer - REG_BUS_CONT_CR_DDR_FIN_CHK_ENB"]
pub type RegbuscontcrddrfinchkenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBUSCONTTGDDRCRCCHKENB` reader - REG_BUS_CONT_TG_DDR_CRC_CHK_ENB"]
pub type RegbusconttgddrcrcchkenbR = crate::BitReader;
#[doc = "Field `REGBUSCONTTGDDRCRCCHKENB` writer - REG_BUS_CONT_TG_DDR_CRC_CHK_ENB"]
pub type RegbusconttgddrcrcchkenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGSCLCONTOCCUR` reader - REG_SCL_CONT_OCCUR"]
pub type RegsclcontoccurR = crate::BitReader;
#[doc = "Field `REGSDACONTOCCUR` reader - REG_SDA_CONT_OCCUR"]
pub type RegsdacontoccurR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - REG_BUS_CONT_CHK_SEL"]
    #[inline(always)]
    pub fn regbuscontchksel(&self) -> RegbuscontchkselR {
        RegbuscontchkselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - REG_BUS_CONT_ARB_ADDR_CHK_ENB"]
    #[inline(always)]
    pub fn regbuscontarbaddrchkenb(&self) -> RegbuscontarbaddrchkenbR {
        RegbuscontarbaddrchkenbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_BUS_CONT_DAA_CHK_EN"]
    #[inline(always)]
    pub fn regbuscontdaachken(&self) -> RegbuscontdaachkenR {
        RegbuscontdaachkenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_BUS_CONT_CR_START_CHK_EN"]
    #[inline(always)]
    pub fn regbuscontcrstartchken(&self) -> RegbuscontcrstartchkenR {
        RegbuscontcrstartchkenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REG_BUS_CONT_TG_START_CHK_ENB"]
    #[inline(always)]
    pub fn regbusconttgstartchkenb(&self) -> RegbusconttgstartchkenbR {
        RegbusconttgstartchkenbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REG_BUS_CONT_CR_DDR_CRC_CHK_ENB"]
    #[inline(always)]
    pub fn regbuscontcrddrcrcchkenb(&self) -> RegbuscontcrddrcrcchkenbR {
        RegbuscontcrddrcrcchkenbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REG_BUS_CONT_CR_DDR_CRC_LAST_BIT_CHK_ENB"]
    #[inline(always)]
    pub fn regbuscontcrddrcrclastbitchkenb(&self) -> RegbuscontcrddrcrclastbitchkenbR {
        RegbuscontcrddrcrclastbitchkenbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_BUS_CONT_CR_DDR_FIN_CHK_ENB"]
    #[inline(always)]
    pub fn regbuscontcrddrfinchkenb(&self) -> RegbuscontcrddrfinchkenbR {
        RegbuscontcrddrfinchkenbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_BUS_CONT_TG_DDR_CRC_CHK_ENB"]
    #[inline(always)]
    pub fn regbusconttgddrcrcchkenb(&self) -> RegbusconttgddrcrcchkenbR {
        RegbusconttgddrcrcchkenbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REG_SCL_CONT_OCCUR"]
    #[inline(always)]
    pub fn regsclcontoccur(&self) -> RegsclcontoccurR {
        RegsclcontoccurR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REG_SDA_CONT_OCCUR"]
    #[inline(always)]
    pub fn regsdacontoccur(&self) -> RegsdacontoccurR {
        RegsdacontoccurR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - REG_BUS_CONT_CHK_SEL"]
    #[inline(always)]
    pub fn regbuscontchksel(&mut self) -> RegbuscontchkselW<I3cphyctrlreg0e4Spec> {
        RegbuscontchkselW::new(self, 0)
    }
    #[doc = "Bit 4 - REG_BUS_CONT_ARB_ADDR_CHK_ENB"]
    #[inline(always)]
    pub fn regbuscontarbaddrchkenb(&mut self) -> RegbuscontarbaddrchkenbW<I3cphyctrlreg0e4Spec> {
        RegbuscontarbaddrchkenbW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_BUS_CONT_DAA_CHK_EN"]
    #[inline(always)]
    pub fn regbuscontdaachken(&mut self) -> RegbuscontdaachkenW<I3cphyctrlreg0e4Spec> {
        RegbuscontdaachkenW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_BUS_CONT_CR_START_CHK_EN"]
    #[inline(always)]
    pub fn regbuscontcrstartchken(&mut self) -> RegbuscontcrstartchkenW<I3cphyctrlreg0e4Spec> {
        RegbuscontcrstartchkenW::new(self, 6)
    }
    #[doc = "Bit 7 - REG_BUS_CONT_TG_START_CHK_ENB"]
    #[inline(always)]
    pub fn regbusconttgstartchkenb(&mut self) -> RegbusconttgstartchkenbW<I3cphyctrlreg0e4Spec> {
        RegbusconttgstartchkenbW::new(self, 7)
    }
    #[doc = "Bit 8 - REG_BUS_CONT_CR_DDR_CRC_CHK_ENB"]
    #[inline(always)]
    pub fn regbuscontcrddrcrcchkenb(&mut self) -> RegbuscontcrddrcrcchkenbW<I3cphyctrlreg0e4Spec> {
        RegbuscontcrddrcrcchkenbW::new(self, 8)
    }
    #[doc = "Bit 9 - REG_BUS_CONT_CR_DDR_CRC_LAST_BIT_CHK_ENB"]
    #[inline(always)]
    pub fn regbuscontcrddrcrclastbitchkenb(
        &mut self,
    ) -> RegbuscontcrddrcrclastbitchkenbW<I3cphyctrlreg0e4Spec> {
        RegbuscontcrddrcrclastbitchkenbW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_BUS_CONT_CR_DDR_FIN_CHK_ENB"]
    #[inline(always)]
    pub fn regbuscontcrddrfinchkenb(&mut self) -> RegbuscontcrddrfinchkenbW<I3cphyctrlreg0e4Spec> {
        RegbuscontcrddrfinchkenbW::new(self, 10)
    }
    #[doc = "Bit 11 - REG_BUS_CONT_TG_DDR_CRC_CHK_ENB"]
    #[inline(always)]
    pub fn regbusconttgddrcrcchkenb(&mut self) -> RegbusconttgddrcrcchkenbW<I3cphyctrlreg0e4Spec> {
        RegbusconttgddrcrcchkenbW::new(self, 11)
    }
}
#[doc = "BUS\\_CONTENTION\\_CHK0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0e4Spec;
impl crate::RegisterSpec for I3cphyctrlreg0e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0e4::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0e4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0e4::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0E4 to value 0x0f90"]
impl crate::Resettable for I3cphyctrlreg0e4Spec {
    const RESET_VALUE: u32 = 0x0f90;
}
