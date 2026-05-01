#[doc = "Register `GPIOB18` reader"]
pub type R = crate::R<Gpiob18Spec>;
#[doc = "Register `GPIOB18` writer"]
pub type W = crate::W<Gpiob18Spec>;
#[doc = "GPIO064 Write Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio064wrPrivilegeRstTolerance {
    #[doc = "0: Write Privilege of GPIO064 is reset by WDT."]
    WritePrivilegeOfGpio064IsResetByWdt = 0,
    #[doc = "1: Write Privilege of GPIO064 is NOT reset by WDT."]
    WritePrivilegeOfGpio064IsNotResetByWdt = 1,
}
impl From<Gpio064wrPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio064wrPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO064WrPrivilegeRstTolerance` reader - GPIO064 Write Privilege Reset Tolerance"]
pub type Gpio064wrPrivilegeRstToleranceR = crate::BitReader<Gpio064wrPrivilegeRstTolerance>;
impl Gpio064wrPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio064wrPrivilegeRstTolerance {
        match self.bits {
            false => Gpio064wrPrivilegeRstTolerance::WritePrivilegeOfGpio064IsResetByWdt,
            true => Gpio064wrPrivilegeRstTolerance::WritePrivilegeOfGpio064IsNotResetByWdt,
        }
    }
    #[doc = "Write Privilege of GPIO064 is reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio064_is_reset_by_wdt(&self) -> bool {
        *self == Gpio064wrPrivilegeRstTolerance::WritePrivilegeOfGpio064IsResetByWdt
    }
    #[doc = "Write Privilege of GPIO064 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio064_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio064wrPrivilegeRstTolerance::WritePrivilegeOfGpio064IsNotResetByWdt
    }
}
#[doc = "Field `GPIO064WrPrivilegeRstTolerance` writer - GPIO064 Write Privilege Reset Tolerance"]
pub type Gpio064wrPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio064wrPrivilegeRstTolerance>;
impl<'a, REG> Gpio064wrPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write Privilege of GPIO064 is reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio064_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio064wrPrivilegeRstTolerance::WritePrivilegeOfGpio064IsResetByWdt)
    }
    #[doc = "Write Privilege of GPIO064 is NOT reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio064_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio064wrPrivilegeRstTolerance::WritePrivilegeOfGpio064IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO065WrPrivilegeRstTolerance` reader - GPIO065 Write Privilege Reset Tolerance"]
