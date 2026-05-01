#[doc = "Register `I3CPHYCTRLREG0CC` reader"]
pub type R = crate::R<I3cphyctrlreg0ccSpec>;
#[doc = "Register `I3CPHYCTRLREG0CC` writer"]
pub type W = crate::W<I3cphyctrlreg0ccSpec>;
#[doc = "Field `REGDEGLITCHLEVEL` reader - REG_DEGLITCH_LEVEL"]
pub type RegdeglitchlevelR = crate::FieldReader;
#[doc = "Field `REGDEGLITCHLEVEL` writer - REG_DEGLITCH_LEVEL"]
pub type RegdeglitchlevelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REGTGABORTRECALCBUSCOND` reader - REG_TG_ABORT_RECALC_BUS_COND"]
pub type RegtgabortrecalcbuscondR = crate::BitReader;
#[doc = "Field `REGTGABORTRECALCBUSCOND` writer - REG_TG_ABORT_RECALC_BUS_COND"]
pub type RegtgabortrecalcbuscondW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTGSTARTDETEN` reader - REG_TG_START_DET_EN"]
pub type RegtgstartdetenR = crate::BitReader;
#[doc = "Field `REGTGSTARTDETEN` writer - REG_TG_START_DET_EN"]
pub type RegtgstartdetenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGI3CSDRCRIDLEDRIVEEN` reader - REG_I3C_SDR_CR_IDLE_DRIVE_EN"]
pub type Regi3csdrcridledriveenR = crate::BitReader;
#[doc = "Field `REGI3CSDRCRIDLEDRIVEEN` writer - REG_I3C_SDR_CR_IDLE_DRIVE_EN"]
pub type Regi3csdrcridledriveenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTGI2CMODESDADRIVEEN` reader - REG_TG_I2C_MODE_SDA_DRIVE_EN"]
pub type Regtgi2cmodesdadriveenR = crate::BitReader;
#[doc = "Field `REGTGI2CMODESDADRIVEEN` writer - REG_TG_I2C_MODE_SDA_DRIVE_EN"]
pub type Regtgi2cmodesdadriveenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPHYAUTOSTOPEN` reader - REG_PHY_AUTO_STOP_EN"]
pub type RegphyautostopenR = crate::BitReader;
#[doc = "Field `REGPHYAUTOSTOPEN` writer - REG_PHY_AUTO_STOP_EN"]
pub type RegphyautostopenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDETIBIMSTINHALTENB` reader - REG_DET_IBI_MST_IN_HALT_ENB"]
pub type RegdetibimstinhaltenbR = crate::BitReader;
#[doc = "Field `REGDETIBIMSTINHALTENB` writer - REG_DET_IBI_MST_IN_HALT_ENB"]
pub type RegdetibimstinhaltenbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGNEGCLKEN` reader - REG_NEG_CLK_EN"]
pub type RegnegclkenR = crate::BitReader;
#[doc = "Field `REGNEGCLKEN` writer - REG_NEG_CLK_EN"]
pub type RegnegclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGCRSTOPFAILCHECKEN` reader - REG_CR_STOP_FAIL_CHECK_EN"]
pub type RegcrstopfailcheckenR = crate::BitReader;
#[doc = "Field `REGCRSTOPFAILCHECKEN` writer - REG_CR_STOP_FAIL_CHECK_EN"]
pub type RegcrstopfailcheckenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBUSFREETIMECHKEN` reader - REG_BUS_FREE_TIME_CHK_EN"]
pub type RegbusfreetimechkenR = crate::BitReader;
#[doc = "Field `REGBUSFREETIMECHKEN` writer - REG_BUS_FREE_TIME_CHK_EN"]
pub type RegbusfreetimechkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTGTOCRHANDOFFWAITSCLRST` reader - REG_TG_TO_CR_HANDOFF_WAIT_SCL_RST"]
pub type RegtgtocrhandoffwaitsclrstR = crate::BitReader;
#[doc = "Field `REGTGTOCRHANDOFFWAITSCLRST` writer - REG_TG_TO_CR_HANDOFF_WAIT_SCL_RST"]
pub type RegtgtocrhandoffwaitsclrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTGTOCRHANDOFFWAITSDARST` reader - REG_TG_TO_CR_HANDOFF_WAIT_SDA_RST"]
pub type RegtgtocrhandoffwaitsdarstR = crate::BitReader;
#[doc = "Field `REGTGTOCRHANDOFFWAITSDARST` writer - REG_TG_TO_CR_HANDOFF_WAIT_SDA_RST"]
pub type RegtgtocrhandoffwaitsdarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGNOHDRMODEDETEN` reader - REG_NO_HDR_MODE_DET_EN"]
pub type RegnohdrmodedetenR = crate::BitReader;
#[doc = "Field `REGNOHDRMODEDETEN` writer - REG_NO_HDR_MODE_DET_EN"]
pub type RegnohdrmodedetenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGBUSIDLECONDSEL` reader - REG_BUS_IDLE_COND_SEL"]
pub type RegbusidlecondselR = crate::BitReader;
#[doc = "Field `REGBUSIDLECONDSEL` writer - REG_BUS_IDLE_COND_SEL"]
pub type RegbusidlecondselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGTIMERSTDETEN` reader - REG_TIME_RST_DET_EN"]
pub type RegtimerstdetenR = crate::BitReader;
#[doc = "Field `REGTIMERSTDETEN` writer - REG_TIME_RST_DET_EN"]
pub type RegtimerstdetenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - REG_DEGLITCH_LEVEL"]
    #[inline(always)]
    pub fn regdeglitchlevel(&self) -> RegdeglitchlevelR {
        RegdeglitchlevelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - REG_TG_ABORT_RECALC_BUS_COND"]
    #[inline(always)]
    pub fn regtgabortrecalcbuscond(&self) -> RegtgabortrecalcbuscondR {
        RegtgabortrecalcbuscondR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REG_TG_START_DET_EN"]
    #[inline(always)]
    pub fn regtgstartdeten(&self) -> RegtgstartdetenR {
        RegtgstartdetenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_I3C_SDR_CR_IDLE_DRIVE_EN"]
    #[inline(always)]
    pub fn regi3csdrcridledriveen(&self) -> Regi3csdrcridledriveenR {
        Regi3csdrcridledriveenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_TG_I2C_MODE_SDA_DRIVE_EN"]
    #[inline(always)]
    pub fn regtgi2cmodesdadriveen(&self) -> Regtgi2cmodesdadriveenR {
        Regtgi2cmodesdadriveenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REG_PHY_AUTO_STOP_EN"]
    #[inline(always)]
    pub fn regphyautostopen(&self) -> RegphyautostopenR {
        RegphyautostopenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REG_DET_IBI_MST_IN_HALT_ENB"]
    #[inline(always)]
    pub fn regdetibimstinhaltenb(&self) -> RegdetibimstinhaltenbR {
        RegdetibimstinhaltenbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REG_NEG_CLK_EN"]
    #[inline(always)]
    pub fn regnegclken(&self) -> RegnegclkenR {
        RegnegclkenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REG_CR_STOP_FAIL_CHECK_EN"]
    #[inline(always)]
    pub fn regcrstopfailchecken(&self) -> RegcrstopfailcheckenR {
        RegcrstopfailcheckenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - REG_BUS_FREE_TIME_CHK_EN"]
    #[inline(always)]
    pub fn regbusfreetimechken(&self) -> RegbusfreetimechkenR {
        RegbusfreetimechkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REG_TG_TO_CR_HANDOFF_WAIT_SCL_RST"]
    #[inline(always)]
    pub fn regtgtocrhandoffwaitsclrst(&self) -> RegtgtocrhandoffwaitsclrstR {
        RegtgtocrhandoffwaitsclrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REG_TG_TO_CR_HANDOFF_WAIT_SDA_RST"]
    #[inline(always)]
    pub fn regtgtocrhandoffwaitsdarst(&self) -> RegtgtocrhandoffwaitsdarstR {
        RegtgtocrhandoffwaitsdarstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - REG_NO_HDR_MODE_DET_EN"]
    #[inline(always)]
    pub fn regnohdrmodedeten(&self) -> RegnohdrmodedetenR {
        RegnohdrmodedetenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - REG_BUS_IDLE_COND_SEL"]
    #[inline(always)]
    pub fn regbusidlecondsel(&self) -> RegbusidlecondselR {
        RegbusidlecondselR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - REG_TIME_RST_DET_EN"]
    #[inline(always)]
    pub fn regtimerstdeten(&self) -> RegtimerstdetenR {
        RegtimerstdetenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_DEGLITCH_LEVEL"]
    #[inline(always)]
    pub fn regdeglitchlevel(&mut self) -> RegdeglitchlevelW<I3cphyctrlreg0ccSpec> {
        RegdeglitchlevelW::new(self, 0)
    }
    #[doc = "Bit 3 - REG_TG_ABORT_RECALC_BUS_COND"]
    #[inline(always)]
    pub fn regtgabortrecalcbuscond(&mut self) -> RegtgabortrecalcbuscondW<I3cphyctrlreg0ccSpec> {
        RegtgabortrecalcbuscondW::new(self, 3)
    }
    #[doc = "Bit 4 - REG_TG_START_DET_EN"]
    #[inline(always)]
    pub fn regtgstartdeten(&mut self) -> RegtgstartdetenW<I3cphyctrlreg0ccSpec> {
        RegtgstartdetenW::new(self, 4)
    }
    #[doc = "Bit 5 - REG_I3C_SDR_CR_IDLE_DRIVE_EN"]
    #[inline(always)]
    pub fn regi3csdrcridledriveen(&mut self) -> Regi3csdrcridledriveenW<I3cphyctrlreg0ccSpec> {
        Regi3csdrcridledriveenW::new(self, 5)
    }
    #[doc = "Bit 6 - REG_TG_I2C_MODE_SDA_DRIVE_EN"]
    #[inline(always)]
    pub fn regtgi2cmodesdadriveen(&mut self) -> Regtgi2cmodesdadriveenW<I3cphyctrlreg0ccSpec> {
        Regtgi2cmodesdadriveenW::new(self, 6)
    }
    #[doc = "Bit 7 - REG_PHY_AUTO_STOP_EN"]
    #[inline(always)]
    pub fn regphyautostopen(&mut self) -> RegphyautostopenW<I3cphyctrlreg0ccSpec> {
        RegphyautostopenW::new(self, 7)
    }
    #[doc = "Bit 8 - REG_DET_IBI_MST_IN_HALT_ENB"]
    #[inline(always)]
    pub fn regdetibimstinhaltenb(&mut self) -> RegdetibimstinhaltenbW<I3cphyctrlreg0ccSpec> {
        RegdetibimstinhaltenbW::new(self, 8)
    }
    #[doc = "Bit 9 - REG_NEG_CLK_EN"]
    #[inline(always)]
    pub fn regnegclken(&mut self) -> RegnegclkenW<I3cphyctrlreg0ccSpec> {
        RegnegclkenW::new(self, 9)
    }
    #[doc = "Bit 10 - REG_CR_STOP_FAIL_CHECK_EN"]
    #[inline(always)]
    pub fn regcrstopfailchecken(&mut self) -> RegcrstopfailcheckenW<I3cphyctrlreg0ccSpec> {
        RegcrstopfailcheckenW::new(self, 10)
    }
    #[doc = "Bit 11 - REG_BUS_FREE_TIME_CHK_EN"]
    #[inline(always)]
    pub fn regbusfreetimechken(&mut self) -> RegbusfreetimechkenW<I3cphyctrlreg0ccSpec> {
        RegbusfreetimechkenW::new(self, 11)
    }
    #[doc = "Bit 12 - REG_TG_TO_CR_HANDOFF_WAIT_SCL_RST"]
    #[inline(always)]
    pub fn regtgtocrhandoffwaitsclrst(
        &mut self,
    ) -> RegtgtocrhandoffwaitsclrstW<I3cphyctrlreg0ccSpec> {
        RegtgtocrhandoffwaitsclrstW::new(self, 12)
    }
    #[doc = "Bit 13 - REG_TG_TO_CR_HANDOFF_WAIT_SDA_RST"]
    #[inline(always)]
    pub fn regtgtocrhandoffwaitsdarst(
        &mut self,
    ) -> RegtgtocrhandoffwaitsdarstW<I3cphyctrlreg0ccSpec> {
        RegtgtocrhandoffwaitsdarstW::new(self, 13)
    }
    #[doc = "Bit 14 - REG_NO_HDR_MODE_DET_EN"]
    #[inline(always)]
    pub fn regnohdrmodedeten(&mut self) -> RegnohdrmodedetenW<I3cphyctrlreg0ccSpec> {
        RegnohdrmodedetenW::new(self, 14)
    }
    #[doc = "Bit 15 - REG_BUS_IDLE_COND_SEL"]
    #[inline(always)]
    pub fn regbusidlecondsel(&mut self) -> RegbusidlecondselW<I3cphyctrlreg0ccSpec> {
        RegbusidlecondselW::new(self, 15)
    }
    #[doc = "Bit 16 - REG_TIME_RST_DET_EN"]
    #[inline(always)]
    pub fn regtimerstdeten(&mut self) -> RegtimerstdetenW<I3cphyctrlreg0ccSpec> {
        RegtimerstdetenW::new(self, 16)
    }
}
#[doc = "PHY\\_OPTION\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0ccSpec;
impl crate::RegisterSpec for I3cphyctrlreg0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0cc::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0cc::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0CC to value 0x0190"]
impl crate::Resettable for I3cphyctrlreg0ccSpec {
    const RESET_VALUE: u32 = 0x0190;
}
