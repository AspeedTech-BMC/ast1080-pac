#[doc = "Register `GPIOC14` reader"]
pub type R = crate::R<Gpioc14Spec>;
#[doc = "Register `GPIOC14` writer"]
pub type W = crate::W<Gpioc14Spec>;
#[doc = "GPIO032 Read Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio032readPrivilegeRstTolerance {
    #[doc = "0: Read Privilege of GPIO032 is reset by WDT."]
    ReadPrivilegeOfGpio032IsResetByWdt = 0,
    #[doc = "1: Read Privilege of GPIO032 is NOT reset by WDT."]
    ReadPrivilegeOfGpio032IsNotResetByWdt = 1,
}
impl From<Gpio032readPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio032readPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO032ReadPrivilegeRstTolerance` reader - GPIO032 Read Privilege Reset Tolerance"]
pub type Gpio032readPrivilegeRstToleranceR = crate::BitReader<Gpio032readPrivilegeRstTolerance>;
impl Gpio032readPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio032readPrivilegeRstTolerance {
        match self.bits {
            false => Gpio032readPrivilegeRstTolerance::ReadPrivilegeOfGpio032IsResetByWdt,
            true => Gpio032readPrivilegeRstTolerance::ReadPrivilegeOfGpio032IsNotResetByWdt,
        }
    }
    #[doc = "Read Privilege of GPIO032 is reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio032_is_reset_by_wdt(&self) -> bool {
        *self == Gpio032readPrivilegeRstTolerance::ReadPrivilegeOfGpio032IsResetByWdt
    }
    #[doc = "Read Privilege of GPIO032 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio032_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio032readPrivilegeRstTolerance::ReadPrivilegeOfGpio032IsNotResetByWdt
    }
}
#[doc = "Field `GPIO032ReadPrivilegeRstTolerance` writer - GPIO032 Read Privilege Reset Tolerance"]
pub type Gpio032readPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio032readPrivilegeRstTolerance>;
impl<'a, REG> Gpio032readPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Privilege of GPIO032 is reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio032_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio032readPrivilegeRstTolerance::ReadPrivilegeOfGpio032IsResetByWdt)
    }
    #[doc = "Read Privilege of GPIO032 is NOT reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio032_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio032readPrivilegeRstTolerance::ReadPrivilegeOfGpio032IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO033ReadPrivilegeRstTolerance` reader - GPIO033 Read Privilege Reset Tolerance"]
