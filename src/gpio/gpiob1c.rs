#[doc = "Register `GPIOB1C` reader"]
pub type R = crate::R<Gpiob1cSpec>;
#[doc = "Register `GPIOB1C` writer"]
pub type W = crate::W<Gpiob1cSpec>;
#[doc = "GPIO096 Write Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio096wrPrivilegeRstTolerance {
    #[doc = "0: Write Privilege of GPIO096 is reset by WDT."]
    WritePrivilegeOfGpio096IsResetByWdt = 0,
    #[doc = "1: Write Privilege of GPIO096 is NOT reset by WDT."]
    WritePrivilegeOfGpio096IsNotResetByWdt = 1,
}
impl From<Gpio096wrPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio096wrPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO096WrPrivilegeRstTolerance` reader - GPIO096 Write Privilege Reset Tolerance"]
pub type Gpio096wrPrivilegeRstToleranceR = crate::BitReader<Gpio096wrPrivilegeRstTolerance>;
impl Gpio096wrPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio096wrPrivilegeRstTolerance {
        match self.bits {
            false => Gpio096wrPrivilegeRstTolerance::WritePrivilegeOfGpio096IsResetByWdt,
            true => Gpio096wrPrivilegeRstTolerance::WritePrivilegeOfGpio096IsNotResetByWdt,
        }
    }
    #[doc = "Write Privilege of GPIO096 is reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio096_is_reset_by_wdt(&self) -> bool {
        *self == Gpio096wrPrivilegeRstTolerance::WritePrivilegeOfGpio096IsResetByWdt
    }
    #[doc = "Write Privilege of GPIO096 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio096_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio096wrPrivilegeRstTolerance::WritePrivilegeOfGpio096IsNotResetByWdt
    }
}
#[doc = "Field `GPIO096WrPrivilegeRstTolerance` writer - GPIO096 Write Privilege Reset Tolerance"]
pub type Gpio096wrPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio096wrPrivilegeRstTolerance>;
impl<'a, REG> Gpio096wrPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write Privilege of GPIO096 is reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio096_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio096wrPrivilegeRstTolerance::WritePrivilegeOfGpio096IsResetByWdt)
    }
    #[doc = "Write Privilege of GPIO096 is NOT reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio096_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio096wrPrivilegeRstTolerance::WritePrivilegeOfGpio096IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO097WrPrivilegeRstTolerance` reader - GPIO097 Write Privilege Reset Tolerance"]
