#[doc = "Register `GPIOB14` reader"]
pub type R = crate::R<Gpiob14Spec>;
#[doc = "Register `GPIOB14` writer"]
pub type W = crate::W<Gpiob14Spec>;
#[doc = "GPIO032 Write Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio032wrPrivilegeRstTolerance {
    #[doc = "0: Write Privilege of GPIO032 is reset by WDT."]
    WritePrivilegeOfGpio032IsResetByWdt = 0,
    #[doc = "1: Write Privilege of GPIO032 is NOT reset by WDT."]
    WritePrivilegeOfGpio032IsNotResetByWdt = 1,
}
impl From<Gpio032wrPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio032wrPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO032WrPrivilegeRstTolerance` reader - GPIO032 Write Privilege Reset Tolerance"]
pub type Gpio032wrPrivilegeRstToleranceR = crate::BitReader<Gpio032wrPrivilegeRstTolerance>;
impl Gpio032wrPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio032wrPrivilegeRstTolerance {
        match self.bits {
            false => Gpio032wrPrivilegeRstTolerance::WritePrivilegeOfGpio032IsResetByWdt,
            true => Gpio032wrPrivilegeRstTolerance::WritePrivilegeOfGpio032IsNotResetByWdt,
        }
    }
    #[doc = "Write Privilege of GPIO032 is reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio032_is_reset_by_wdt(&self) -> bool {
        *self == Gpio032wrPrivilegeRstTolerance::WritePrivilegeOfGpio032IsResetByWdt
    }
    #[doc = "Write Privilege of GPIO032 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio032_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio032wrPrivilegeRstTolerance::WritePrivilegeOfGpio032IsNotResetByWdt
    }
}
#[doc = "Field `GPIO032WrPrivilegeRstTolerance` writer - GPIO032 Write Privilege Reset Tolerance"]
pub type Gpio032wrPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio032wrPrivilegeRstTolerance>;
impl<'a, REG> Gpio032wrPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write Privilege of GPIO032 is reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio032_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio032wrPrivilegeRstTolerance::WritePrivilegeOfGpio032IsResetByWdt)
    }
    #[doc = "Write Privilege of GPIO032 is NOT reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio032_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio032wrPrivilegeRstTolerance::WritePrivilegeOfGpio032IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO033WrPrivilegeRstTolerance` reader - GPIO033 Write Privilege Reset Tolerance"]
