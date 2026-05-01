#[doc = "Register `GPIOC18` reader"]
pub type R = crate::R<Gpioc18Spec>;
#[doc = "Register `GPIOC18` writer"]
pub type W = crate::W<Gpioc18Spec>;
#[doc = "GPIO064 Read Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio064readPrivilegeRstTolerance {
    #[doc = "0: Read Privilege of GPIO064 is reset by WDT."]
    ReadPrivilegeOfGpio064IsResetByWdt = 0,
    #[doc = "1: Read Privilege of GPIO064 is NOT reset by WDT."]
    ReadPrivilegeOfGpio064IsNotResetByWdt = 1,
}
impl From<Gpio064readPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio064readPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO064ReadPrivilegeRstTolerance` reader - GPIO064 Read Privilege Reset Tolerance"]
pub type Gpio064readPrivilegeRstToleranceR = crate::BitReader<Gpio064readPrivilegeRstTolerance>;
impl Gpio064readPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio064readPrivilegeRstTolerance {
        match self.bits {
            false => Gpio064readPrivilegeRstTolerance::ReadPrivilegeOfGpio064IsResetByWdt,
            true => Gpio064readPrivilegeRstTolerance::ReadPrivilegeOfGpio064IsNotResetByWdt,
        }
    }
    #[doc = "Read Privilege of GPIO064 is reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio064_is_reset_by_wdt(&self) -> bool {
        *self == Gpio064readPrivilegeRstTolerance::ReadPrivilegeOfGpio064IsResetByWdt
    }
    #[doc = "Read Privilege of GPIO064 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio064_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio064readPrivilegeRstTolerance::ReadPrivilegeOfGpio064IsNotResetByWdt
    }
}
#[doc = "Field `GPIO064ReadPrivilegeRstTolerance` writer - GPIO064 Read Privilege Reset Tolerance"]
pub type Gpio064readPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio064readPrivilegeRstTolerance>;
impl<'a, REG> Gpio064readPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Privilege of GPIO064 is reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio064_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio064readPrivilegeRstTolerance::ReadPrivilegeOfGpio064IsResetByWdt)
    }
    #[doc = "Read Privilege of GPIO064 is NOT reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio064_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio064readPrivilegeRstTolerance::ReadPrivilegeOfGpio064IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO065ReadPrivilegeRstTolerance` reader - GPIO065 Read Privilege Reset Tolerance"]