pub type Gpio097wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO097WrPrivilegeRstTolerance` writer - GPIO097 Write Privilege Reset Tolerance"]
pub type Gpio097wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO098WrPrivilegeRstTolerance` reader - GPIO098 Write Privilege Reset Tolerance"]
pub type Gpio098wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO098WrPrivilegeRstTolerance` writer - GPIO098 Write Privilege Reset Tolerance"]
pub type Gpio098wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO099WrPrivilegeRstTolerance` reader - GPIO099 Write Privilege Reset Tolerance"]
pub type Gpio099wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO099WrPrivilegeRstTolerance` writer - GPIO099 Write Privilege Reset Tolerance"]
pub type Gpio099wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO100WrPrivilegeRstTolerance` reader - GPIO100 Write Privilege Reset Tolerance"]
pub type Gpio100wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO100WrPrivilegeRstTolerance` writer - GPIO100 Write Privilege Reset Tolerance"]
pub type Gpio100wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO101WrPrivilegeRstTolerance` reader - GPIO101 Write Privilege Reset Tolerance"]
pub type Gpio101wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO101WrPrivilegeRstTolerance` writer - GPIO101 Write Privilege Reset Tolerance"]
pub type Gpio101wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO102WrPrivilegeRstTolerance` reader - GPIO102 Write Privilege Reset Tolerance"]
pub type Gpio102wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO102WrPrivilegeRstTolerance` writer - GPIO102 Write Privilege Reset Tolerance"]
pub type Gpio102wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO103WrPrivilegeRstTolerance` reader - GPIO103 Write Privilege Reset Tolerance"]
pub type Gpio103wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO103WrPrivilegeRstTolerance` writer - GPIO103 Write Privilege Reset Tolerance"]
pub type Gpio103wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO104WrPrivilegeRstTolerance` reader - GPIO104 Write Privilege Reset Tolerance"]
pub type Gpio104wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO104WrPrivilegeRstTolerance` writer - GPIO104 Write Privilege Reset Tolerance"]
pub type Gpio104wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO105WrPrivilegeRstTolerance` reader - GPIO105 Write Privilege Reset Tolerance"]
pub type Gpio105wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO105WrPrivilegeRstTolerance` writer - GPIO105 Write Privilege Reset Tolerance"]
pub type Gpio105wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO106WrPrivilegeRstTolerance` reader - GPIO106 Write Privilege Reset Tolerance"]
pub type Gpio106wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO106WrPrivilegeRstTolerance` writer - GPIO106 Write Privilege Reset Tolerance"]
pub type Gpio106wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO107WrPrivilegeRstTolerance` reader - GPIO107 Write Privilege Reset Tolerance"]
pub type Gpio107wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO107WrPrivilegeRstTolerance` writer - GPIO107 Write Privilege Reset Tolerance"]
pub type Gpio107wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO108WrPrivilegeRstTolerance` reader - GPIO108 Write Privilege Reset Tolerance"]
pub type Gpio108wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO108WrPrivilegeRstTolerance` writer - GPIO108 Write Privilege Reset Tolerance"]
pub type Gpio108wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO109WrPrivilegeRstTolerance` reader - GPIO109 Write Privilege Reset Tolerance"]
pub type Gpio109wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO109WrPrivilegeRstTolerance` writer - GPIO109 Write Privilege Reset Tolerance"]
pub type Gpio109wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO110WrPrivilegeRstTolerance` reader - GPIO110 Write Privilege Reset Tolerance"]
pub type Gpio110wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO110WrPrivilegeRstTolerance` writer - GPIO110 Write Privilege Reset Tolerance"]
pub type Gpio110wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO111WrPrivilegeRstTolerance` reader - GPIO111 Write Privilege Reset Tolerance"]
pub type Gpio111wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO111WrPrivilegeRstTolerance` writer - GPIO111 Write Privilege Reset Tolerance"]
pub type Gpio111wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO112WrPrivilegeRstTolerance` reader - GPIO112 Write Privilege Reset Tolerance"]
pub type Gpio112wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO112WrPrivilegeRstTolerance` writer - GPIO112 Write Privilege Reset Tolerance"]
pub type Gpio112wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO113WrPrivilegeRstTolerance` reader - GPIO113 Write Privilege Reset Tolerance"]
pub type Gpio113wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO113WrPrivilegeRstTolerance` writer - GPIO113 Write Privilege Reset Tolerance"]
pub type Gpio113wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO114WrPrivilegeRstTolerance` reader - GPIO114 Write Privilege Reset Tolerance"]
pub type Gpio114wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO114WrPrivilegeRstTolerance` writer - GPIO114 Write Privilege Reset Tolerance"]
pub type Gpio114wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO115WrPrivilegeRstTolerance` reader - GPIO115 Write Privilege Reset Tolerance"]
pub type Gpio115wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO115WrPrivilegeRstTolerance` writer - GPIO115 Write Privilege Reset Tolerance"]
pub type Gpio115wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO116WrPrivilegeRstTolerance` reader - GPIO116 Write Privilege Reset Tolerance"]
pub type Gpio116wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO116WrPrivilegeRstTolerance` writer - GPIO116 Write Privilege Reset Tolerance"]
pub type Gpio116wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO117WrPrivilegeRstTolerance` reader - GPIO117 Write Privilege Reset Tolerance"]
pub type Gpio117wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO117WrPrivilegeRstTolerance` writer - GPIO117 Write Privilege Reset Tolerance"]
pub type Gpio117wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO118WrPrivilegeRstTolerance` reader - GPIO118 Write Privilege Reset Tolerance"]
pub type Gpio118wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO118WrPrivilegeRstTolerance` writer - GPIO118 Write Privilege Reset Tolerance"]
pub type Gpio118wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO119WrPrivilegeRstTolerance` reader - GPIO119 Write Privilege Reset Tolerance"]
pub type Gpio119wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO119WrPrivilegeRstTolerance` writer - GPIO119 Write Privilege Reset Tolerance"]
pub type Gpio119wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO120WrPrivilegeRstTolerance` reader - GPIO120 Write Privilege Reset Tolerance"]
pub type Gpio120wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO120WrPrivilegeRstTolerance` writer - GPIO120 Write Privilege Reset Tolerance"]
pub type Gpio120wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO121WrPrivilegeRstTolerance` reader - GPIO121 Write Privilege Reset Tolerance"]
pub type Gpio121wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO121WrPrivilegeRstTolerance` writer - GPIO121 Write Privilege Reset Tolerance"]
pub type Gpio121wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO122WrPrivilegeRstTolerance` reader - GPIO122 Write Privilege Reset Tolerance"]
pub type Gpio122wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO122WrPrivilegeRstTolerance` writer - GPIO122 Write Privilege Reset Tolerance"]
pub type Gpio122wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO123WrPrivilegeRstTolerance` reader - GPIO123 Write Privilege Reset Tolerance"]
pub type Gpio123wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO123WrPrivilegeRstTolerance` writer - GPIO123 Write Privilege Reset Tolerance"]
pub type Gpio123wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO124WrPrivilegeRstTolerance` reader - GPIO124 Write Privilege Reset Tolerance"]
pub type Gpio124wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO124WrPrivilegeRstTolerance` writer - GPIO124 Write Privilege Reset Tolerance"]
pub type Gpio124wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO125WrPrivilegeRstTolerance` reader - GPIO125 Write Privilege Reset Tolerance"]
pub type Gpio125wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO125WrPrivilegeRstTolerance` writer - GPIO125 Write Privilege Reset Tolerance"]
pub type Gpio125wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO126WrPrivilegeRstTolerance` reader - GPIO126 Write Privilege Reset Tolerance"]
pub type Gpio126wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO126WrPrivilegeRstTolerance` writer - GPIO126 Write Privilege Reset Tolerance"]
pub type Gpio126wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO127WrPrivilegeRstTolerance` reader - GPIO127 Write Privilege Reset Tolerance"]
pub type Gpio127wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO127WrPrivilegeRstTolerance` writer - GPIO127 Write Privilege Reset Tolerance"]
pub type Gpio127wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO096 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio096wr_privilege_rst_tolerance(&self) -> Gpio096wrPrivilegeRstToleranceR {
        Gpio096wrPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO097 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio097wr_privilege_rst_tolerance(&self) -> Gpio097wrPrivilegeRstToleranceR {
        Gpio097wrPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO098 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio098wr_privilege_rst_tolerance(&self) -> Gpio098wrPrivilegeRstToleranceR {
        Gpio098wrPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO099 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio099wr_privilege_rst_tolerance(&self) -> Gpio099wrPrivilegeRstToleranceR {
        Gpio099wrPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO100 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio100wr_privilege_rst_tolerance(&self) -> Gpio100wrPrivilegeRstToleranceR {
        Gpio100wrPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO101 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio101wr_privilege_rst_tolerance(&self) -> Gpio101wrPrivilegeRstToleranceR {
        Gpio101wrPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO102 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio102wr_privilege_rst_tolerance(&self) -> Gpio102wrPrivilegeRstToleranceR {
        Gpio102wrPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO103 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio103wr_privilege_rst_tolerance(&self) -> Gpio103wrPrivilegeRstToleranceR {
        Gpio103wrPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO104 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio104wr_privilege_rst_tolerance(&self) -> Gpio104wrPrivilegeRstToleranceR {
        Gpio104wrPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO105 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio105wr_privilege_rst_tolerance(&self) -> Gpio105wrPrivilegeRstToleranceR {
        Gpio105wrPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO106 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio106wr_privilege_rst_tolerance(&self) -> Gpio106wrPrivilegeRstToleranceR {
        Gpio106wrPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO107 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio107wr_privilege_rst_tolerance(&self) -> Gpio107wrPrivilegeRstToleranceR {
        Gpio107wrPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO108 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio108wr_privilege_rst_tolerance(&self) -> Gpio108wrPrivilegeRstToleranceR {
        Gpio108wrPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO109 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio109wr_privilege_rst_tolerance(&self) -> Gpio109wrPrivilegeRstToleranceR {
        Gpio109wrPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO110 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio110wr_privilege_rst_tolerance(&self) -> Gpio110wrPrivilegeRstToleranceR {
        Gpio110wrPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO111 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio111wr_privilege_rst_tolerance(&self) -> Gpio111wrPrivilegeRstToleranceR {
        Gpio111wrPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO112 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio112wr_privilege_rst_tolerance(&self) -> Gpio112wrPrivilegeRstToleranceR {
        Gpio112wrPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO113 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio113wr_privilege_rst_tolerance(&self) -> Gpio113wrPrivilegeRstToleranceR {
        Gpio113wrPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO114 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio114wr_privilege_rst_tolerance(&self) -> Gpio114wrPrivilegeRstToleranceR {
        Gpio114wrPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO115 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio115wr_privilege_rst_tolerance(&self) -> Gpio115wrPrivilegeRstToleranceR {
        Gpio115wrPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO116 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio116wr_privilege_rst_tolerance(&self) -> Gpio116wrPrivilegeRstToleranceR {
        Gpio116wrPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO117 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio117wr_privilege_rst_tolerance(&self) -> Gpio117wrPrivilegeRstToleranceR {
        Gpio117wrPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO118 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio118wr_privilege_rst_tolerance(&self) -> Gpio118wrPrivilegeRstToleranceR {
        Gpio118wrPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO119 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio119wr_privilege_rst_tolerance(&self) -> Gpio119wrPrivilegeRstToleranceR {
        Gpio119wrPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO120 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio120wr_privilege_rst_tolerance(&self) -> Gpio120wrPrivilegeRstToleranceR {
        Gpio120wrPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO121 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio121wr_privilege_rst_tolerance(&self) -> Gpio121wrPrivilegeRstToleranceR {
        Gpio121wrPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO122 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio122wr_privilege_rst_tolerance(&self) -> Gpio122wrPrivilegeRstToleranceR {
        Gpio122wrPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO123 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio123wr_privilege_rst_tolerance(&self) -> Gpio123wrPrivilegeRstToleranceR {
        Gpio123wrPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO124 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio124wr_privilege_rst_tolerance(&self) -> Gpio124wrPrivilegeRstToleranceR {
        Gpio124wrPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO125 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio125wr_privilege_rst_tolerance(&self) -> Gpio125wrPrivilegeRstToleranceR {
        Gpio125wrPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO126 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio126wr_privilege_rst_tolerance(&self) -> Gpio126wrPrivilegeRstToleranceR {
        Gpio126wrPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO127 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio127wr_privilege_rst_tolerance(&self) -> Gpio127wrPrivilegeRstToleranceR {
        Gpio127wrPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO096 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio096wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio096wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio096wrPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO097 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio097wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio097wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio097wrPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO098 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio098wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio098wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio098wrPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO099 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio099wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio099wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio099wrPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO100 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio100wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio100wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio100wrPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO101 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio101wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio101wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio101wrPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO102 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio102wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio102wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio102wrPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO103 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio103wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio103wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio103wrPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO104 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio104wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio104wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio104wrPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO105 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio105wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio105wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio105wrPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO106 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio106wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio106wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio106wrPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO107 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio107wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio107wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio107wrPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO108 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio108wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio108wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio108wrPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO109 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio109wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio109wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio109wrPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO110 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio110wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio110wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio110wrPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO111 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio111wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio111wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio111wrPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO112 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio112wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio112wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio112wrPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO113 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio113wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio113wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio113wrPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO114 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio114wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio114wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio114wrPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO115 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio115wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio115wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio115wrPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO116 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio116wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio116wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio116wrPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO117 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio117wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio117wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio117wrPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO118 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio118wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio118wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio118wrPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO119 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio119wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio119wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio119wrPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO120 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio120wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio120wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio120wrPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO121 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio121wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio121wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio121wrPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO122 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio122wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio122wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio122wrPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO123 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio123wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio123wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio123wrPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO124 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio124wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio124wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio124wrPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO125 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio125wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio125wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio125wrPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO126 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio126wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio126wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio126wrPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO127 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio127wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio127wrPrivilegeRstToleranceW<Gpiob1cSpec> {
        Gpio127wrPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Write Privilege Reset Tolerance Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpiob1cSpec;
impl crate::RegisterSpec for Gpiob1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob1c::R`](R) reader structure"]
impl crate::Readable for Gpiob1cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpiob1c::W`](W) writer structure"]
impl crate::Writable for Gpiob1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOB1C to value 0"]
impl crate::Resettable for Gpiob1cSpec {}
