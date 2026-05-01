#[doc = "Register `SGPIO3C` reader"]
pub type R = crate::R<Sgpio3cSpec>;
#[doc = "Register `SGPIO3C` writer"]
pub type W = crate::W<Sgpio3cSpec>;
#[doc = "Field `WrProtOfSGPIO00` reader - Write Protection of hlinkSGPIO00"]
pub type WrProtOfSgpio00R = crate::BitReader;
#[doc = "Field `WrProtOfSGPIO00` writer - Write Protection of hlinkSGPIO00"]
pub type WrProtOfSgpio00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WrProtOfSGPIO02` reader - Write Protection of hlinkSGPIO02"]
pub type WrProtOfSgpio02R = crate::BitReader;
#[doc = "Field `WrProtOfSGPIO02` writer - Write Protection of hlinkSGPIO02"]
pub type WrProtOfSgpio02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WrProtOfSGPIO03` reader - Write Protection of hlinkSGPIO03"]
pub type WrProtOfSgpio03R = crate::BitReader;
#[doc = "Field `WrProtOfSGPIO03` writer - Write Protection of hlinkSGPIO03"]
pub type WrProtOfSgpio03W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO038` reader - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio038R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO038` writer - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio038W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO037` reader - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio037R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO037` writer - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio037W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO036` reader - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio036R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO036` writer - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio036W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO035` reader - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio035R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO035` writer - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio035W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO034` reader - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio034R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO034` writer - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio034W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO033` reader - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio033R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO033` writer - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio033W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO032` reader - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio032R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO032` writer - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio032W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO031` reader - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio031R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO031` writer - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio031W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RstToleranceOfSGPIO03` reader - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio03R = crate::BitReader;
#[doc = "Field `RstToleranceOfSGPIO03` writer - Reset Tolerance of hlinkSGPIO03"]
pub type RstToleranceOfSgpio03W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Protection of hlinkSGPIO00"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio00(&self) -> WrProtOfSgpio00R {
        WrProtOfSgpio00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Write Protection of hlinkSGPIO02"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio02(&self) -> WrProtOfSgpio02R {
        WrProtOfSgpio02R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Write Protection of hlinkSGPIO03"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio03(&self) -> WrProtOfSgpio03R {
        WrProtOfSgpio03R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio038(&self) -> RstToleranceOfSgpio038R {
        RstToleranceOfSgpio038R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio037(&self) -> RstToleranceOfSgpio037R {
        RstToleranceOfSgpio037R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio036(&self) -> RstToleranceOfSgpio036R {
        RstToleranceOfSgpio036R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio035(&self) -> RstToleranceOfSgpio035R {
        RstToleranceOfSgpio035R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio034(&self) -> RstToleranceOfSgpio034R {
        RstToleranceOfSgpio034R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio033(&self) -> RstToleranceOfSgpio033R {
        RstToleranceOfSgpio033R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio032(&self) -> RstToleranceOfSgpio032R {
        RstToleranceOfSgpio032R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio031(&self) -> RstToleranceOfSgpio031R {
        RstToleranceOfSgpio031R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03(&self) -> RstToleranceOfSgpio03R {
        RstToleranceOfSgpio03R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection of hlinkSGPIO00"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio00(&mut self) -> WrProtOfSgpio00W<Sgpio3cSpec> {
        WrProtOfSgpio00W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Sgpio3cSpec> {
        Reserved2W::new(self, 1)
    }
    #[doc = "Bit 8 - Write Protection of hlinkSGPIO02"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio02(&mut self) -> WrProtOfSgpio02W<Sgpio3cSpec> {
        WrProtOfSgpio02W::new(self, 8)
    }
    #[doc = "Bits 9:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Sgpio3cSpec> {
        Reserved1W::new(self, 9)
    }
    #[doc = "Bit 15 - Write Protection of hlinkSGPIO03"]
    #[inline(always)]
    pub fn wr_prot_of_sgpio03(&mut self) -> WrProtOfSgpio03W<Sgpio3cSpec> {
        WrProtOfSgpio03W::new(self, 15)
    }
    #[doc = "Bit 16 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio038(&mut self) -> RstToleranceOfSgpio038W<Sgpio3cSpec> {
        RstToleranceOfSgpio038W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio037(&mut self) -> RstToleranceOfSgpio037W<Sgpio3cSpec> {
        RstToleranceOfSgpio037W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio036(&mut self) -> RstToleranceOfSgpio036W<Sgpio3cSpec> {
        RstToleranceOfSgpio036W::new(self, 18)
    }
    #[doc = "Bit 19 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio035(&mut self) -> RstToleranceOfSgpio035W<Sgpio3cSpec> {
        RstToleranceOfSgpio035W::new(self, 19)
    }
    #[doc = "Bit 20 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio034(&mut self) -> RstToleranceOfSgpio034W<Sgpio3cSpec> {
        RstToleranceOfSgpio034W::new(self, 20)
    }
    #[doc = "Bit 21 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio033(&mut self) -> RstToleranceOfSgpio033W<Sgpio3cSpec> {
        RstToleranceOfSgpio033W::new(self, 21)
    }
    #[doc = "Bit 22 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio032(&mut self) -> RstToleranceOfSgpio032W<Sgpio3cSpec> {
        RstToleranceOfSgpio032W::new(self, 22)
    }
    #[doc = "Bit 23 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio031(&mut self) -> RstToleranceOfSgpio031W<Sgpio3cSpec> {
        RstToleranceOfSgpio031W::new(self, 23)
    }
    #[doc = "Bit 24 - Reset Tolerance of hlinkSGPIO03"]
    #[inline(always)]
    pub fn rst_tolerance_of_sgpio03(&mut self) -> RstToleranceOfSgpio03W<Sgpio3cSpec> {
        RstToleranceOfSgpio03W::new(self, 24)
    }
}
#[doc = "Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio3cSpec;
impl crate::RegisterSpec for Sgpio3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio3c::R`](R) reader structure"]
impl crate::Readable for Sgpio3cSpec {}
#[doc = "`write(|w| ..)` method takes [`sgpio3c::W`](W) writer structure"]
impl crate::Writable for Sgpio3cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO3C to value 0"]
impl crate::Resettable for Sgpio3cSpec {}