pub type Gpio065readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO065ReadPrivilegeRstTolerance` writer - GPIO065 Read Privilege Reset Tolerance"]
pub type Gpio065readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO066ReadPrivilegeRstTolerance` reader - GPIO066 Read Privilege Reset Tolerance"]
pub type Gpio066readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO066ReadPrivilegeRstTolerance` writer - GPIO066 Read Privilege Reset Tolerance"]
pub type Gpio066readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO067ReadPrivilegeRstTolerance` reader - GPIO067 Read Privilege Reset Tolerance"]
pub type Gpio067readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO067ReadPrivilegeRstTolerance` writer - GPIO067 Read Privilege Reset Tolerance"]
pub type Gpio067readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO068ReadPrivilegeRstTolerance` reader - GPIO068 Read Privilege Reset Tolerance"]
pub type Gpio068readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO068ReadPrivilegeRstTolerance` writer - GPIO068 Read Privilege Reset Tolerance"]
pub type Gpio068readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO069ReadPrivilegeRstTolerance` reader - GPIO069 Read Privilege Reset Tolerance"]
pub type Gpio069readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO069ReadPrivilegeRstTolerance` writer - GPIO069 Read Privilege Reset Tolerance"]
pub type Gpio069readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO070ReadPrivilegeRstTolerance` reader - GPIO070 Read Privilege Reset Tolerance"]
pub type Gpio070readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO070ReadPrivilegeRstTolerance` writer - GPIO070 Read Privilege Reset Tolerance"]
pub type Gpio070readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO071ReadPrivilegeRstTolerance` reader - GPIO071 Read Privilege Reset Tolerance"]
pub type Gpio071readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO071ReadPrivilegeRstTolerance` writer - GPIO071 Read Privilege Reset Tolerance"]
pub type Gpio071readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO072ReadPrivilegeRstTolerance` reader - GPIO072 Read Privilege Reset Tolerance"]
pub type Gpio072readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO072ReadPrivilegeRstTolerance` writer - GPIO072 Read Privilege Reset Tolerance"]
pub type Gpio072readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO073ReadPrivilegeRstTolerance` reader - GPIO073 Read Privilege Reset Tolerance"]
pub type Gpio073readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO073ReadPrivilegeRstTolerance` writer - GPIO073 Read Privilege Reset Tolerance"]
pub type Gpio073readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO074ReadPrivilegeRstTolerance` reader - GPIO074 Read Privilege Reset Tolerance"]
pub type Gpio074readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO074ReadPrivilegeRstTolerance` writer - GPIO074 Read Privilege Reset Tolerance"]
pub type Gpio074readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO075ReadPrivilegeRstTolerance` reader - GPIO075 Read Privilege Reset Tolerance"]
pub type Gpio075readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO075ReadPrivilegeRstTolerance` writer - GPIO075 Read Privilege Reset Tolerance"]
pub type Gpio075readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO076ReadPrivilegeRstTolerance` reader - GPIO076 Read Privilege Reset Tolerance"]
pub type Gpio076readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO076ReadPrivilegeRstTolerance` writer - GPIO076 Read Privilege Reset Tolerance"]
pub type Gpio076readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO077ReadPrivilegeRstTolerance` reader - GPIO077 Read Privilege Reset Tolerance"]
pub type Gpio077readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO077ReadPrivilegeRstTolerance` writer - GPIO077 Read Privilege Reset Tolerance"]
pub type Gpio077readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO078ReadPrivilegeRstTolerance` reader - GPIO078 Read Privilege Reset Tolerance"]
pub type Gpio078readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO078ReadPrivilegeRstTolerance` writer - GPIO078 Read Privilege Reset Tolerance"]
pub type Gpio078readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO079ReadPrivilegeRstTolerance` reader - GPIO079 Read Privilege Reset Tolerance"]
pub type Gpio079readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO079ReadPrivilegeRstTolerance` writer - GPIO079 Read Privilege Reset Tolerance"]
pub type Gpio079readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO080ReadPrivilegeRstTolerance` reader - GPIO080 Read Privilege Reset Tolerance"]
pub type Gpio080readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO080ReadPrivilegeRstTolerance` writer - GPIO080 Read Privilege Reset Tolerance"]
pub type Gpio080readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO081ReadPrivilegeRstTolerance` reader - GPIO081 Read Privilege Reset Tolerance"]
pub type Gpio081readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO081ReadPrivilegeRstTolerance` writer - GPIO081 Read Privilege Reset Tolerance"]
pub type Gpio081readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO082ReadPrivilegeRstTolerance` reader - GPIO082 Read Privilege Reset Tolerance"]
pub type Gpio082readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO082ReadPrivilegeRstTolerance` writer - GPIO082 Read Privilege Reset Tolerance"]
pub type Gpio082readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO083ReadPrivilegeRstTolerance` reader - GPIO083 Read Privilege Reset Tolerance"]
pub type Gpio083readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO083ReadPrivilegeRstTolerance` writer - GPIO083 Read Privilege Reset Tolerance"]
pub type Gpio083readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO084ReadPrivilegeRstTolerance` reader - GPIO084 Read Privilege Reset Tolerance"]
pub type Gpio084readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO084ReadPrivilegeRstTolerance` writer - GPIO084 Read Privilege Reset Tolerance"]
pub type Gpio084readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO085ReadPrivilegeRstTolerance` reader - GPIO085 Read Privilege Reset Tolerance"]
pub type Gpio085readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO085ReadPrivilegeRstTolerance` writer - GPIO085 Read Privilege Reset Tolerance"]
pub type Gpio085readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO086ReadPrivilegeRstTolerance` reader - GPIO086 Read Privilege Reset Tolerance"]
pub type Gpio086readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO086ReadPrivilegeRstTolerance` writer - GPIO086 Read Privilege Reset Tolerance"]
pub type Gpio086readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO087ReadPrivilegeRstTolerance` reader - GPIO087 Read Privilege Reset Tolerance"]
pub type Gpio087readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO087ReadPrivilegeRstTolerance` writer - GPIO087 Read Privilege Reset Tolerance"]
pub type Gpio087readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO088ReadPrivilegeRstTolerance` reader - GPIO088 Read Privilege Reset Tolerance"]
pub type Gpio088readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO088ReadPrivilegeRstTolerance` writer - GPIO088 Read Privilege Reset Tolerance"]
pub type Gpio088readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO089ReadPrivilegeRstTolerance` reader - GPIO089 Read Privilege Reset Tolerance"]
pub type Gpio089readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO089ReadPrivilegeRstTolerance` writer - GPIO089 Read Privilege Reset Tolerance"]
pub type Gpio089readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO090ReadPrivilegeRstTolerance` reader - GPIO090 Read Privilege Reset Tolerance"]
pub type Gpio090readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO090ReadPrivilegeRstTolerance` writer - GPIO090 Read Privilege Reset Tolerance"]
pub type Gpio090readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO091ReadPrivilegeRstTolerance` reader - GPIO091 Read Privilege Reset Tolerance"]
pub type Gpio091readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO091ReadPrivilegeRstTolerance` writer - GPIO091 Read Privilege Reset Tolerance"]
pub type Gpio091readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO092ReadPrivilegeRstTolerance` reader - GPIO092 Read Privilege Reset Tolerance"]
pub type Gpio092readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO092ReadPrivilegeRstTolerance` writer - GPIO092 Read Privilege Reset Tolerance"]
pub type Gpio092readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO093ReadPrivilegeRstTolerance` reader - GPIO093 Read Privilege Reset Tolerance"]
pub type Gpio093readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO093ReadPrivilegeRstTolerance` writer - GPIO093 Read Privilege Reset Tolerance"]
pub type Gpio093readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO094ReadPrivilegeRstTolerance` reader - GPIO094 Read Privilege Reset Tolerance"]
pub type Gpio094readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO094ReadPrivilegeRstTolerance` writer - GPIO094 Read Privilege Reset Tolerance"]
pub type Gpio094readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO095ReadPrivilegeRstTolerance` reader - GPIO095 Read Privilege Reset Tolerance"]
pub type Gpio095readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO095ReadPrivilegeRstTolerance` writer - GPIO095 Read Privilege Reset Tolerance"]
pub type Gpio095readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO064 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio064read_privilege_rst_tolerance(&self) -> Gpio064readPrivilegeRstToleranceR {
        Gpio064readPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO065 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio065read_privilege_rst_tolerance(&self) -> Gpio065readPrivilegeRstToleranceR {
        Gpio065readPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO066 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio066read_privilege_rst_tolerance(&self) -> Gpio066readPrivilegeRstToleranceR {
        Gpio066readPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO067 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio067read_privilege_rst_tolerance(&self) -> Gpio067readPrivilegeRstToleranceR {
        Gpio067readPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO068 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio068read_privilege_rst_tolerance(&self) -> Gpio068readPrivilegeRstToleranceR {
        Gpio068readPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO069 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio069read_privilege_rst_tolerance(&self) -> Gpio069readPrivilegeRstToleranceR {
        Gpio069readPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO070 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio070read_privilege_rst_tolerance(&self) -> Gpio070readPrivilegeRstToleranceR {
        Gpio070readPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO071 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio071read_privilege_rst_tolerance(&self) -> Gpio071readPrivilegeRstToleranceR {
        Gpio071readPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO072 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio072read_privilege_rst_tolerance(&self) -> Gpio072readPrivilegeRstToleranceR {
        Gpio072readPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO073 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio073read_privilege_rst_tolerance(&self) -> Gpio073readPrivilegeRstToleranceR {
        Gpio073readPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO074 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio074read_privilege_rst_tolerance(&self) -> Gpio074readPrivilegeRstToleranceR {
        Gpio074readPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO075 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio075read_privilege_rst_tolerance(&self) -> Gpio075readPrivilegeRstToleranceR {
        Gpio075readPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO076 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio076read_privilege_rst_tolerance(&self) -> Gpio076readPrivilegeRstToleranceR {
        Gpio076readPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO077 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio077read_privilege_rst_tolerance(&self) -> Gpio077readPrivilegeRstToleranceR {
        Gpio077readPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO078 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio078read_privilege_rst_tolerance(&self) -> Gpio078readPrivilegeRstToleranceR {
        Gpio078readPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO079 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio079read_privilege_rst_tolerance(&self) -> Gpio079readPrivilegeRstToleranceR {
        Gpio079readPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO080 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio080read_privilege_rst_tolerance(&self) -> Gpio080readPrivilegeRstToleranceR {
        Gpio080readPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO081 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio081read_privilege_rst_tolerance(&self) -> Gpio081readPrivilegeRstToleranceR {
        Gpio081readPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO082 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio082read_privilege_rst_tolerance(&self) -> Gpio082readPrivilegeRstToleranceR {
        Gpio082readPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO083 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio083read_privilege_rst_tolerance(&self) -> Gpio083readPrivilegeRstToleranceR {
        Gpio083readPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO084 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio084read_privilege_rst_tolerance(&self) -> Gpio084readPrivilegeRstToleranceR {
        Gpio084readPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO085 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio085read_privilege_rst_tolerance(&self) -> Gpio085readPrivilegeRstToleranceR {
        Gpio085readPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO086 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio086read_privilege_rst_tolerance(&self) -> Gpio086readPrivilegeRstToleranceR {
        Gpio086readPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO087 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio087read_privilege_rst_tolerance(&self) -> Gpio087readPrivilegeRstToleranceR {
        Gpio087readPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO088 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio088read_privilege_rst_tolerance(&self) -> Gpio088readPrivilegeRstToleranceR {
        Gpio088readPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO089 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio089read_privilege_rst_tolerance(&self) -> Gpio089readPrivilegeRstToleranceR {
        Gpio089readPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO090 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio090read_privilege_rst_tolerance(&self) -> Gpio090readPrivilegeRstToleranceR {
        Gpio090readPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO091 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio091read_privilege_rst_tolerance(&self) -> Gpio091readPrivilegeRstToleranceR {
        Gpio091readPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO092 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio092read_privilege_rst_tolerance(&self) -> Gpio092readPrivilegeRstToleranceR {
        Gpio092readPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO093 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio093read_privilege_rst_tolerance(&self) -> Gpio093readPrivilegeRstToleranceR {
        Gpio093readPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO094 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio094read_privilege_rst_tolerance(&self) -> Gpio094readPrivilegeRstToleranceR {
        Gpio094readPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO095 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio095read_privilege_rst_tolerance(&self) -> Gpio095readPrivilegeRstToleranceR {
        Gpio095readPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO064 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio064read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio064readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio064readPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO065 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio065read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio065readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio065readPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO066 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio066read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio066readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio066readPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO067 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio067read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio067readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio067readPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO068 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio068read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio068readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio068readPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO069 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio069read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio069readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio069readPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO070 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio070read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio070readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio070readPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO071 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio071read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio071readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio071readPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO072 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio072read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio072readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio072readPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO073 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio073read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio073readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio073readPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO074 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio074read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio074readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio074readPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO075 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio075read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio075readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio075readPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO076 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio076read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio076readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio076readPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO077 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio077read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio077readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio077readPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO078 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio078read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio078readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio078readPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO079 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio079read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio079readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio079readPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO080 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio080read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio080readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio080readPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO081 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio081read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio081readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio081readPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO082 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio082read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio082readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio082readPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO083 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio083read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio083readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio083readPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO084 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio084read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio084readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio084readPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO085 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio085read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio085readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio085readPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO086 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio086read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio086readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio086readPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO087 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio087read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio087readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio087readPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO088 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio088read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio088readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio088readPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO089 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio089read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio089readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio089readPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO090 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio090read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio090readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio090readPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO091 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio091read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio091readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio091readPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO092 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio092read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio092readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio092readPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO093 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio093read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio093readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio093readPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO094 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio094read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio094readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio094readPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO095 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio095read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio095readPrivilegeRstToleranceW<Gpioc18Spec> {
        Gpio095readPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Read Privilege Reset Tolerance Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioc18Spec;
impl crate::RegisterSpec for Gpioc18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc18::R`](R) reader structure"]
impl crate::Readable for Gpioc18Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioc18::W`](W) writer structure"]
impl crate::Writable for Gpioc18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC18 to value 0"]
impl crate::Resettable for Gpioc18Spec {}