pub type Gpio033wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO033WrPrivilegeRstTolerance` writer - GPIO033 Write Privilege Reset Tolerance"]
pub type Gpio033wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO034WrPrivilegeRstTolerance` reader - GPIO034 Write Privilege Reset Tolerance"]
pub type Gpio034wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO034WrPrivilegeRstTolerance` writer - GPIO034 Write Privilege Reset Tolerance"]
pub type Gpio034wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO035WrPrivilegeRstTolerance` reader - GPIO035 Write Privilege Reset Tolerance"]
pub type Gpio035wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO035WrPrivilegeRstTolerance` writer - GPIO035 Write Privilege Reset Tolerance"]
pub type Gpio035wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO036WrPrivilegeRstTolerance` reader - GPIO036 Write Privilege Reset Tolerance"]
pub type Gpio036wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO036WrPrivilegeRstTolerance` writer - GPIO036 Write Privilege Reset Tolerance"]
pub type Gpio036wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO037WrPrivilegeRstTolerance` reader - GPIO037 Write Privilege Reset Tolerance"]
pub type Gpio037wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO037WrPrivilegeRstTolerance` writer - GPIO037 Write Privilege Reset Tolerance"]
pub type Gpio037wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO038WrPrivilegeRstTolerance` reader - GPIO038 Write Privilege Reset Tolerance"]
pub type Gpio038wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO038WrPrivilegeRstTolerance` writer - GPIO038 Write Privilege Reset Tolerance"]
pub type Gpio038wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO039WrPrivilegeRstTolerance` reader - GPIO039 Write Privilege Reset Tolerance"]
pub type Gpio039wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO039WrPrivilegeRstTolerance` writer - GPIO039 Write Privilege Reset Tolerance"]
pub type Gpio039wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO040WrPrivilegeRstTolerance` reader - GPIO040 Write Privilege Reset Tolerance"]
pub type Gpio040wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO040WrPrivilegeRstTolerance` writer - GPIO040 Write Privilege Reset Tolerance"]
pub type Gpio040wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO041WrPrivilegeRstTolerance` reader - GPIO041 Write Privilege Reset Tolerance"]
pub type Gpio041wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO041WrPrivilegeRstTolerance` writer - GPIO041 Write Privilege Reset Tolerance"]
pub type Gpio041wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO042WrPrivilegeRstTolerance` reader - GPIO042 Write Privilege Reset Tolerance"]
pub type Gpio042wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO042WrPrivilegeRstTolerance` writer - GPIO042 Write Privilege Reset Tolerance"]
pub type Gpio042wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO043WrPrivilegeRstTolerance` reader - GPIO043 Write Privilege Reset Tolerance"]
pub type Gpio043wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO043WrPrivilegeRstTolerance` writer - GPIO043 Write Privilege Reset Tolerance"]
pub type Gpio043wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO044WrPrivilegeRstTolerance` reader - GPIO044 Write Privilege Reset Tolerance"]
pub type Gpio044wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO044WrPrivilegeRstTolerance` writer - GPIO044 Write Privilege Reset Tolerance"]
pub type Gpio044wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO045WrPrivilegeRstTolerance` reader - GPIO045 Write Privilege Reset Tolerance"]
pub type Gpio045wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO045WrPrivilegeRstTolerance` writer - GPIO045 Write Privilege Reset Tolerance"]
pub type Gpio045wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO046WrPrivilegeRstTolerance` reader - GPIO046 Write Privilege Reset Tolerance"]
pub type Gpio046wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO046WrPrivilegeRstTolerance` writer - GPIO046 Write Privilege Reset Tolerance"]
pub type Gpio046wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO047WrPrivilegeRstTolerance` reader - GPIO047 Write Privilege Reset Tolerance"]
pub type Gpio047wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO047WrPrivilegeRstTolerance` writer - GPIO047 Write Privilege Reset Tolerance"]
pub type Gpio047wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO048WrPrivilegeRstTolerance` reader - GPIO048 Write Privilege Reset Tolerance"]
pub type Gpio048wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO048WrPrivilegeRstTolerance` writer - GPIO048 Write Privilege Reset Tolerance"]
pub type Gpio048wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO049WrPrivilegeRstTolerance` reader - GPIO049 Write Privilege Reset Tolerance"]
pub type Gpio049wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO049WrPrivilegeRstTolerance` writer - GPIO049 Write Privilege Reset Tolerance"]
pub type Gpio049wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO050WrPrivilegeRstTolerance` reader - GPIO050 Write Privilege Reset Tolerance"]
pub type Gpio050wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO050WrPrivilegeRstTolerance` writer - GPIO050 Write Privilege Reset Tolerance"]
pub type Gpio050wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO051WrPrivilegeRstTolerance` reader - GPIO051 Write Privilege Reset Tolerance"]
pub type Gpio051wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO051WrPrivilegeRstTolerance` writer - GPIO051 Write Privilege Reset Tolerance"]
pub type Gpio051wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO052WrPrivilegeRstTolerance` reader - GPIO052 Write Privilege Reset Tolerance"]
pub type Gpio052wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO052WrPrivilegeRstTolerance` writer - GPIO052 Write Privilege Reset Tolerance"]
pub type Gpio052wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO053WrPrivilegeRstTolerance` reader - GPIO053 Write Privilege Reset Tolerance"]
pub type Gpio053wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO053WrPrivilegeRstTolerance` writer - GPIO053 Write Privilege Reset Tolerance"]
pub type Gpio053wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO054WrPrivilegeRstTolerance` reader - GPIO054 Write Privilege Reset Tolerance"]
pub type Gpio054wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO054WrPrivilegeRstTolerance` writer - GPIO054 Write Privilege Reset Tolerance"]
pub type Gpio054wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO055WrPrivilegeRstTolerance` reader - GPIO055 Write Privilege Reset Tolerance"]
pub type Gpio055wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO055WrPrivilegeRstTolerance` writer - GPIO055 Write Privilege Reset Tolerance"]
pub type Gpio055wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO056WrPrivilegeRstTolerance` reader - GPIO056 Write Privilege Reset Tolerance"]
pub type Gpio056wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO056WrPrivilegeRstTolerance` writer - GPIO056 Write Privilege Reset Tolerance"]
pub type Gpio056wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO057WrPrivilegeRstTolerance` reader - GPIO057 Write Privilege Reset Tolerance"]
pub type Gpio057wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO057WrPrivilegeRstTolerance` writer - GPIO057 Write Privilege Reset Tolerance"]
pub type Gpio057wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO058WrPrivilegeRstTolerance` reader - GPIO058 Write Privilege Reset Tolerance"]
pub type Gpio058wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO058WrPrivilegeRstTolerance` writer - GPIO058 Write Privilege Reset Tolerance"]
pub type Gpio058wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO059WrPrivilegeRstTolerance` reader - GPIO059 Write Privilege Reset Tolerance"]
pub type Gpio059wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO059WrPrivilegeRstTolerance` writer - GPIO059 Write Privilege Reset Tolerance"]
pub type Gpio059wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO060WrPrivilegeRstTolerance` reader - GPIO060 Write Privilege Reset Tolerance"]
pub type Gpio060wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO060WrPrivilegeRstTolerance` writer - GPIO060 Write Privilege Reset Tolerance"]
pub type Gpio060wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO061WrPrivilegeRstTolerance` reader - GPIO061 Write Privilege Reset Tolerance"]
pub type Gpio061wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO061WrPrivilegeRstTolerance` writer - GPIO061 Write Privilege Reset Tolerance"]
pub type Gpio061wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO062WrPrivilegeRstTolerance` reader - GPIO062 Write Privilege Reset Tolerance"]
pub type Gpio062wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO062WrPrivilegeRstTolerance` writer - GPIO062 Write Privilege Reset Tolerance"]
pub type Gpio062wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO063WrPrivilegeRstTolerance` reader - GPIO063 Write Privilege Reset Tolerance"]
pub type Gpio063wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO063WrPrivilegeRstTolerance` writer - GPIO063 Write Privilege Reset Tolerance"]
pub type Gpio063wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO032 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio032wr_privilege_rst_tolerance(&self) -> Gpio032wrPrivilegeRstToleranceR {
        Gpio032wrPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO033 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio033wr_privilege_rst_tolerance(&self) -> Gpio033wrPrivilegeRstToleranceR {
        Gpio033wrPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO034 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio034wr_privilege_rst_tolerance(&self) -> Gpio034wrPrivilegeRstToleranceR {
        Gpio034wrPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO035 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio035wr_privilege_rst_tolerance(&self) -> Gpio035wrPrivilegeRstToleranceR {
        Gpio035wrPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO036 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio036wr_privilege_rst_tolerance(&self) -> Gpio036wrPrivilegeRstToleranceR {
        Gpio036wrPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO037 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio037wr_privilege_rst_tolerance(&self) -> Gpio037wrPrivilegeRstToleranceR {
        Gpio037wrPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO038 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio038wr_privilege_rst_tolerance(&self) -> Gpio038wrPrivilegeRstToleranceR {
        Gpio038wrPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO039 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio039wr_privilege_rst_tolerance(&self) -> Gpio039wrPrivilegeRstToleranceR {
        Gpio039wrPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO040 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio040wr_privilege_rst_tolerance(&self) -> Gpio040wrPrivilegeRstToleranceR {
        Gpio040wrPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO041 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio041wr_privilege_rst_tolerance(&self) -> Gpio041wrPrivilegeRstToleranceR {
        Gpio041wrPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO042 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio042wr_privilege_rst_tolerance(&self) -> Gpio042wrPrivilegeRstToleranceR {
        Gpio042wrPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO043 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio043wr_privilege_rst_tolerance(&self) -> Gpio043wrPrivilegeRstToleranceR {
        Gpio043wrPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO044 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio044wr_privilege_rst_tolerance(&self) -> Gpio044wrPrivilegeRstToleranceR {
        Gpio044wrPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO045 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio045wr_privilege_rst_tolerance(&self) -> Gpio045wrPrivilegeRstToleranceR {
        Gpio045wrPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO046 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio046wr_privilege_rst_tolerance(&self) -> Gpio046wrPrivilegeRstToleranceR {
        Gpio046wrPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO047 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio047wr_privilege_rst_tolerance(&self) -> Gpio047wrPrivilegeRstToleranceR {
        Gpio047wrPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO048 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio048wr_privilege_rst_tolerance(&self) -> Gpio048wrPrivilegeRstToleranceR {
        Gpio048wrPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO049 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio049wr_privilege_rst_tolerance(&self) -> Gpio049wrPrivilegeRstToleranceR {
        Gpio049wrPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO050 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio050wr_privilege_rst_tolerance(&self) -> Gpio050wrPrivilegeRstToleranceR {
        Gpio050wrPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO051 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio051wr_privilege_rst_tolerance(&self) -> Gpio051wrPrivilegeRstToleranceR {
        Gpio051wrPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO052 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio052wr_privilege_rst_tolerance(&self) -> Gpio052wrPrivilegeRstToleranceR {
        Gpio052wrPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO053 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio053wr_privilege_rst_tolerance(&self) -> Gpio053wrPrivilegeRstToleranceR {
        Gpio053wrPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO054 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio054wr_privilege_rst_tolerance(&self) -> Gpio054wrPrivilegeRstToleranceR {
        Gpio054wrPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO055 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio055wr_privilege_rst_tolerance(&self) -> Gpio055wrPrivilegeRstToleranceR {
        Gpio055wrPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO056 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio056wr_privilege_rst_tolerance(&self) -> Gpio056wrPrivilegeRstToleranceR {
        Gpio056wrPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO057 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio057wr_privilege_rst_tolerance(&self) -> Gpio057wrPrivilegeRstToleranceR {
        Gpio057wrPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO058 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio058wr_privilege_rst_tolerance(&self) -> Gpio058wrPrivilegeRstToleranceR {
        Gpio058wrPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO059 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio059wr_privilege_rst_tolerance(&self) -> Gpio059wrPrivilegeRstToleranceR {
        Gpio059wrPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO060 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio060wr_privilege_rst_tolerance(&self) -> Gpio060wrPrivilegeRstToleranceR {
        Gpio060wrPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO061 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio061wr_privilege_rst_tolerance(&self) -> Gpio061wrPrivilegeRstToleranceR {
        Gpio061wrPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO062 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio062wr_privilege_rst_tolerance(&self) -> Gpio062wrPrivilegeRstToleranceR {
        Gpio062wrPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO063 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio063wr_privilege_rst_tolerance(&self) -> Gpio063wrPrivilegeRstToleranceR {
        Gpio063wrPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO032 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio032wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio032wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio032wrPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO033 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio033wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio033wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio033wrPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO034 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio034wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio034wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio034wrPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO035 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio035wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio035wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio035wrPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO036 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio036wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio036wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio036wrPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO037 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio037wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio037wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio037wrPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO038 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio038wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio038wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio038wrPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO039 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio039wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio039wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio039wrPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO040 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio040wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio040wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio040wrPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO041 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio041wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio041wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio041wrPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO042 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio042wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio042wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio042wrPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO043 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio043wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio043wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio043wrPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO044 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio044wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio044wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio044wrPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO045 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio045wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio045wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio045wrPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO046 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio046wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio046wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio046wrPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO047 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio047wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio047wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio047wrPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO048 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio048wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio048wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio048wrPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO049 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio049wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio049wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio049wrPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO050 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio050wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio050wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio050wrPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO051 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio051wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio051wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio051wrPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO052 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio052wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio052wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio052wrPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO053 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio053wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio053wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio053wrPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO054 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio054wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio054wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio054wrPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO055 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio055wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio055wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio055wrPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO056 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio056wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio056wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio056wrPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO057 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio057wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio057wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio057wrPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO058 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio058wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio058wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio058wrPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO059 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio059wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio059wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio059wrPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO060 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio060wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio060wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio060wrPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO061 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio061wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio061wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio061wrPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO062 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio062wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio062wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio062wrPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO063 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio063wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio063wrPrivilegeRstToleranceW<Gpiob14Spec> {
        Gpio063wrPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Write Privilege Reset Tolerance Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpiob14Spec;
impl crate::RegisterSpec for Gpiob14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob14::R`](R) reader structure"]
impl crate::Readable for Gpiob14Spec {}
#[doc = "`write(|w| ..)` method takes [`gpiob14::W`](W) writer structure"]
impl crate::Writable for Gpiob14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOB14 to value 0"]
impl crate::Resettable for Gpiob14Spec {}