pub type Gpio065wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO065WrPrivilegeRstTolerance` writer - GPIO065 Write Privilege Reset Tolerance"]
pub type Gpio065wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO066WrPrivilegeRstTolerance` reader - GPIO066 Write Privilege Reset Tolerance"]
pub type Gpio066wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO066WrPrivilegeRstTolerance` writer - GPIO066 Write Privilege Reset Tolerance"]
pub type Gpio066wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO067WrPrivilegeRstTolerance` reader - GPIO067 Write Privilege Reset Tolerance"]
pub type Gpio067wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO067WrPrivilegeRstTolerance` writer - GPIO067 Write Privilege Reset Tolerance"]
pub type Gpio067wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO068WrPrivilegeRstTolerance` reader - GPIO068 Write Privilege Reset Tolerance"]
pub type Gpio068wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO068WrPrivilegeRstTolerance` writer - GPIO068 Write Privilege Reset Tolerance"]
pub type Gpio068wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO069WrPrivilegeRstTolerance` reader - GPIO069 Write Privilege Reset Tolerance"]
pub type Gpio069wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO069WrPrivilegeRstTolerance` writer - GPIO069 Write Privilege Reset Tolerance"]
pub type Gpio069wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO070WrPrivilegeRstTolerance` reader - GPIO070 Write Privilege Reset Tolerance"]
pub type Gpio070wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO070WrPrivilegeRstTolerance` writer - GPIO070 Write Privilege Reset Tolerance"]
pub type Gpio070wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO071WrPrivilegeRstTolerance` reader - GPIO071 Write Privilege Reset Tolerance"]
pub type Gpio071wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO071WrPrivilegeRstTolerance` writer - GPIO071 Write Privilege Reset Tolerance"]
pub type Gpio071wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO072WrPrivilegeRstTolerance` reader - GPIO072 Write Privilege Reset Tolerance"]
pub type Gpio072wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO072WrPrivilegeRstTolerance` writer - GPIO072 Write Privilege Reset Tolerance"]
pub type Gpio072wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO073WrPrivilegeRstTolerance` reader - GPIO073 Write Privilege Reset Tolerance"]
pub type Gpio073wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO073WrPrivilegeRstTolerance` writer - GPIO073 Write Privilege Reset Tolerance"]
pub type Gpio073wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO074WrPrivilegeRstTolerance` reader - GPIO074 Write Privilege Reset Tolerance"]
pub type Gpio074wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO074WrPrivilegeRstTolerance` writer - GPIO074 Write Privilege Reset Tolerance"]
pub type Gpio074wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO075WrPrivilegeRstTolerance` reader - GPIO075 Write Privilege Reset Tolerance"]
pub type Gpio075wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO075WrPrivilegeRstTolerance` writer - GPIO075 Write Privilege Reset Tolerance"]
pub type Gpio075wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO076WrPrivilegeRstTolerance` reader - GPIO076 Write Privilege Reset Tolerance"]
pub type Gpio076wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO076WrPrivilegeRstTolerance` writer - GPIO076 Write Privilege Reset Tolerance"]
pub type Gpio076wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO077WrPrivilegeRstTolerance` reader - GPIO077 Write Privilege Reset Tolerance"]
pub type Gpio077wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO077WrPrivilegeRstTolerance` writer - GPIO077 Write Privilege Reset Tolerance"]
pub type Gpio077wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO078WrPrivilegeRstTolerance` reader - GPIO078 Write Privilege Reset Tolerance"]
pub type Gpio078wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO078WrPrivilegeRstTolerance` writer - GPIO078 Write Privilege Reset Tolerance"]
pub type Gpio078wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO079WrPrivilegeRstTolerance` reader - GPIO079 Write Privilege Reset Tolerance"]
pub type Gpio079wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO079WrPrivilegeRstTolerance` writer - GPIO079 Write Privilege Reset Tolerance"]
pub type Gpio079wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO080WrPrivilegeRstTolerance` reader - GPIO080 Write Privilege Reset Tolerance"]
pub type Gpio080wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO080WrPrivilegeRstTolerance` writer - GPIO080 Write Privilege Reset Tolerance"]
pub type Gpio080wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO081WrPrivilegeRstTolerance` reader - GPIO081 Write Privilege Reset Tolerance"]
pub type Gpio081wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO081WrPrivilegeRstTolerance` writer - GPIO081 Write Privilege Reset Tolerance"]
pub type Gpio081wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO082WrPrivilegeRstTolerance` reader - GPIO082 Write Privilege Reset Tolerance"]
pub type Gpio082wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO082WrPrivilegeRstTolerance` writer - GPIO082 Write Privilege Reset Tolerance"]
pub type Gpio082wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO083WrPrivilegeRstTolerance` reader - GPIO083 Write Privilege Reset Tolerance"]
pub type Gpio083wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO083WrPrivilegeRstTolerance` writer - GPIO083 Write Privilege Reset Tolerance"]
pub type Gpio083wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO084WrPrivilegeRstTolerance` reader - GPIO084 Write Privilege Reset Tolerance"]
pub type Gpio084wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO084WrPrivilegeRstTolerance` writer - GPIO084 Write Privilege Reset Tolerance"]
pub type Gpio084wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO085WrPrivilegeRstTolerance` reader - GPIO085 Write Privilege Reset Tolerance"]
pub type Gpio085wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO085WrPrivilegeRstTolerance` writer - GPIO085 Write Privilege Reset Tolerance"]
pub type Gpio085wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO086WrPrivilegeRstTolerance` reader - GPIO086 Write Privilege Reset Tolerance"]
pub type Gpio086wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO086WrPrivilegeRstTolerance` writer - GPIO086 Write Privilege Reset Tolerance"]
pub type Gpio086wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO087WrPrivilegeRstTolerance` reader - GPIO087 Write Privilege Reset Tolerance"]
pub type Gpio087wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO087WrPrivilegeRstTolerance` writer - GPIO087 Write Privilege Reset Tolerance"]
pub type Gpio087wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO088WrPrivilegeRstTolerance` reader - GPIO088 Write Privilege Reset Tolerance"]
pub type Gpio088wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO088WrPrivilegeRstTolerance` writer - GPIO088 Write Privilege Reset Tolerance"]
pub type Gpio088wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO089WrPrivilegeRstTolerance` reader - GPIO089 Write Privilege Reset Tolerance"]
pub type Gpio089wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO089WrPrivilegeRstTolerance` writer - GPIO089 Write Privilege Reset Tolerance"]
pub type Gpio089wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO090WrPrivilegeRstTolerance` reader - GPIO090 Write Privilege Reset Tolerance"]
pub type Gpio090wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO090WrPrivilegeRstTolerance` writer - GPIO090 Write Privilege Reset Tolerance"]
pub type Gpio090wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO091WrPrivilegeRstTolerance` reader - GPIO091 Write Privilege Reset Tolerance"]
pub type Gpio091wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO091WrPrivilegeRstTolerance` writer - GPIO091 Write Privilege Reset Tolerance"]
pub type Gpio091wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO092WrPrivilegeRstTolerance` reader - GPIO092 Write Privilege Reset Tolerance"]
pub type Gpio092wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO092WrPrivilegeRstTolerance` writer - GPIO092 Write Privilege Reset Tolerance"]
pub type Gpio092wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO093WrPrivilegeRstTolerance` reader - GPIO093 Write Privilege Reset Tolerance"]
pub type Gpio093wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO093WrPrivilegeRstTolerance` writer - GPIO093 Write Privilege Reset Tolerance"]
pub type Gpio093wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO094WrPrivilegeRstTolerance` reader - GPIO094 Write Privilege Reset Tolerance"]
pub type Gpio094wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO094WrPrivilegeRstTolerance` writer - GPIO094 Write Privilege Reset Tolerance"]
pub type Gpio094wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO095WrPrivilegeRstTolerance` reader - GPIO095 Write Privilege Reset Tolerance"]
pub type Gpio095wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO095WrPrivilegeRstTolerance` writer - GPIO095 Write Privilege Reset Tolerance"]
pub type Gpio095wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO064 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio064wr_privilege_rst_tolerance(&self) -> Gpio064wrPrivilegeRstToleranceR {
        Gpio064wrPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO065 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio065wr_privilege_rst_tolerance(&self) -> Gpio065wrPrivilegeRstToleranceR {
        Gpio065wrPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO066 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio066wr_privilege_rst_tolerance(&self) -> Gpio066wrPrivilegeRstToleranceR {
        Gpio066wrPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO067 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio067wr_privilege_rst_tolerance(&self) -> Gpio067wrPrivilegeRstToleranceR {
        Gpio067wrPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO068 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio068wr_privilege_rst_tolerance(&self) -> Gpio068wrPrivilegeRstToleranceR {
        Gpio068wrPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO069 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio069wr_privilege_rst_tolerance(&self) -> Gpio069wrPrivilegeRstToleranceR {
        Gpio069wrPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO070 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio070wr_privilege_rst_tolerance(&self) -> Gpio070wrPrivilegeRstToleranceR {
        Gpio070wrPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO071 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio071wr_privilege_rst_tolerance(&self) -> Gpio071wrPrivilegeRstToleranceR {
        Gpio071wrPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO072 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio072wr_privilege_rst_tolerance(&self) -> Gpio072wrPrivilegeRstToleranceR {
        Gpio072wrPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO073 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio073wr_privilege_rst_tolerance(&self) -> Gpio073wrPrivilegeRstToleranceR {
        Gpio073wrPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO074 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio074wr_privilege_rst_tolerance(&self) -> Gpio074wrPrivilegeRstToleranceR {
        Gpio074wrPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO075 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio075wr_privilege_rst_tolerance(&self) -> Gpio075wrPrivilegeRstToleranceR {
        Gpio075wrPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO076 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio076wr_privilege_rst_tolerance(&self) -> Gpio076wrPrivilegeRstToleranceR {
        Gpio076wrPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO077 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio077wr_privilege_rst_tolerance(&self) -> Gpio077wrPrivilegeRstToleranceR {
        Gpio077wrPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO078 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio078wr_privilege_rst_tolerance(&self) -> Gpio078wrPrivilegeRstToleranceR {
        Gpio078wrPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO079 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio079wr_privilege_rst_tolerance(&self) -> Gpio079wrPrivilegeRstToleranceR {
        Gpio079wrPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO080 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio080wr_privilege_rst_tolerance(&self) -> Gpio080wrPrivilegeRstToleranceR {
        Gpio080wrPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO081 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio081wr_privilege_rst_tolerance(&self) -> Gpio081wrPrivilegeRstToleranceR {
        Gpio081wrPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO082 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio082wr_privilege_rst_tolerance(&self) -> Gpio082wrPrivilegeRstToleranceR {
        Gpio082wrPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO083 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio083wr_privilege_rst_tolerance(&self) -> Gpio083wrPrivilegeRstToleranceR {
        Gpio083wrPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO084 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio084wr_privilege_rst_tolerance(&self) -> Gpio084wrPrivilegeRstToleranceR {
        Gpio084wrPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO085 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio085wr_privilege_rst_tolerance(&self) -> Gpio085wrPrivilegeRstToleranceR {
        Gpio085wrPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO086 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio086wr_privilege_rst_tolerance(&self) -> Gpio086wrPrivilegeRstToleranceR {
        Gpio086wrPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO087 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio087wr_privilege_rst_tolerance(&self) -> Gpio087wrPrivilegeRstToleranceR {
        Gpio087wrPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO088 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio088wr_privilege_rst_tolerance(&self) -> Gpio088wrPrivilegeRstToleranceR {
        Gpio088wrPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO089 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio089wr_privilege_rst_tolerance(&self) -> Gpio089wrPrivilegeRstToleranceR {
        Gpio089wrPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO090 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio090wr_privilege_rst_tolerance(&self) -> Gpio090wrPrivilegeRstToleranceR {
        Gpio090wrPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO091 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio091wr_privilege_rst_tolerance(&self) -> Gpio091wrPrivilegeRstToleranceR {
        Gpio091wrPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO092 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio092wr_privilege_rst_tolerance(&self) -> Gpio092wrPrivilegeRstToleranceR {
        Gpio092wrPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO093 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio093wr_privilege_rst_tolerance(&self) -> Gpio093wrPrivilegeRstToleranceR {
        Gpio093wrPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO094 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio094wr_privilege_rst_tolerance(&self) -> Gpio094wrPrivilegeRstToleranceR {
        Gpio094wrPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO095 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio095wr_privilege_rst_tolerance(&self) -> Gpio095wrPrivilegeRstToleranceR {
        Gpio095wrPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO064 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio064wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio064wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio064wrPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO065 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio065wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio065wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio065wrPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO066 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio066wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio066wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio066wrPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO067 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio067wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio067wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio067wrPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO068 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio068wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio068wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio068wrPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO069 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio069wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio069wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio069wrPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO070 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio070wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio070wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio070wrPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO071 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio071wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio071wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio071wrPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO072 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio072wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio072wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio072wrPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO073 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio073wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio073wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio073wrPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO074 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio074wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio074wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio074wrPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO075 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio075wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio075wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio075wrPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO076 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio076wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio076wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio076wrPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO077 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio077wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio077wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio077wrPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO078 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio078wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio078wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio078wrPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO079 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio079wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio079wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio079wrPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO080 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio080wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio080wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio080wrPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO081 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio081wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio081wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio081wrPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO082 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio082wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio082wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio082wrPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO083 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio083wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio083wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio083wrPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO084 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio084wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio084wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio084wrPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO085 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio085wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio085wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio085wrPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO086 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio086wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio086wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio086wrPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO087 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio087wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio087wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio087wrPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO088 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio088wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio088wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio088wrPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO089 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio089wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio089wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio089wrPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO090 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio090wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio090wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio090wrPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO091 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio091wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio091wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio091wrPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO092 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio092wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio092wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio092wrPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO093 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio093wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio093wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio093wrPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO094 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio094wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio094wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio094wrPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO095 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio095wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio095wrPrivilegeRstToleranceW<Gpiob18Spec> {
        Gpio095wrPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Write Privilege Reset Tolerance Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpiob18Spec;
impl crate::RegisterSpec for Gpiob18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob18::R`](R) reader structure"]
impl crate::Readable for Gpiob18Spec {}
#[doc = "`write(|w| ..)` method takes [`gpiob18::W`](W) writer structure"]
impl crate::Writable for Gpiob18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOB18 to value 0"]
impl crate::Resettable for Gpiob18Spec {}