pub type Gpio033readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO033ReadPrivilegeRstTolerance` writer - GPIO033 Read Privilege Reset Tolerance"]
pub type Gpio033readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO034ReadPrivilegeRstTolerance` reader - GPIO034 Read Privilege Reset Tolerance"]
pub type Gpio034readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO034ReadPrivilegeRstTolerance` writer - GPIO034 Read Privilege Reset Tolerance"]
pub type Gpio034readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO035ReadPrivilegeRstTolerance` reader - GPIO035 Read Privilege Reset Tolerance"]
pub type Gpio035readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO035ReadPrivilegeRstTolerance` writer - GPIO035 Read Privilege Reset Tolerance"]
pub type Gpio035readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO036ReadPrivilegeRstTolerance` reader - GPIO036 Read Privilege Reset Tolerance"]
pub type Gpio036readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO036ReadPrivilegeRstTolerance` writer - GPIO036 Read Privilege Reset Tolerance"]
pub type Gpio036readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO037ReadPrivilegeRstTolerance` reader - GPIO037 Read Privilege Reset Tolerance"]
pub type Gpio037readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO037ReadPrivilegeRstTolerance` writer - GPIO037 Read Privilege Reset Tolerance"]
pub type Gpio037readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO038ReadPrivilegeRstTolerance` reader - GPIO038 Read Privilege Reset Tolerance"]
pub type Gpio038readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO038ReadPrivilegeRstTolerance` writer - GPIO038 Read Privilege Reset Tolerance"]
pub type Gpio038readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO039ReadPrivilegeRstTolerance` reader - GPIO039 Read Privilege Reset Tolerance"]
pub type Gpio039readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO039ReadPrivilegeRstTolerance` writer - GPIO039 Read Privilege Reset Tolerance"]
pub type Gpio039readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO040ReadPrivilegeRstTolerance` reader - GPIO040 Read Privilege Reset Tolerance"]
pub type Gpio040readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO040ReadPrivilegeRstTolerance` writer - GPIO040 Read Privilege Reset Tolerance"]
pub type Gpio040readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO041ReadPrivilegeRstTolerance` reader - GPIO041 Read Privilege Reset Tolerance"]
pub type Gpio041readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO041ReadPrivilegeRstTolerance` writer - GPIO041 Read Privilege Reset Tolerance"]
pub type Gpio041readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO042ReadPrivilegeRstTolerance` reader - GPIO042 Read Privilege Reset Tolerance"]
pub type Gpio042readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO042ReadPrivilegeRstTolerance` writer - GPIO042 Read Privilege Reset Tolerance"]
pub type Gpio042readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO043ReadPrivilegeRstTolerance` reader - GPIO043 Read Privilege Reset Tolerance"]
pub type Gpio043readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO043ReadPrivilegeRstTolerance` writer - GPIO043 Read Privilege Reset Tolerance"]
pub type Gpio043readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO044ReadPrivilegeRstTolerance` reader - GPIO044 Read Privilege Reset Tolerance"]
pub type Gpio044readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO044ReadPrivilegeRstTolerance` writer - GPIO044 Read Privilege Reset Tolerance"]
pub type Gpio044readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO045ReadPrivilegeRstTolerance` reader - GPIO045 Read Privilege Reset Tolerance"]
pub type Gpio045readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO045ReadPrivilegeRstTolerance` writer - GPIO045 Read Privilege Reset Tolerance"]
pub type Gpio045readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO046ReadPrivilegeRstTolerance` reader - GPIO046 Read Privilege Reset Tolerance"]
pub type Gpio046readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO046ReadPrivilegeRstTolerance` writer - GPIO046 Read Privilege Reset Tolerance"]
pub type Gpio046readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO047ReadPrivilegeRstTolerance` reader - GPIO047 Read Privilege Reset Tolerance"]
pub type Gpio047readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO047ReadPrivilegeRstTolerance` writer - GPIO047 Read Privilege Reset Tolerance"]
pub type Gpio047readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO048ReadPrivilegeRstTolerance` reader - GPIO048 Read Privilege Reset Tolerance"]
pub type Gpio048readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO048ReadPrivilegeRstTolerance` writer - GPIO048 Read Privilege Reset Tolerance"]
pub type Gpio048readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO049ReadPrivilegeRstTolerance` reader - GPIO049 Read Privilege Reset Tolerance"]
pub type Gpio049readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO049ReadPrivilegeRstTolerance` writer - GPIO049 Read Privilege Reset Tolerance"]
pub type Gpio049readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO050ReadPrivilegeRstTolerance` reader - GPIO050 Read Privilege Reset Tolerance"]
pub type Gpio050readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO050ReadPrivilegeRstTolerance` writer - GPIO050 Read Privilege Reset Tolerance"]
pub type Gpio050readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO051ReadPrivilegeRstTolerance` reader - GPIO051 Read Privilege Reset Tolerance"]
pub type Gpio051readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO051ReadPrivilegeRstTolerance` writer - GPIO051 Read Privilege Reset Tolerance"]
pub type Gpio051readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO052ReadPrivilegeRstTolerance` reader - GPIO052 Read Privilege Reset Tolerance"]
pub type Gpio052readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO052ReadPrivilegeRstTolerance` writer - GPIO052 Read Privilege Reset Tolerance"]
pub type Gpio052readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO053ReadPrivilegeRstTolerance` reader - GPIO053 Read Privilege Reset Tolerance"]
pub type Gpio053readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO053ReadPrivilegeRstTolerance` writer - GPIO053 Read Privilege Reset Tolerance"]
pub type Gpio053readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO054ReadPrivilegeRstTolerance` reader - GPIO054 Read Privilege Reset Tolerance"]
pub type Gpio054readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO054ReadPrivilegeRstTolerance` writer - GPIO054 Read Privilege Reset Tolerance"]
pub type Gpio054readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO055ReadPrivilegeRstTolerance` reader - GPIO055 Read Privilege Reset Tolerance"]
pub type Gpio055readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO055ReadPrivilegeRstTolerance` writer - GPIO055 Read Privilege Reset Tolerance"]
pub type Gpio055readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO056ReadPrivilegeRstTolerance` reader - GPIO056 Read Privilege Reset Tolerance"]
pub type Gpio056readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO056ReadPrivilegeRstTolerance` writer - GPIO056 Read Privilege Reset Tolerance"]
pub type Gpio056readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO057ReadPrivilegeRstTolerance` reader - GPIO057 Read Privilege Reset Tolerance"]
pub type Gpio057readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO057ReadPrivilegeRstTolerance` writer - GPIO057 Read Privilege Reset Tolerance"]
pub type Gpio057readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO058ReadPrivilegeRstTolerance` reader - GPIO058 Read Privilege Reset Tolerance"]
pub type Gpio058readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO058ReadPrivilegeRstTolerance` writer - GPIO058 Read Privilege Reset Tolerance"]
pub type Gpio058readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO059ReadPrivilegeRstTolerance` reader - GPIO059 Read Privilege Reset Tolerance"]
pub type Gpio059readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO059ReadPrivilegeRstTolerance` writer - GPIO059 Read Privilege Reset Tolerance"]
pub type Gpio059readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO060ReadPrivilegeRstTolerance` reader - GPIO060 Read Privilege Reset Tolerance"]
pub type Gpio060readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO060ReadPrivilegeRstTolerance` writer - GPIO060 Read Privilege Reset Tolerance"]
pub type Gpio060readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO061ReadPrivilegeRstTolerance` reader - GPIO061 Read Privilege Reset Tolerance"]
pub type Gpio061readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO061ReadPrivilegeRstTolerance` writer - GPIO061 Read Privilege Reset Tolerance"]
pub type Gpio061readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO062ReadPrivilegeRstTolerance` reader - GPIO062 Read Privilege Reset Tolerance"]
pub type Gpio062readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO062ReadPrivilegeRstTolerance` writer - GPIO062 Read Privilege Reset Tolerance"]
pub type Gpio062readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO063ReadPrivilegeRstTolerance` reader - GPIO063 Read Privilege Reset Tolerance"]
pub type Gpio063readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO063ReadPrivilegeRstTolerance` writer - GPIO063 Read Privilege Reset Tolerance"]
pub type Gpio063readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO032 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio032read_privilege_rst_tolerance(&self) -> Gpio032readPrivilegeRstToleranceR {
        Gpio032readPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO033 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio033read_privilege_rst_tolerance(&self) -> Gpio033readPrivilegeRstToleranceR {
        Gpio033readPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO034 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio034read_privilege_rst_tolerance(&self) -> Gpio034readPrivilegeRstToleranceR {
        Gpio034readPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO035 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio035read_privilege_rst_tolerance(&self) -> Gpio035readPrivilegeRstToleranceR {
        Gpio035readPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO036 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio036read_privilege_rst_tolerance(&self) -> Gpio036readPrivilegeRstToleranceR {
        Gpio036readPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO037 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio037read_privilege_rst_tolerance(&self) -> Gpio037readPrivilegeRstToleranceR {
        Gpio037readPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO038 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio038read_privilege_rst_tolerance(&self) -> Gpio038readPrivilegeRstToleranceR {
        Gpio038readPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO039 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio039read_privilege_rst_tolerance(&self) -> Gpio039readPrivilegeRstToleranceR {
        Gpio039readPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO040 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio040read_privilege_rst_tolerance(&self) -> Gpio040readPrivilegeRstToleranceR {
        Gpio040readPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO041 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio041read_privilege_rst_tolerance(&self) -> Gpio041readPrivilegeRstToleranceR {
        Gpio041readPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO042 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio042read_privilege_rst_tolerance(&self) -> Gpio042readPrivilegeRstToleranceR {
        Gpio042readPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO043 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio043read_privilege_rst_tolerance(&self) -> Gpio043readPrivilegeRstToleranceR {
        Gpio043readPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO044 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio044read_privilege_rst_tolerance(&self) -> Gpio044readPrivilegeRstToleranceR {
        Gpio044readPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO045 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio045read_privilege_rst_tolerance(&self) -> Gpio045readPrivilegeRstToleranceR {
        Gpio045readPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO046 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio046read_privilege_rst_tolerance(&self) -> Gpio046readPrivilegeRstToleranceR {
        Gpio046readPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO047 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio047read_privilege_rst_tolerance(&self) -> Gpio047readPrivilegeRstToleranceR {
        Gpio047readPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO048 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio048read_privilege_rst_tolerance(&self) -> Gpio048readPrivilegeRstToleranceR {
        Gpio048readPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO049 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio049read_privilege_rst_tolerance(&self) -> Gpio049readPrivilegeRstToleranceR {
        Gpio049readPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO050 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio050read_privilege_rst_tolerance(&self) -> Gpio050readPrivilegeRstToleranceR {
        Gpio050readPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO051 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio051read_privilege_rst_tolerance(&self) -> Gpio051readPrivilegeRstToleranceR {
        Gpio051readPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO052 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio052read_privilege_rst_tolerance(&self) -> Gpio052readPrivilegeRstToleranceR {
        Gpio052readPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO053 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio053read_privilege_rst_tolerance(&self) -> Gpio053readPrivilegeRstToleranceR {
        Gpio053readPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO054 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio054read_privilege_rst_tolerance(&self) -> Gpio054readPrivilegeRstToleranceR {
        Gpio054readPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO055 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio055read_privilege_rst_tolerance(&self) -> Gpio055readPrivilegeRstToleranceR {
        Gpio055readPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO056 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio056read_privilege_rst_tolerance(&self) -> Gpio056readPrivilegeRstToleranceR {
        Gpio056readPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO057 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio057read_privilege_rst_tolerance(&self) -> Gpio057readPrivilegeRstToleranceR {
        Gpio057readPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO058 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio058read_privilege_rst_tolerance(&self) -> Gpio058readPrivilegeRstToleranceR {
        Gpio058readPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO059 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio059read_privilege_rst_tolerance(&self) -> Gpio059readPrivilegeRstToleranceR {
        Gpio059readPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO060 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio060read_privilege_rst_tolerance(&self) -> Gpio060readPrivilegeRstToleranceR {
        Gpio060readPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO061 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio061read_privilege_rst_tolerance(&self) -> Gpio061readPrivilegeRstToleranceR {
        Gpio061readPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO062 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio062read_privilege_rst_tolerance(&self) -> Gpio062readPrivilegeRstToleranceR {
        Gpio062readPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO063 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio063read_privilege_rst_tolerance(&self) -> Gpio063readPrivilegeRstToleranceR {
        Gpio063readPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO032 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio032read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio032readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio032readPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO033 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio033read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio033readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio033readPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO034 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio034read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio034readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio034readPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO035 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio035read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio035readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio035readPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO036 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio036read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio036readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio036readPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO037 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio037read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio037readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio037readPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO038 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio038read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio038readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio038readPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO039 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio039read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio039readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio039readPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO040 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio040read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio040readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio040readPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO041 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio041read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio041readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio041readPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO042 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio042read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio042readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio042readPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO043 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio043read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio043readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio043readPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO044 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio044read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio044readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio044readPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO045 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio045read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio045readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio045readPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO046 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio046read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio046readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio046readPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO047 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio047read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio047readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio047readPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO048 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio048read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio048readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio048readPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO049 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio049read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio049readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio049readPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO050 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio050read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio050readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio050readPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO051 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio051read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio051readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio051readPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO052 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio052read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio052readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio052readPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO053 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio053read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio053readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio053readPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO054 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio054read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio054readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio054readPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO055 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio055read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio055readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio055readPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO056 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio056read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio056readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio056readPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO057 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio057read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio057readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio057readPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO058 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio058read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio058readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio058readPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO059 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio059read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio059readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio059readPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO060 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio060read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio060readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio060readPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO061 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio061read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio061readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio061readPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO062 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio062read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio062readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio062readPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO063 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio063read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio063readPrivilegeRstToleranceW<Gpioc14Spec> {
        Gpio063readPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Read Privilege Reset Tolerance Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioc14Spec;
impl crate::RegisterSpec for Gpioc14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc14::R`](R) reader structure"]
impl crate::Readable for Gpioc14Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioc14::W`](W) writer structure"]
impl crate::Writable for Gpioc14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC14 to value 0"]
impl crate::Resettable for Gpioc14Spec {}
