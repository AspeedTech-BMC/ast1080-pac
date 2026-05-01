#[doc = "Register `SGPIO03C` reader"]
pub type R = crate::R<Sgpio03cSpec>;
#[doc = "Register `SGPIO03C` writer"]
pub type W = crate::W<Sgpio03cSpec>;
#[doc = "Field `WrProtOfSGPIO000` reader - Write Protection of SGPIO000"]
pub type WrProtOfSgpio000R = crate::BitReader;
#[doc = "Field `WrProtOfSGPIO000` writer - Write Protection of SGPIO000"]
pub type WrProtOfSgpio000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WrProtOfSGPIO020` reader - Write Protection of SGPIO020"]
pub type WrProtOfSgpio020R = crate::BitReader;
#[doc = "Field `WrProtOfSGPIO020` writer - Write Protection of SGPIO020"]
pub type WrProtOfSgpio020W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WrProtOfSGPIO03C3116` reader - Write Protection of SGPIO03C\\[31:16\\]"]
pub type WrProtOfSgpio03c3116R = crate::BitReader;
#[doc = "Field `WrProtOfSGPIO03C3116` writer - Write Protection of SGPIO03C\\[31:16\\]"]
pub type WrProtOfSgpio03c3116W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reset Tolerance of SGPIO03C\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstToleranceOfSgpio03c0 {
    #[doc = "0: SRST\\#"]
    Srst = 0,
    #[doc = "1: WDT reset"]
    WdtReset = 1,
}
impl From<RstToleranceOfSgpio03c0> for bool {
    #[inline(always)]
    fn from(variant: RstToleranceOfSgpio03c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstToleranceOfSGPIO03C0` reader - Reset Tolerance of SGPIO03C\\[0\\]"]
pub type RstToleranceOfSgpio03c0R = crate::BitReader<RstToleranceOfSgpio03c0>;
impl RstToleranceOfSgpio03c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstToleranceOfSgpio03c0 {
        match self.bits {
            false => RstToleranceOfSgpio03c0::Srst,
            true => RstToleranceOfSgpio03c0::WdtReset,
        }
    }
    #[doc = "SRST\\#"]
    #[inline(always)]
    pub fn is_srst(&self) -> bool {
        *self == RstToleranceOfSgpio03c0::Srst
    }
    #[doc = "WDT reset"]
    #[inline(always)]
    pub fn is_wdt_reset(&self) -> bool {
        *self == RstToleranceOfSgpio03c0::WdtReset
    }
}
#[doc = "Field `RstToleranceOfSGPIO03C0` writer - Reset Tolerance of SGPIO03C\\[0\\]"]
pub type RstToleranceOfSgpio03c0W<'a, REG> = crate::BitWriter<'a, REG, RstToleranceOfSgpio03c0>;
impl<'a, REG> RstToleranceOfSgpio03c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SRST\\#"]
    #[inline(always)]
    pub fn srst(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceOfSgpio03c0::Srst)
    }
    #[doc = "WDT reset"]
    #[inline(always)]
    pub fn wdt_reset(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceOfSgpio03c0::WdtReset)
    }
}
#[doc = "Field `RstToleranceOfSGPIO03C1` reader - Reset Tolerance of SGPIO03C\\[1\\]"]
pub type RstToleranceOfSgpio03c1R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO03C1` writer - Reset Tolerance of SGPIO03C\\[1\\]"]
pub type RstToleranceOfSgpio03c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO03C2` reader - Reset Tolerance of SGPIO03C\\[2\\]"]
pub type RstToleranceOfSgpio03c2R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO03C2` writer - Reset Tolerance of SGPIO03C\\[2\\]"]
pub type RstToleranceOfSgpio03c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO03C3` reader - Reset Tolerance of SGPIO03C\\[3\\]"]
pub type RstToleranceOfSgpio03c3R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO03C3` writer - Reset Tolerance of SGPIO03C\\[3\\]"]
pub type RstToleranceOfSgpio03c3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO03C4` reader - Reset Tolerance of SGPIO03C\\[4\\]"]
pub type RstToleranceOfSgpio03c4R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO03C4` writer - Reset Tolerance of SGPIO03C\\[4\\]"]
pub type RstToleranceOfSgpio03c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO03C5` reader - Reset Tolerance of SGPIO03C\\[5\\]"]
pub type RstToleranceOfSgpio03c5R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO03C5` writer - Reset Tolerance of SGPIO03C\\[5\\]"]
pub type RstToleranceOfSgpio03c5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO03C6` reader - Reset Tolerance of SGPIO03C\\[6\\]"]
pub type RstToleranceOfSgpio03c6R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO03C6` writer - Reset Tolerance of SGPIO03C\\[6\\]"]
pub type RstToleranceOfSgpio03c6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO03C7` reader - Reset Tolerance of SGPIO03C\\[7\\]"]
pub type RstToleranceOfSgpio03c7R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO03C7` writer - Reset Tolerance of SGPIO03C\\[7\\]"]
pub type RstToleranceOfSgpio03c7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO03C8` reader - Reset Tolerance of SGPIO03C\\[8\\]"]
pub type RstToleranceOfSgpio03c8R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO03C8` writer - Reset Tolerance of SGPIO03C\\[8\\]"]
pub type RstToleranceOfSgpio03c8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Protection of SGPIO000"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio000(&self) -> WrProtOfSgpio000R {
        WrProtOfSgpio000R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Write Protection of SGPIO020"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio020(&self) -> WrProtOfSgpio020R {
        WrProtOfSgpio020R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Write Protection of SGPIO03C\\[31:16\\]"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio03c3116(&self) -> WrProtOfSgpio03c3116R {
        WrProtOfSgpio03c3116R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset Tolerance of SGPIO03C\\[0\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c0(&self) -> RstToleranceOfSgpio03c0R {
        RstToleranceOfSgpio03c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset Tolerance of SGPIO03C\\[1\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c1(&self) -> RstToleranceOfSgpio03c1R {
        RstToleranceOfSgpio03c1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Tolerance of SGPIO03C\\[2\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c2(&self) -> RstToleranceOfSgpio03c2R {
        RstToleranceOfSgpio03c2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset Tolerance of SGPIO03C\\[3\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c3(&self) -> RstToleranceOfSgpio03c3R {
        RstToleranceOfSgpio03c3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset Tolerance of SGPIO03C\\[4\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c4(&self) -> RstToleranceOfSgpio03c4R {
        RstToleranceOfSgpio03c4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset Tolerance of SGPIO03C\\[5\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c5(&self) -> RstToleranceOfSgpio03c5R {
        RstToleranceOfSgpio03c5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset Tolerance of SGPIO03C\\[6\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c6(&self) -> RstToleranceOfSgpio03c6R {
        RstToleranceOfSgpio03c6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset Tolerance of SGPIO03C\\[7\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c7(&self) -> RstToleranceOfSgpio03c7R {
        RstToleranceOfSgpio03c7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset Tolerance of SGPIO03C\\[8\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c8(&self) -> RstToleranceOfSgpio03c8R {
        RstToleranceOfSgpio03c8R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection of SGPIO000"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio000(&mut self) -> WrProtOfSgpio000W<Sgpio03cSpec> {
        WrProtOfSgpio000W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Sgpio03cSpec> {
        Reserved2W::new(self, 1)
    }
    #[doc = "Bit 8 - Write Protection of SGPIO020"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio020(&mut self) -> WrProtOfSgpio020W<Sgpio03cSpec> {
        WrProtOfSgpio020W::new(self, 8)
    }
    #[doc = "Bits 9:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Sgpio03cSpec> {
        Reserved1W::new(self, 9)
    }
    #[doc = "Bit 15 - Write Protection of SGPIO03C\\[31:16\\]"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio03c3116(&mut self) -> WrProtOfSgpio03c3116W<Sgpio03cSpec> {
        WrProtOfSgpio03c3116W::new(self, 15)
    }
    #[doc = "Bit 16 - Reset Tolerance of SGPIO03C\\[0\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c0(&mut self) -> RstToleranceOfSgpio03c0W<Sgpio03cSpec> {
        RstToleranceOfSgpio03c0W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset Tolerance of SGPIO03C\\[1\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c1(&mut self) -> RstToleranceOfSgpio03c1W<Sgpio03cSpec> {
        RstToleranceOfSgpio03c1W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset Tolerance of SGPIO03C\\[2\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c2(&mut self) -> RstToleranceOfSgpio03c2W<Sgpio03cSpec> {
        RstToleranceOfSgpio03c2W::new(self, 18)
    }
    #[doc = "Bit 19 - Reset Tolerance of SGPIO03C\\[3\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c3(&mut self) -> RstToleranceOfSgpio03c3W<Sgpio03cSpec> {
        RstToleranceOfSgpio03c3W::new(self, 19)
    }
    #[doc = "Bit 20 - Reset Tolerance of SGPIO03C\\[4\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c4(&mut self) -> RstToleranceOfSgpio03c4W<Sgpio03cSpec> {
        RstToleranceOfSgpio03c4W::new(self, 20)
    }
    #[doc = "Bit 21 - Reset Tolerance of SGPIO03C\\[5\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c5(&mut self) -> RstToleranceOfSgpio03c5W<Sgpio03cSpec> {
        RstToleranceOfSgpio03c5W::new(self, 21)
    }
    #[doc = "Bit 22 - Reset Tolerance of SGPIO03C\\[6\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c6(&mut self) -> RstToleranceOfSgpio03c6W<Sgpio03cSpec> {
        RstToleranceOfSgpio03c6W::new(self, 22)
    }
    #[doc = "Bit 23 - Reset Tolerance of SGPIO03C\\[7\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c7(&mut self) -> RstToleranceOfSgpio03c7W<Sgpio03cSpec> {
        RstToleranceOfSgpio03c7W::new(self, 23)
    }
    #[doc = "Bit 24 - Reset Tolerance of SGPIO03C\\[8\\]"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03c8(&mut self) -> RstToleranceOfSgpio03c8W<Sgpio03cSpec> {
        RstToleranceOfSgpio03c8W::new(self, 24)
    }
}
#[doc = "Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio03cSpec;
impl crate::RegisterSpec for Sgpio03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio03c::R`](R) reader structure"]
impl crate::Readable for Sgpio03cSpec {}
#[doc = "`write(|w| ..)` method takes [`sgpio03c::W`](W) writer structure"]
impl crate::Writable for Sgpio03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO03C to value 0"]
impl crate::Resettable for Sgpio03cSpec {}
