#[doc = "Register `GPIO03C` reader"]
pub type R = crate::R<Gpio03cSpec>;
#[doc = "Register `GPIO03C` writer"]
pub type W = crate::W<Gpio03cSpec>;
#[doc = "Field `WrProtOfGPIO000` reader - Write Protection of GPIO000"]
pub type WrProtOfGpio000R = crate::BitReader;
#[doc = "Field `WrProtOfGPIO000` writer - Write Protection of GPIO000"]
pub type WrProtOfGpio000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfGPIO004` reader - Write Protection of GPIO004"]
pub type WrProtOfGpio004R = crate::BitReader;
#[doc = "Field `WrProtOfGPIO004` writer - Write Protection of GPIO004"]
pub type WrProtOfGpio004W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfGPIO008` reader - Write Protection of GPIO008"]
pub type WrProtOfGpio008R = crate::BitReader;
#[doc = "Field `WrProtOfGPIO008` writer - Write Protection of GPIO008"]
pub type WrProtOfGpio008W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfGPIO00C` reader - Write Protection of GPIO00C"]
pub type WrProtOfGpio00cR = crate::BitReader;
#[doc = "Field `WrProtOfGPIO00C` writer - Write Protection of GPIO00C"]
pub type WrProtOfGpio00cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfGPIO010` reader - Write Protection of GPIO010"]
pub type WrProtOfGpio010R = crate::BitReader;
#[doc = "Field `WrProtOfGPIO010` writer - Write Protection of GPIO010"]
pub type WrProtOfGpio010W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfGPIO014` reader - Write Protection of GPIO014"]
pub type WrProtOfGpio014R = crate::BitReader;
#[doc = "Field `WrProtOfGPIO014` writer - Write Protection of GPIO014"]
pub type WrProtOfGpio014W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfGPIO018` reader - Write Protection of GPIO018"]
pub type WrProtOfGpio018R = crate::BitReader;
#[doc = "Field `WrProtOfGPIO018` writer - Write Protection of GPIO018"]
pub type WrProtOfGpio018W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfGPIO01C` reader - Write Protection of GPIO01C"]
pub type WrProtOfGpio01cR = crate::BitReader;
#[doc = "Field `WrProtOfGPIO01C` writer - Write Protection of GPIO01C"]
pub type WrProtOfGpio01cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfGPIO020` reader - Write Protection of GPIO020"]
pub type WrProtOfGpio020R = crate::BitReader;
#[doc = "Field `WrProtOfGPIO020` writer - Write Protection of GPIO020"]
pub type WrProtOfGpio020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfGPIO03C3116` reader - Write Protection of GPIO03C\\[31:16\\]"]
pub type WrProtOfGpio03c3116R = crate::BitReader;
#[doc = "Field `WrProtOfGPIO03C3116` writer - Write Protection of GPIO03C\\[31:16\\]"]
pub type WrProtOfGpio03c3116W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reset Tolerance of GPIO03C\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstToleranceOfGpio03c0 {
    #[doc = "0: SRST\\#"]
    Srst = 0,
    #[doc = "1: WDT reset"]
    WdtReset = 1,
}
impl From<RstToleranceOfGpio03c0> for bool {
    #[inline(always)]
    fn from(variant: RstToleranceOfGpio03c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstToleranceOfGPIO03C0` reader - Reset Tolerance of GPIO03C\\[0\\]"]
