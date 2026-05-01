#[doc = "Register `GPIO010` reader"]
pub type R = crate::R<Gpio010Spec>;
#[doc = "Register `GPIO010` writer"]
pub type W = crate::W<Gpio010Spec>;
#[doc = "Field `EnblBlinkCounter1` reader - Enable Blink Counter #1"]
pub type EnblBlinkCounter1R = crate::BitReader;
#[doc = "Field `EnblBlinkCounter1` writer - Enable Blink Counter #1"]
pub type EnblBlinkCounter1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Source of Counter #1 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSourceOfCounter1sel {
    #[doc = "0: Choose PCLK/(\\hlink{GPIO00C+1) as source clock."]
    ChoosePclkhlinkGpio00c1AsSourceClock = 0,
    #[doc = "1: Choose CLK1M as source clock."]
    ChooseClk1mAsSourceClock = 1,
}
impl From<ClkSourceOfCounter1sel> for bool {
    #[inline(always)]
    fn from(variant: ClkSourceOfCounter1sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSourceOfCounter1Sel` reader - Clock Source of Counter #1 Selection"]
pub type ClkSourceOfCounter1selR = crate::BitReader<ClkSourceOfCounter1sel>;
impl ClkSourceOfCounter1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSourceOfCounter1sel {
        match self.bits {
            false => ClkSourceOfCounter1sel::ChoosePclkhlinkGpio00c1AsSourceClock,
            true => ClkSourceOfCounter1sel::ChooseClk1mAsSourceClock,
        }
    }
    #[doc = "Choose PCLK/(\\hlink{GPIO00C+1) as source clock."]
    #[inline(always)]
    pub fn is_choose_pclkhlink_gpio00c1_as_source_clock(&self) -> bool {
        *self == ClkSourceOfCounter1sel::ChoosePclkhlinkGpio00c1AsSourceClock
    }
    #[doc = "Choose CLK1M as source clock."]
    #[inline(always)]
    pub fn is_choose_clk1m_as_source_clock(&self) -> bool {
        *self == ClkSourceOfCounter1sel::ChooseClk1mAsSourceClock
    }
}
#[doc = "Field `ClkSourceOfCounter1Sel` writer - Clock Source of Counter #1 Selection"]
pub type ClkSourceOfCounter1selW<'a, REG> = crate::BitWriter<'a, REG, ClkSourceOfCounter1sel>;
impl<'a, REG> ClkSourceOfCounter1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Choose PCLK/(\\hlink{GPIO00C+1) as source clock."]
    #[inline(always)]
    pub fn choose_pclkhlink_gpio00c1_as_source_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSourceOfCounter1sel::ChoosePclkhlinkGpio00c1AsSourceClock)
    }
    #[doc = "Choose CLK1M as source clock."]
    #[inline(always)]
    pub fn choose_clk1m_as_source_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSourceOfCounter1sel::ChooseClk1mAsSourceClock)
    }
}
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EnblBlinkCounter2` reader - Enable Blink Counter #2"]
pub type EnblBlinkCounter2R = crate::BitReader;
#[doc = "Field `EnblBlinkCounter2` writer - Enable Blink Counter #2"]
pub type EnblBlinkCounter2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Source of Counter #2 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSourceOfCounter2sel {
    #[doc = "0: Choose PCLK/(\\hlink{GPIO00C+1) as source clock."]
    ChoosePclkhlinkGpio00c1AsSourceClock = 0,
    #[doc = "1: Choose CLK1M as source clock."]
    ChooseClk1mAsSourceClock = 1,
}
impl From<ClkSourceOfCounter2sel> for bool {
    #[inline(always)]
    fn from(variant: ClkSourceOfCounter2sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSourceOfCounter2Sel` reader - Clock Source of Counter #2 Selection"]
pub type ClkSourceOfCounter2selR = crate::BitReader<ClkSourceOfCounter2sel>;
impl ClkSourceOfCounter2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSourceOfCounter2sel {
        match self.bits {
            false => ClkSourceOfCounter2sel::ChoosePclkhlinkGpio00c1AsSourceClock,
            true => ClkSourceOfCounter2sel::ChooseClk1mAsSourceClock,
        }
    }
    #[doc = "Choose PCLK/(\\hlink{GPIO00C+1) as source clock."]
    #[inline(always)]
    pub fn is_choose_pclkhlink_gpio00c1_as_source_clock(&self) -> bool {
        *self == ClkSourceOfCounter2sel::ChoosePclkhlinkGpio00c1AsSourceClock
    }
    #[doc = "Choose CLK1M as source clock."]
    #[inline(always)]
    pub fn is_choose_clk1m_as_source_clock(&self) -> bool {
        *self == ClkSourceOfCounter2sel::ChooseClk1mAsSourceClock
    }
}
#[doc = "Field `ClkSourceOfCounter2Sel` writer - Clock Source of Counter #2 Selection"]
pub type ClkSourceOfCounter2selW<'a, REG> = crate::BitWriter<'a, REG, ClkSourceOfCounter2sel>;
impl<'a, REG> ClkSourceOfCounter2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Choose PCLK/(\\hlink{GPIO00C+1) as source clock."]
    #[inline(always)]
    pub fn choose_pclkhlink_gpio00c1_as_source_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSourceOfCounter2sel::ChoosePclkhlinkGpio00c1AsSourceClock)
    }
    #[doc = "Choose CLK1M as source clock."]
    #[inline(always)]
    pub fn choose_clk1m_as_source_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSourceOfCounter2sel::ChooseClk1mAsSourceClock)
    }
}
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `EnblBlinkCounter3` reader - Enable Blink Counter #3"]
pub type EnblBlinkCounter3R = crate::BitReader;
#[doc = "Field `EnblBlinkCounter3` writer - Enable Blink Counter #3"]
pub type EnblBlinkCounter3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock Source of Counter #3 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkSourceOfCounter3sel {
    #[doc = "0: Choose PCLK/(\\hlink{GPIO00C+1) as source clock."]
    ChoosePclkhlinkGpio00c1AsSourceClock = 0,
    #[doc = "1: Choose CLK1M as source clock."]
    ChooseClk1mAsSourceClock = 1,
}
impl From<ClkSourceOfCounter3sel> for bool {
    #[inline(always)]
    fn from(variant: ClkSourceOfCounter3sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ClkSourceOfCounter3Sel` reader - Clock Source of Counter #3 Selection"]
pub type ClkSourceOfCounter3selR = crate::BitReader<ClkSourceOfCounter3sel>;
impl ClkSourceOfCounter3selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkSourceOfCounter3sel {
        match self.bits {
            false => ClkSourceOfCounter3sel::ChoosePclkhlinkGpio00c1AsSourceClock,
            true => ClkSourceOfCounter3sel::ChooseClk1mAsSourceClock,
        }
    }
    #[doc = "Choose PCLK/(\\hlink{GPIO00C+1) as source clock."]
    #[inline(always)]
    pub fn is_choose_pclkhlink_gpio00c1_as_source_clock(&self) -> bool {
        *self == ClkSourceOfCounter3sel::ChoosePclkhlinkGpio00c1AsSourceClock
    }
    #[doc = "Choose CLK1M as source clock."]
    #[inline(always)]
    pub fn is_choose_clk1m_as_source_clock(&self) -> bool {
        *self == ClkSourceOfCounter3sel::ChooseClk1mAsSourceClock
    }
}
#[doc = "Field `ClkSourceOfCounter3Sel` writer - Clock Source of Counter #3 Selection"]
pub type ClkSourceOfCounter3selW<'a, REG> = crate::BitWriter<'a, REG, ClkSourceOfCounter3sel>;
impl<'a, REG> ClkSourceOfCounter3selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Choose PCLK/(\\hlink{GPIO00C+1) as source clock."]
    #[inline(always)]
    pub fn choose_pclkhlink_gpio00c1_as_source_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSourceOfCounter3sel::ChoosePclkhlinkGpio00c1AsSourceClock)
    }
    #[doc = "Choose CLK1M as source clock."]
    #[inline(always)]
    pub fn choose_clk1m_as_source_clock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSourceOfCounter3sel::ChooseClk1mAsSourceClock)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Blink Counter #1"]
    #[inline(always)]
    pub fn enbl_blink_counter1(&self) -> EnblBlinkCounter1R {
        EnblBlinkCounter1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Source of Counter #1 Selection"]
    #[inline(always)]
    pub fn clk_source_of_counter1sel(&self) -> ClkSourceOfCounter1selR {
        ClkSourceOfCounter1selR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Enable Blink Counter #2"]
    #[inline(always)]
    pub fn enbl_blink_counter2(&self) -> EnblBlinkCounter2R {
        EnblBlinkCounter2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Source of Counter #2 Selection"]
    #[inline(always)]
    pub fn clk_source_of_counter2sel(&self) -> ClkSourceOfCounter2selR {
        ClkSourceOfCounter2selR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable Blink Counter #3"]
    #[inline(always)]
    pub fn enbl_blink_counter3(&self) -> EnblBlinkCounter3R {
        EnblBlinkCounter3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Source of Counter #3 Selection"]
    #[inline(always)]
    pub fn clk_source_of_counter3sel(&self) -> ClkSourceOfCounter3selR {
        ClkSourceOfCounter3selR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Blink Counter #1"]
    #[inline(always)]
    pub fn enbl_blink_counter1(&mut self) -> EnblBlinkCounter1W<Gpio010Spec> {
        EnblBlinkCounter1W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock Source of Counter #1 Selection"]
    #[inline(always)]
    pub fn clk_source_of_counter1sel(&mut self) -> ClkSourceOfCounter1selW<Gpio010Spec> {
        ClkSourceOfCounter1selW::new(self, 1)
    }
    #[doc = "Bit 4 - Enable Blink Counter #2"]
    #[inline(always)]
    pub fn enbl_blink_counter2(&mut self) -> EnblBlinkCounter2W<Gpio010Spec> {
        EnblBlinkCounter2W::new(self, 4)
    }
    #[doc = "Bit 5 - Clock Source of Counter #2 Selection"]
    #[inline(always)]
    pub fn clk_source_of_counter2sel(&mut self) -> ClkSourceOfCounter2selW<Gpio010Spec> {
        ClkSourceOfCounter2selW::new(self, 5)
    }
    #[doc = "Bit 8 - Enable Blink Counter #3"]
    #[inline(always)]
    pub fn enbl_blink_counter3(&mut self) -> EnblBlinkCounter3W<Gpio010Spec> {
        EnblBlinkCounter3W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock Source of Counter #3 Selection"]
    #[inline(always)]
    pub fn clk_source_of_counter3sel(&mut self) -> ClkSourceOfCounter3selW<Gpio010Spec> {
        ClkSourceOfCounter3selW::new(self, 9)
    }
}
#[doc = "GPIO Blink Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio010Spec;
impl crate::RegisterSpec for Gpio010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio010::R`](R) reader structure"]
impl crate::Readable for Gpio010Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio010::W`](W) writer structure"]
impl crate::Writable for Gpio010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO010 to value 0"]
impl crate::Resettable for Gpio010Spec {}