pub type RstToleranceOfGpio03c0R = crate::BitReader<RstToleranceOfGpio03c0>;
impl RstToleranceOfGpio03c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstToleranceOfGpio03c0 {
        match self.bits {
            false => RstToleranceOfGpio03c0::Srst,
            true => RstToleranceOfGpio03c0::WdtReset,
        }
    }
    #[doc = "SRST\\#"]
    #[inline(always)]
    pub fn is_srst(&self) -> bool {
        *self == RstToleranceOfGpio03c0::Srst
    }
    #[doc = "WDT reset"]
    #[inline(always)]
    pub fn is_wdt_reset(&self) -> bool {
        *self == RstToleranceOfGpio03c0::WdtReset
    }
}
#[doc = "Field `RstToleranceOfGPIO03C0` writer - Reset Tolerance of GPIO03C\\[0\\]"]
pub type RstToleranceOfGpio03c0W<'a, REG> = crate::BitWriter<'a, REG, RstToleranceOfGpio03c0>;
impl<'a, REG> RstToleranceOfGpio03c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRST\\#"]
    #[inline(always)]
    pub fn srst(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceOfGpio03c0::Srst)
    }
    #[doc = "WDT reset"]
    #[inline(always)]
    pub fn wdt_reset(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceOfGpio03c0::WdtReset)
    }
}
#[doc = "Field `RstToleranceOfGPIO03C1` reader - Reset Tolerance of GPIO03C\\[1\\]"]
pub type RstToleranceOfGpio03c1R = crate::BitReader;
#[doc = "Field `RstToleranceOfGPIO03C1` writer - Reset Tolerance of GPIO03C\\[1\\]"]
pub type RstToleranceOfGpio03c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfGPIO03C2` reader - Reset Tolerance of GPIO03C\\[2\\]"]
pub type RstToleranceOfGpio03c2R = crate::BitReader;
#[doc = "Field `RstToleranceOfGPIO03C2` writer - Reset Tolerance of GPIO03C\\[2\\]"]
pub type RstToleranceOfGpio03c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfGPIO03C3` reader - Reset Tolerance of GPIO03C\\[3\\]"]
pub type RstToleranceOfGpio03c3R = crate::BitReader;
#[doc = "Field `RstToleranceOfGPIO03C3` writer - Reset Tolerance of GPIO03C\\[3\\]"]
pub type RstToleranceOfGpio03c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfGPIO03C4` reader - Reset Tolerance of GPIO03C\\[4\\]"]
pub type RstToleranceOfGpio03c4R = crate::BitReader;
#[doc = "Field `RstToleranceOfGPIO03C4` writer - Reset Tolerance of GPIO03C\\[4\\]"]
pub type RstToleranceOfGpio03c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfGPIO03C5` reader - Reset Tolerance of GPIO03C\\[5\\]"]
pub type RstToleranceOfGpio03c5R = crate::BitReader;
#[doc = "Field `RstToleranceOfGPIO03C5` writer - Reset Tolerance of GPIO03C\\[5\\]"]
pub type RstToleranceOfGpio03c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfGPIO03C6` reader - Reset Tolerance of GPIO03C\\[6\\]"]
pub type RstToleranceOfGpio03c6R = crate::BitReader;
#[doc = "Field `RstToleranceOfGPIO03C6` writer - Reset Tolerance of GPIO03C\\[6\\]"]
pub type RstToleranceOfGpio03c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfGPIO03C7` reader - Reset Tolerance of GPIO03C\\[7\\]"]
pub type RstToleranceOfGpio03c7R = crate::BitReader;
#[doc = "Field `RstToleranceOfGPIO03C7` writer - Reset Tolerance of GPIO03C\\[7\\]"]
pub type RstToleranceOfGpio03c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfGPIO03C8` reader - Reset Tolerance of GPIO03C\\[8\\]"]
pub type RstToleranceOfGpio03c8R = crate::BitReader;
#[doc = "Field `RstToleranceOfGPIO03C8` writer - Reset Tolerance of GPIO03C\\[8\\]"]
pub type RstToleranceOfGpio03c8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Protection of GPIO000"]
    #[inline(always)]
    pub fn wr_prot_of_gpio000(&self) -> WrProtOfGpio000R {
        WrProtOfGpio000R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protection of GPIO004"]
    #[inline(always)]
    pub fn wr_prot_of_gpio004(&self) -> WrProtOfGpio004R {
        WrProtOfGpio004R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Protection of GPIO008"]
    #[inline(always)]
    pub fn wr_prot_of_gpio008(&self) -> WrProtOfGpio008R {
        WrProtOfGpio008R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write Protection of GPIO00C"]
    #[inline(always)]
    pub fn wr_prot_of_gpio00c(&self) -> WrProtOfGpio00cR {
        WrProtOfGpio00cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Protection of GPIO010"]
    #[inline(always)]
    pub fn wr_prot_of_gpio010(&self) -> WrProtOfGpio010R {
        WrProtOfGpio010R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protection of GPIO014"]
    #[inline(always)]
    pub fn wr_prot_of_gpio014(&self) -> WrProtOfGpio014R {
        WrProtOfGpio014R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Protection of GPIO018"]
    #[inline(always)]
    pub fn wr_prot_of_gpio018(&self) -> WrProtOfGpio018R {
        WrProtOfGpio018R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Protection of GPIO01C"]
    #[inline(always)]
    pub fn wr_prot_of_gpio01c(&self) -> WrProtOfGpio01cR {
        WrProtOfGpio01cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Protection of GPIO020"]
    #[inline(always)]
    pub fn wr_prot_of_gpio020(&self) -> WrProtOfGpio020R {
        WrProtOfGpio020R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write Protection of GPIO03C\\[31:16\\]"]
    #[inline(always)]
    pub fn wr_prot_of_gpio03c3116(&self) -> WrProtOfGpio03c3116R {
        WrProtOfGpio03c3116R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset Tolerance of GPIO03C\\[0\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c0(&self) -> RstToleranceOfGpio03c0R {
        RstToleranceOfGpio03c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset Tolerance of GPIO03C\\[1\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c1(&self) -> RstToleranceOfGpio03c1R {
        RstToleranceOfGpio03c1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Tolerance of GPIO03C\\[2\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c2(&self) -> RstToleranceOfGpio03c2R {
        RstToleranceOfGpio03c2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset Tolerance of GPIO03C\\[3\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c3(&self) -> RstToleranceOfGpio03c3R {
        RstToleranceOfGpio03c3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset Tolerance of GPIO03C\\[4\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c4(&self) -> RstToleranceOfGpio03c4R {
        RstToleranceOfGpio03c4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset Tolerance of GPIO03C\\[5\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c5(&self) -> RstToleranceOfGpio03c5R {
        RstToleranceOfGpio03c5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset Tolerance of GPIO03C\\[6\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c6(&self) -> RstToleranceOfGpio03c6R {
        RstToleranceOfGpio03c6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset Tolerance of GPIO03C\\[7\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c7(&self) -> RstToleranceOfGpio03c7R {
        RstToleranceOfGpio03c7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset Tolerance of GPIO03C\\[8\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c8(&self) -> RstToleranceOfGpio03c8R {
        RstToleranceOfGpio03c8R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection of GPIO000"]
    #[inline(always)]
    pub fn wr_prot_of_gpio000(&mut self) -> WrProtOfGpio000W<Gpio03cSpec> {
        WrProtOfGpio000W::new(self, 0)
    }
    #[doc = "Bit 1 - Write Protection of GPIO004"]
    #[inline(always)]
    pub fn wr_prot_of_gpio004(&mut self) -> WrProtOfGpio004W<Gpio03cSpec> {
        WrProtOfGpio004W::new(self, 1)
    }
    #[doc = "Bit 2 - Write Protection of GPIO008"]
    #[inline(always)]
    pub fn wr_prot_of_gpio008(&mut self) -> WrProtOfGpio008W<Gpio03cSpec> {
        WrProtOfGpio008W::new(self, 2)
    }
    #[doc = "Bit 3 - Write Protection of GPIO00C"]
    #[inline(always)]
    pub fn wr_prot_of_gpio00c(&mut self) -> WrProtOfGpio00cW<Gpio03cSpec> {
        WrProtOfGpio00cW::new(self, 3)
    }
    #[doc = "Bit 4 - Write Protection of GPIO010"]
    #[inline(always)]
    pub fn wr_prot_of_gpio010(&mut self) -> WrProtOfGpio010W<Gpio03cSpec> {
        WrProtOfGpio010W::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protection of GPIO014"]
    #[inline(always)]
    pub fn wr_prot_of_gpio014(&mut self) -> WrProtOfGpio014W<Gpio03cSpec> {
        WrProtOfGpio014W::new(self, 5)
    }
    #[doc = "Bit 6 - Write Protection of GPIO018"]
    #[inline(always)]
    pub fn wr_prot_of_gpio018(&mut self) -> WrProtOfGpio018W<Gpio03cSpec> {
        WrProtOfGpio018W::new(self, 6)
    }
    #[doc = "Bit 7 - Write Protection of GPIO01C"]
    #[inline(always)]
    pub fn wr_prot_of_gpio01c(&mut self) -> WrProtOfGpio01cW<Gpio03cSpec> {
        WrProtOfGpio01cW::new(self, 7)
    }
    #[doc = "Bit 8 - Write Protection of GPIO020"]
    #[inline(always)]
    pub fn wr_prot_of_gpio020(&mut self) -> WrProtOfGpio020W<Gpio03cSpec> {
        WrProtOfGpio020W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Gpio03cSpec> {
        Reserved6W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Gpio03cSpec> {
        Reserved5W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Gpio03cSpec> {
        Reserved4W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Gpio03cSpec> {
        Reserved3W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Gpio03cSpec> {
        Reserved2W::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpio03cSpec> {
        Reserved1W::new(self, 14)
    }
    #[doc = "Bit 15 - Write Protection of GPIO03C\\[31:16\\]"]
    #[inline(always)]
    pub fn wr_prot_of_gpio03c3116(&mut self) -> WrProtOfGpio03c3116W<Gpio03cSpec> {
        WrProtOfGpio03c3116W::new(self, 15)
    }
    #[doc = "Bit 16 - Reset Tolerance of GPIO03C\\[0\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c0(&mut self) -> RstToleranceOfGpio03c0W<Gpio03cSpec> {
        RstToleranceOfGpio03c0W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset Tolerance of GPIO03C\\[1\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c1(&mut self) -> RstToleranceOfGpio03c1W<Gpio03cSpec> {
        RstToleranceOfGpio03c1W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset Tolerance of GPIO03C\\[2\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c2(&mut self) -> RstToleranceOfGpio03c2W<Gpio03cSpec> {
        RstToleranceOfGpio03c2W::new(self, 18)
    }
    #[doc = "Bit 19 - Reset Tolerance of GPIO03C\\[3\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c3(&mut self) -> RstToleranceOfGpio03c3W<Gpio03cSpec> {
        RstToleranceOfGpio03c3W::new(self, 19)
    }
    #[doc = "Bit 20 - Reset Tolerance of GPIO03C\\[4\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c4(&mut self) -> RstToleranceOfGpio03c4W<Gpio03cSpec> {
        RstToleranceOfGpio03c4W::new(self, 20)
    }
    #[doc = "Bit 21 - Reset Tolerance of GPIO03C\\[5\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c5(&mut self) -> RstToleranceOfGpio03c5W<Gpio03cSpec> {
        RstToleranceOfGpio03c5W::new(self, 21)
    }
    #[doc = "Bit 22 - Reset Tolerance of GPIO03C\\[6\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c6(&mut self) -> RstToleranceOfGpio03c6W<Gpio03cSpec> {
        RstToleranceOfGpio03c6W::new(self, 22)
    }
    #[doc = "Bit 23 - Reset Tolerance of GPIO03C\\[7\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c7(&mut self) -> RstToleranceOfGpio03c7W<Gpio03cSpec> {
        RstToleranceOfGpio03c7W::new(self, 23)
    }
    #[doc = "Bit 24 - Reset Tolerance of GPIO03C\\[8\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_gpio03c8(&mut self) -> RstToleranceOfGpio03c8W<Gpio03cSpec> {
        RstToleranceOfGpio03c8W::new(self, 24)
    }
}
#[doc = "Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio03cSpec;
impl crate::RegisterSpec for Gpio03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio03c::R`](R) reader structure"]
impl crate::Readable for Gpio03cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio03c::W`](W) writer structure"]
impl crate::Writable for Gpio03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO03C to value 0"]
impl crate::Resettable for Gpio03cSpec {}
